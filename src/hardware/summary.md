# Summary information about the GameBoy Hardware (Organization & Architecture)

> Information summary from the "GAME BOY (TM) CPU Manual"

## General information

- CPU: 8-bit
- Main RAM: 8K Byte
- Video RAM: 8K Byte
- Resolution: 160x144 (20x18 tiles)
- Max # of sprites: 40
- Max # sprites/line: 10
- Max sprite size: 8x16
- Min sprite size: 8x8
- Clock Speed: 4.194304 MHz (4.194/8.388MHz GBC)
- Horizontal Sync: 9198 KHz
- Vertical Sync: 59.73 Hz
- Sound: 4 channels with stereo sound

- 1 machine cycle = 4 clock cycles

|                 | GB CPU Speed  | NOP Instruction  |
| --------------- |:-------------:|:----------------:|
| Machine Cycles  | 1.5 MHz       | 1 cycles         |
| Clock Cycles    | 4.19 MHz      | 4 cycles         |


## Memory Architecture

## Memory Mapping Configuration

```
Interrupt Enable Register
--------------------------------- FFFF
Internal RAM
--------------------------------- FF80
Empty but unusable for I/O
--------------------------------- FF4C
I/O ports
--------------------------------- FF00
Empty but unusable for I/O
--------------------------------- FEA0
Sprite Attrib Memory (0AM)
--------------------------------- FE00
Echo of 8kB Internal RAM
--------------------------------- E000
8kB Internal RAM
--------------------------------- C000
8kB Switchable RAM Bank
--------------------------------- A000
8kB Video RAM
--------------------------------- 8000  --
16kB Switchable ROM Bank                 |
--------------------------------- 4000   | = 32kB Cartridge
16kB ROM Bank #0                         |
--------------------------------- 0000  --
```

### Reserved Memory Locations

```
0000          Reset 00 Address
              (RST 00 calls this address)
0008          Reset 08 Address
              (RST 08 calls this address)
0010          Reset 10 Address
              (RST 10 calls this address)
0018          Reset 18 Address
              (RST 18 calls this address)
0020          Reset 20 Address
              (RST 20 calls this address)
0028          Reset 28 Address
              (RST 28 calls this address)
0030          Reset 30 Address
              (RST 30 calls this address)
0038          Reset 38 Address
              (RST 38 calls this address)
0040          Vertical Blank Interrupt Start Address
0048          LCDC Status Interrupt Start Address
0050          Timer Overflow Interrupt Start Address
0058          Serial Transfer Completion Interrupt Start Address
0060          High-to-Low of P10-P13 Interrupt Start Address
```

Internal information at cartridges for area located at `[0100]-[014F]`:

```
0100-0103     Initial execution point
              (Usually NOP and JP)
0104-0133     Scrolling Nintendo Graphic:
              CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
              00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
              BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E

0134-0142     Title of the game in (Uppercase ASCII), up to 16 characters
              (filled with 00 for the remaining characters)

0143          80 = Chromatic GameBoy, 00 or other = Achromatic GameBoy

0144          ASCII Hexadecimal digit, high nibble of licensee code (new)

0145          ASCII Hexadecimal digit, low nibble of licensee code (new)
              (Usually 00 if [014B] != 33)

0146          GB/SGB Indicator
              (00 = GameBoy, 03 = Super GameBoy functions)

0147          Cartridge type:
              (00 = GameBoy, 03 = Super GameBoy functions)
              00 ROM ONLY
              01 ROM + MBC1
              02 ROM + MBC1 + RAM
              03 ROM + MBC1 + RAMM + BATTERY
              05 ROM + MBC2
              08 ROM + RAM
              09 ROM + RAM + BATTERY
              0B ROM + MMM01
              0C ROM + MMM01 + SRAM
              0D ROM + MMM01 + SRAM + BATTERY
              0F ROM + MBC3 + TIMER + BATTERY
              10 ROM + MBC3 + TIMER + RAM + BATTERY
              11 ROM + MBC3
              12 ROM + MBC3 + RAM
              13 ROM + MBC3 + RAM + BATTERY
              19 ROM + MBC5
              1A ROM + MBC5 + RAM
              1B ROM + MBC5 + RAM + BATTERY
              1C ROM + MBC5 + RUMBLE
              1D ROM + MBC5 + RUMBLE + SRAM
              1E ROM + MBC5 + RUMBLE + SRAM + BATTERY
              1F Pocket Camera
              FD Bandai TAMA5
              FE Hudson HuC-3
              FF Hudson HuC-1

0148          ROM size:
              00  256 Kb  = 32 KB  = 2 banks
              01  512 Kb  = 64 KB  = 4 banks
              02  1 Mb    = 128 KB = 8 banks
              03  2 Mb    = 256 KB = 16 banks
              04  4 Mb    = 512 KB = 32 banks
              05  8 Mb    = 1 MB   = 64 banks
              06  16 Mb   = 2 MB   = 128 banks
              52  9 Mb    = 1.1 MB = 72 banks
              53  10 Mb   = 1.2 MB = 80 banks
              54  12 Mb   = 1.5 MB = 96 banks

0149          RAM size:
              00 None
              01 16 Kb    = 2 KB   = 2 banks
              02 64 Kb    = 8 KB   = 2 banks
              03 256 Kb   = 32 KB  = 4 banks
              04 1 Mb     = 128 KB = 16 banks

014A          Destination code: 00 JPN | 01 Otherwise

014B          Licensee code (old)
              33 Check [0144]/[0145] for Licensee code
              79 Accolade
              A4 Konami
              (Super GameBoy functions not available if != 33)

014C          Mask ROM Version number (usually 00)

014D          Complement check
              (Program should not run if incorrect)

014E-014F     Checksum (higher byte first)
              Calculated by adding all bytes of a cartridge, except for two checksum bytes,
              and taking two lower bytes of the result
```

### Cartridge Types

According to the byte at the cartridge location `[0147]`

- ROM Only
  A 32 KB (256 Kb) ROM and occupies `[0000]-[7FFF]`

- MBC1 (Memory Bank Controller 1)
  Two different maximum memory modes, according to the cartridge setup

- MBC2 (Memory Bank Controller 2)
  Additonal maximum memory modes, according to the cartridge setup

- MBC3 (Memory Bank Controller 3)
  Access up to 16 Mb of ROM
  Similar to MBC1, according to the cartridge setup

- MBC5 (Memory Bank Controller 5)
  Similar to MBC3, according to the cartridge setup
  Access up to 64 Mb of ROM and up to 1 Mb of RAM
  Guaranteed to run in GameBoy Color double-speed

- Rumble Cartridges
  Uses a MBC5 Memory Bank Controller

- HuC-1 (Memory Bank / Infrared Controller)


### Special Mode

Power Up Sequence:

1. 256 byte program, starting at memory location `[0000]`, is executed
2. Read the cartridge locations from `[0104]` to `[0133]`
3. Place this graphic of a Nintendo logo on the screen, at the top
4. The image is scrolled until it is in the middle of the screen
5. Two musical notes are played
6. The cartridge locations from `[0104]` to `[0133]` are re-read,
   compared to a table in the internnal ROM
7. If any bytes failes to compare, then the GameBoy stops comparing bytes
   and simply halts all operations

For the GameBoy, it follows:

7.  Adds all bytes from `[0134]` to `[014D]`
9.  A value of 25 decimal (`0x19`) is added to the total
10. If the total is not zero, the GameBoy will stop all operations and halt

Then, it follows:

11. If all the checks pass, then the internal ROM is disabled and cartridge
    program execution begins at location `[0100]`
12. The initial state for the registers are:
    AF = 01 (GameBoy), AF=11 (GameBoy Color)
    F  = B0
    BC = 0013
    DE = 00D8
    HL = 014D
    Stack Pointer = FFFE
    `[FF05]` = 00; TIMA (Timer Counter)
    `[FF06]` = 00; TMA  (Timer Modulo)
    `[FF07]` = 00; TAC  (Timer Control)
    `[FF10]` = 80; NR10
    `[FF11]` = BF; NR11
    `[FF12]` = F3; NR12
    `[FF14]` = BF; NR14
    `[FF16]` = 3F; NR21
    `[FF17]` = 00; NR22
    `[FF19]` = BF; NR24
    `[FF1A]` = 7F; NR30
    `[FF1B]` = FF; NR31
    `[FF1C]` = 9F; NR32
    `[FF1D]` = BF; NR33 * Error
    `[FF20]` = FF; NR41
    `[FF21]` = 00; NR42
    `[FF22]` = 00; NR43
    `[FF23]` = BF; NR44 * Error
    `[FF24]` = 77; NR50
    `[FF25]` = F3; NR51
    `[FF26]` = F1; (GameBoy); NR52
    `[FF40]` = 91; LCDC
    `[FF42]` = 00; SCY
    `[FF43]` = 00; SCX
    `[FF45]` = 00; LYC
    `[FF47]` = FC; BGP
    `[FF48]` = FF; OBP0
    `[FF49]` = FF; OBP1
    `[FF4A]` = 00; WY
    `[FF4B]` = 00; WX
    `[FFFF]` = 00; IE

"All GameBoy emulators tend to set all RAM values to 00 on entry.'


### Stop Mode

The STOP command halts the GameBoy processor and screen until any button is pressed.
The GameBoy screen goes whie with a single dark horizontal line.
The GameBoy Color screen goes black.


### Low Power Mode

The HALT instruction is used whenever possible to reduce power consumption.
This commands stops the system clock reducing the power consumption of both the CPU and ROM.
The CPU will remain suspended until an interrupt occurs at which point the interrupt is
serviced and then the instruction immediately following the HALT is executed.
If interrupts are disabled (DI), then halt doesn't susped operation but it does cause
the program counter to stop counting for one instruction on the GameBoy.

The instruction immediately following the HALT instruction is skipped when interrupts are
disabled (DI) on the GameBoy. As a result, always put a NOP after the HALT instruction.
This instruction skipping doesn't occur when interrupts are enabled (EI). This skipping
does not seem to occur on the GameBoy Color even in regular GameBoy mode (`[0143]` = 00).


## Video: VRAM area and Displaying Visual Data

NOTA: Essa parte é bem confusa. Pelo que entendi, o autor está explicando sobre a área de Buffer,
controlada por SCROLLX e SCROLLY, e a área apresentada, controlada por WNDPOSX e WNDPPOSY.
Analisar outros textos para melhor compreender como funciona a parte de Video.

The main GameBoy screen buffer (background) consists of 256 x 256 pixels or 32 x 32 tiles
(8 x 8 pixels each). Only 160 x 144 pixels can be displayed on the screen.

### Background Tile Map, Tile Data Table and the Background Window

- Registers SCROLLX and SCROLLY hold the coordinates of background to be displayed in the left
  upper corner of the screen. Background wraps around the screen: when part of it goes off the screen,
  it appears on the opposite side.
- The Background Tile Map contains the number of tiles to be displayed. It is organized as 32 rows
  of 32 bytes each
- Each byte contains a number of a Tile to be displayed
- Tile patterns are taken from the Tile Data Table located either at `[8000]-[8FFF]` or
  `[8800]-[97FF]`. In the first case, patterns are numbered with unsigned numbers from
  0 to 255 (pattern `#0` lies at `[8000]` and so forth). In the second case, patterns have
  signed numbers from -128 to 127 (pattern `#0` lies att address `[9000]`). The Tile Data
  Table address for the background can be selected via de LCDC register.

#### Window

- Window overlaying the Background
- This window is not scrollable: always displayed from its upper left corner
- The location of a window on the screen can be adjusted via WNDPOSX and WNDPOSY registers
- Screen coordinates of the top left corner of a window are WNDPOSX, WNDPPOSY
- The tile numbers for the window are stored in the Tile Data Table
- None of the windows tiles are ever transparent
- Both the Background and the Window share the same Tile Data Table
- Both the Background and the Window can be disabled or enabled separately via bits in the LCDC
  register

If the Window is used and a scan line interrupt disables it (either by writing to LCDC or by
setting WNDPOSX > 166), and a scan line interrupt a little later on enables it, then the Window
will resume appearing on the screen at the exact position of the Window where it left off earlier.
That way, even if there are only 16 lines of useful graphics in the window, you could display
the first 8 lines at the top of the screen and the next 8 lines at the bottom, if you wanted to do so.

WNDPOSX may be changed during a scan line interrupt (to either cause a graphic distortion effect
or to disable the window: WNDPOSX > 166), but changes to WNDPPOSY are not dynamic and won't be
noticed until the next screen redraw.

#### The Tile Pattern Tables

The tile images are stored in the Tile Pattern Tables. Each 8 x 8 pixels image occupies 16 bytes;
each 2 bytes represent a line.

NOTA: São 8 x 8 pixels, 16 bytes = 128 bits, 2 bits por pixel. Cada 2 bytes representa uma linha.

### Sprites

GameBoy video controller can display up to 400 sprites either in 8 x 8 pixels or in 8 x 16 pixels.
Because of hardware limitation, only 10 spirtes can be displayed per scan line. Sprite patterns
hve the same format as tiles, but they are taken from the Sprite Pattern Table located at `[8000]-[8FFF]`
nd they have unsigned integer numbering. Sprite attributes reside in the Sprite Attribute Table (OAM -
Object Attribute Memory) at `[FE00]-[FE9F]`.

OAM is divided into 40 4-bytes blocks each of which corresponds to a sprite. Each Block have the
following format:

```
Byte0      Y position on the screen
Byte1      X position on the screen
Byte2      Pattern number 0-255 (Unlike some tile numbers, sprite patter numbers are unsigned.
           LSB is ignored (treated as 0) in 8 x 16 pixels mode)
Byte3      Flags:
              Bit7     Priority
                       If this bit is set to 0, sprite is displayed on top of background & window.
                       If this bit is set to 1, then sprite will be hidden behind colors 1, 2, and 3
                       of the background & window. (Sprite only prevails over color 0 of BG & Window)
              Bit6     Y flip
                       Sprite pattern is flipped vertically if this bit is set to 1.
              Bit5     X flip
                       Sprite pattern is flipped horizontally if this bit is set to 1.
              Bit4     Palette number
                       Sprite colors are taken from OBJ1PAL if this bit is set to 1, and
                       from OBJ0PAL otherwise
```

- In the 8 x 16 pixels sprite mode, the least significant bit of the sprite pattern number is ignored
  and treated as 0.
- When sprites with differnt X-coordinate values overlap, the one with the smaller
  X-coordinate (closer to the left) will have priority and appear above any others.
- When sprites with the same X-coordinate values overlap, they have priority according to table
  ordering: `[FE00]`, highest; `[FE04]`, next highest; etc.
- Sprite X=0, Y=0 hides a sprite. To display a sprite, use the following formulas:
  (1) `SpriteScreenPositionX = SpriteX - 8`;
  (2) `SpriteScreenPositionY = SpriteY - 16`.
- To display a sprite in the upper left corner of the screen, set sprite X=8, Y=16.
- Only 10 sprites can be displayed on any one line. When this limit is exceeded, the lower priority
sprites (priorities listed above) won't be displayed. To keep unused sprites from ffecting onscreen
sprites, set their Y coordinate to Y=0 or X >= 144 + 16. Just setting the X coordinate to X=0
or X >= 160 + 8 on a sprite will hide it, but it will affect other sprites sharing the same lines.
