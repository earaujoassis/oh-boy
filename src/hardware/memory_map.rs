/// Special Memory Addresses & Memory Mapping Areas

pub const IROM: u16 = 0x0000; // Internal / BOOT ROM (if enabled) Start
pub const IROX: u16 = 0x00FF; // Internal / BOOT ROM (if enabled) End
pub const IROZ: u16 = 0x0100; // External ROM Space
pub const ROM0: u16 = 0x0000; // (Codebase Acronym) ROM Bank #0 Start
//             FROM   0x0001
//               TO   0x003F
pub const VBI0: u16 = 0x0040; // Vertical Blank Interrupt Start Address
//             FROM   0x0041
//               TO   0x0047
pub const LCS0: u16 = 0x0048; // LCD Control Status Interrupt Start Address
//             FROM   0x0049
//               TO   0x004F
pub const TOI0: u16 = 0x0050; // Timer Overflow Interrupt Start Address
//             FROM   0x0051
//               TO   0x0057
pub const STCI: u16 = 0x0058; // Serial Transfer Completion Interrupt Start Address
//             FROM   0x0059
//               TO   0x005F
pub const HTLP: u16 = 0x0060; // High-to-Low of P10-P13 Interrupt Start Address
//             FROM   0x0061
//               TO   0x00FF
pub const BGN:  u16 = 0x0100; // Begin Instructions Point
//             FROM   0x0101
//               TO   0x0103
pub const NTD0: u16 = 0x0104; // Scrolling Nintendo Graphic Start
//             FROM   0x0105
//               TO   0x0132
pub const NTD9: u16 = 0x0133; // Scrolling Nintendo Graphic End
pub const TN0:  u16 = 0x0134; // Title of the Game (Uppercase ASCII) Start
//             FROM   0x0135
//               TO   0x0141
pub const TN9:  u16 = 0x0142; // Title of the Game (Uppercase ASCII) End
pub const COLO: u16 = 0x0143; // Colorful or Colorless GameBoy Configuration
pub const HLIC: u16 = 0x0144; // ASCII High nibble of Licensee
pub const LLIC: u16 = 0x0145; // ASCII Low nibble of Licensee
pub const GBV:  u16 = 0x0146; // GAME BOY Version Indicator
pub const RTC:  u16 = 0x0147; // ROM Cartridge Type
pub const OSIZ: u16 = 0x0148; // ROM Size
pub const ASIZ: u16 = 0x0149; // RAM Size
pub const DCOD: u16 = 0x014A; // Destination Code (Japan or Worldwide)
pub const LCOD: u16 = 0x014B; // Licensee Code (Old Format)
pub const MROM: u16 = 0x014C; // Mask ROM Version Number
pub const CHEK: u16 = 0x014D; // Complement Check
pub const CSU0: u16 = 0x014E; // Checksum Start
pub const CSU9: u16 = 0x014F; // Checksum End
//             FROM   0x0150
//               TO   0x3FFE
pub const ROM9: u16 = 0x3FFF; // (Codebase Acronym) Non-switchable ROM Bank #0 End
pub const RB0:  u16 = 0x4000; // (Codebase Acronym) Switchable / External ROM Bank Start
//             FROM   0x4001
//               TO   0x7FFE
pub const RB9:  u16 = 0x7FFF; // (Codebase Acronym) Switchable / External ROM Bank End
pub const VR0:  u16 = 0x8000; // (Codebase Acronym) VRAM Start
//             FROM   0x8001
//               TO   0x9FFE
pub const VR9:  u16 = 0x9FFF; // (Codebase Acronym) VRAM End
pub const SWR0: u16 = 0xA000; // (Codebase Acronym) Switchable / External RAM Bank Start
//             FROM   0xA001
//               TO   0xBFFE
pub const SWR9: u16 = 0xBFFF; // (Codebase Acronym) Switchable / External RAM Bank End
pub const WR0:  u16 = 0xC000; // (Codebase Acronym) Internal (Work) RAM Start
//             FROM   0xC001
//               TO   0xDFFE
pub const WR9:  u16 = 0xDFFF; // (Codebase Acronym) Internal (Work) RAM End
pub const ER0:  u16 = 0xE000; // (Codebase Acronym) ECHO of RAM (0xC000) Start
//             FROM   0xE001
//               TO   0xFDFE
pub const ER9:  u16 = 0xFDFF; // (Codebase Acronym) ECHO of RAM (0xDFFF) End
pub const OAM0: u16 = 0xFE00; // (Codebase Acronym) Object Attribute Memory (OAM) Start
//             FROM   0xFE01
//               TO   0xFE9E
pub const OAM9: u16 = 0xFE9F; // (Codebase Acronym) Object Attribute Memory (OAM) End
pub const RAM0: u16 = 0xFEA0; // (Codebase Acronym) High RAM Area Start
//             FROM   0xFEA1
//               TO   0xFEFF
pub const URAM: u16 = 0xFEFF; // Unused High RAM Area End
//                    0xFF00     SPECIAL MEMORY ADDRESSES (START)
pub const HRAM: u16 = 0xFF00; // Usable High RAM Area Start
pub const P1:   u16 = 0xFF00; // P1   (Read Joypad Info. & Determine System Type R/W)
pub const SB:   u16 = 0xFF01; // SB   (Serial Transfer Data R/W)
pub const SC:   u16 = 0xFF02; // SC   (Serial Transfer Control R/W)
//                    0xFF03
pub const DIV:  u16 = 0xFF04; // DIV  (Divider R/W)
pub const TIMA: u16 = 0xFF05; // TIMA (Timer Counter R/W)
pub const TMA:  u16 = 0xFF06; // TMA  (Timer Modulo R/W)
pub const TAC:  u16 = 0xFF07; // TAC  (Timer Control R/W)
//             FROM   0xFF08
//               TO   0xFF0E
pub const IF:   u16 = 0xFF0F; // IF   (Interrupt Flag R/W)
pub const NR10: u16 = 0xFF10; // NR10 (Sound Mode 1, Sweep Register R/W)
pub const NR11: u16 = 0xFF11; // NR11 (Sound Mode 1, Sound Length / Wave Pattern Duty R/W)
pub const NR12: u16 = 0xFF12; // NR12 (Sound Mode 1, Envelope R/W)
pub const NR13: u16 = 0xFF13; // NR13 (Sound Mode 1, Frequency Low W)
pub const NR14: u16 = 0xFF14; // NR14 (Sound Mode 1, Frequency High R/W)
//                    0xFF15
pub const NR21: u16 = 0xFF16; // NR21 (Sound Mode 2, Sound Length / Wave Pattern Duty R/W)
pub const NR22: u16 = 0xFF17; // NR22 (Sound Mode 2, Envelope R/W)
pub const NR23: u16 = 0xFF18; // NR23 (Sound Mode 2, Frequency Low W)
pub const NR24: u16 = 0xFF19; // NR24 (Sound Mode 2, Frequency High R/W)
pub const NR30: u16 = 0xFF1A; // NR30 (Sound Mode 3, Sound ON/OFF RW)
pub const NR31: u16 = 0xFF1B; // NR31 (Sound Mode 3, Sound Length R/W)
pub const NR32: u16 = 0xFF1C; // NR32 (Sound Mode 3, Select Output Level R/W)
pub const NR33: u16 = 0xFF1D; // NR33 (Sound Mode 3, Frequency Lower Data W)
pub const NR34: u16 = 0xFF1E; // NR34 (Sound Mode 3, Frequency Higher Data R/W)
pub const NR41: u16 = 0xFF20; // NR41 (Sound Mode 4, SOund Length R/W)
pub const NR42: u16 = 0xFF21; // NR42 (Sound Mode 4, Envelope R/W)
pub const NR43: u16 = 0xFF22; // NR43 (Sound Mode 4, Polynomial Counter R/W)
pub const NR44: u16 = 0xFF23; // NR44 (Sound Mode 4, Counter/Consecutive; Initial R/W)
pub const NR50: u16 = 0xFF24; // NR50 (Channel Control / ON/OFF / Volume R/W)
pub const NR51: u16 = 0xFF25; // NR51 (Selection of Sound Output Terminal R/W)
pub const NR52: u16 = 0xFF26; // NR52 (Sound ON/OFF R/W)
//             FROM   0xFF27
//               TO   0xFF2F
pub const WPR0: u16 = 0xFF30; // (Codebase Acronym) Wave Pattern RAM Bank Start
//             FROM   0xFF31
//               TO   0xFF3E
pub const WPR9: u16 = 0xFF3F; // (Codebase Acronym) Wave Pattern RAM Bank End
pub const LCDC: u16 = 0xFF40; // LCDC (LCD Control R/W)
pub const STAT: u16 = 0xFF41; // STAT (LCD Control Status R/W)
pub const SCY:  u16 = 0xFF42; // SCY  (Scroll Y R/W)
pub const SCX:  u16 = 0xFF43; // SCX  (Scroll X R/W)
pub const LY:   u16 = 0xFF44; // LY   (LCD Control Y-Coordinate R)
pub const LYC:  u16 = 0xFF45; // LYC  (LY Compare R/W)
pub const DMA:  u16 = 0xFF46; // DMA  (DMA Transfer and Start Address W)
pub const BGP:  u16 = 0xFF47; // BGP  (Background & Window Palette Data R/W)
pub const OBP0: u16 = 0xFF48; // OBP0 (Object Palette 0 Data R/W)
pub const OBP1: u16 = 0xFF49; // OBP1 (Object Palette 1 Data R/W)
pub const WY:   u16 = 0xFF4A; // WY   (Window Y Position R/W)
pub const WX:   u16 = 0xFF4B; // WX   (Window X Position R/W) (Minus 7)
//             FROM   0xFF4C
pub const DMGS: u16 = 0xFF50; // DMG ROM Status
//               TO   0xFFFE
pub const IE:   u16 = 0xFFFF; // IE   (Interrupt Enable R/W)
//                    0xFFFF     SPECIAL MEMORY ADDRESSES (END)
pub const RAM9: u16 = 0xFFFF; // (Codebase Acronym) High RAM Area End
