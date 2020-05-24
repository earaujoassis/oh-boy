; This code may be copyrighted. It was made available as a mean of learning and sharing knowledge.
; It was obtained through the following web address: https://gist.github.com/drhelius/6063288
; If you're the author/copyright holder, please let me know if you want me to remove this code.

    LD SP,$FFFE         ; $0000  Setup Stack
    XOR A               ; $0003  Zero the memory from $8000-$9FFF (VRAM)
    LD HL,$9FFF         ; $0004
Addr_0007:
    LD (HL-),A          ; $0007
    BIT 7,H             ; $0008
    JR NZ, Addr_0007    ; $000A
    LD HL,$FF26         ; $000C  Setup Audio
    LD C,$11            ; $000F
    LD A,$80            ; $0011
    LD (HL-),A          ; $0013
    LD ($FF00+C),A      ; $0014
    INC C               ; $0015
    LD A,$F3            ; $0016
    LD ($FF00+C),A      ; $0018
    LD (HL-),A          ; $0019
    LD A,$77            ; $001A
    LD (HL),A           ; $001C
    LD A,$FC            ; $001D  Setup BG Palette
    LD ($FF00+$47),A    ; $001F
    LD DE,$0104         ; $0021  Convert and load logo data from cart into Video RAM
    LD HL,$8010         ; $0024
Addr_0027:
    LD A,(DE)           ; $0027
    CALL Addr_0095      ; $0028
    CALL Addr_0096      ; $002B
    INC DE              ; $002E
    LD A,E              ; $002F
    CP $34              ; $0030
    JR NZ, Addr_0027    ; $0032
    LD DE,$00D8         ; $0034  Load 8 additional bytes into Video RAM
    LD B,$08            ; $0037
Addr_0039:
    LD A,(DE)           ; $0039
    INC DE              ; $003A
    LD (HL+),A          ; $003B
    INC HL              ; $003C
    DEC B               ; $003D
    JR NZ, Addr_0039    ; $003E
    LD A,$19            ; $0040  Setup Background Tilemap
    LD ($9910),A        ; $0042
    LD HL,$992F         ; $0045
Addr_0048:
    LD C,$0C            ; $0048
Addr_004A:
    DEC A               ; $004A
    JR Z, Addr_0055     ; $004B
    LD (HL-),A          ; $004D
    DEC C               ; $004E
    JR NZ, Addr_004A    ; $004F
    LD L,$0F            ; $0051
    JR Addr_0048        ; $0053

    ; === Scroll logo on screen, and play logo sound ===

Addr_0055:
    LD H,A              ; $0055  Initialize scroll count, H=0
    LD A,$64            ; $0056
    LD D,A              ; $0058  Set loop count, D=$64
    LD ($FF00+$42),A    ; $0059  Set vertical scroll register
    LD A,$91            ; $005B
    LD ($FF00+$40),A    ; $005D  Turn on LCD, showing Background
    INC B               ; $005F  Set B=1
Addr_0060:
    LD E,$02            ; $0060
Addr_0062:
    LD C,$0C            ; $0062
Addr_0064:
    LD A,($FF00+$44)    ; $0064  Wait for screen frame
    CP $90              ; $0066
    JR NZ, Addr_0064    ; $0068
    DEC C               ; $006A
    JR NZ, Addr_0064    ; $006B
    DEC E               ; $006D
    JR NZ, Addr_0062    ; $006E
    LD C,$13            ; $0070  Setup for scroll
    INC H               ; $0072  Increment scroll count
    LD A,H              ; $0073
    LD E,$83            ; $0074
    CP $62              ; $0076  $62 counts in, play sound #1
    JR Z, Addr_0080     ; $0078
    LD E,$C1            ; $007A
    CP $64              ; $007C
    JR NZ, Addr_0086    ; $007E  $64 counts in, play sound #2
Addr_0080:
    LD A,E              ; $0080  Play sound
    LD ($FF00+C),A      ; $0081
    INC C               ; $0082
    LD A,$87            ; $0083
    LD ($FF00+C),A      ; $0085
Addr_0086:
    LD A,($FF00+$42)    ; $0086
    SUB B               ; $0088
    LD ($FF00+$42),A    ; $0089  Scroll logo up if B=1
    DEC D               ; $008B
    JR NZ, Addr_0060    ; $008C
    DEC B               ; $008E  Set B=0 First time
    JR NZ, Addr_00E0    ; $008F      ... Next time, cause jump to "Nintendo Logo check"
    LD D,$20            ; $0091  Use scrolling loop to pause
    JR Addr_0060        ; $0093

    ; ==== Graphic Routine ====

Addr_0095:
    LD C,A              ; $0095  "Double up" all the bits of the graphics data and store in Video RAM
Addr_0096:
    LD B,$04            ; $0096
Addr_0098:
    PUSH BC             ; $0098
    RL C                ; $0099
    RLA                 ; $009B
    POP BC              ; $009C
    RL C                ; $009D
    RLA                 ; $009F
    DEC B               ; $00A0
    JR NZ, Addr_0098    ; $00A1
    LD (HL+),A          ; $00A3
    INC HL              ; $00A4
    LD (HL+),A          ; $00A5
    INC HL              ; $00A6
    RET                 ; $00A7

Addr_00A8:
    ;Nintendo Logo
    .DB $CE,$ED,$66,$66,$CC,$0D,$00,$0B,$03,$73,$00,$83,$00,$0C,$00,$0D
    .DB $00,$08,$11,$1F,$88,$89,$00,$0E,$DC,$CC,$6E,$E6,$DD,$DD,$D9,$99
    .DB $BB,$BB,$67,$63,$6E,$0E,$EC,$CC,$DD,$DC,$99,$9F,$BB,$B9,$33,$3E

Addr_00D8:
    ;More video data
    .DB $3C,$42,$B9,$A5,$B9,$A5,$42,$3C

    ; ===== Nintendo logo comparison routine =====

Addr_00E0:
    LD HL,$0104         ; $00E0  Point HL to Nintendo logo in cart
    LD DE,$00A8         ; $00E3  Point DE to Nintendo logo in DMG ROM
Addr_00E6:
    LD A,(DE)           ; $00E6
    INC DE              ; $00E7
    CP (HL)             ; $00E8  Compare logo data in cart to DMG ROM
    JR NZ,$FE           ; $00E9  If not a match, lock up here
    INC HL              ; $00EB
    LD A,L              ; $00EC
    CP $34              ; $00ED  Do this for $30 bytes
    JR NZ, Addr_00E6    ; $00EF
    LD B,$19            ; $00F1
    LD A,B              ; $00F3
Addr_00F4:
    ADD (HL)            ; $00F4
    INC HL              ; $00F5
    DEC B               ; $00F6
    JR NZ, Addr_00F4    ; $00F7
    ADD (HL)            ; $00F9
    JR NZ,$FE           ; $00FA  If $19 + bytes from $0134-$014D  don't add to $00 lock up
    LD A,$01            ; $00FC
    LD ($FF00+$50),A    ; $00FE  Turn-off DMG ROM
