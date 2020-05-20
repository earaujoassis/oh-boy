/// This function is used to decode (or debug) an instruction for a given byte/opcode
pub fn decode(opcode: u8) -> String {
    match opcode {
        /* NOP  */ 0x00 => { "NOP".to_string() },
        /* STOP */ 0x10 => { "STOP".to_string() },
        /* HALT */ 0x76 => { "HALT".to_string() },
        /* DI   */ 0xF3 => { "DI".to_string() },
        /* EI   */ 0xFB => { "EI".to_string() },
        /* LD BC,d16 */ 0x01 => { "LD BC,d16".to_string() },
        /* LD (a16),SP */ 0x08 => { "LD (a16),SP".to_string() },
        /* LD DE,d16 */ 0x11 => { "LD DE,d16".to_string() },
        /* LD HL,d16 */ 0x21 => { "LD HL,d16".to_string() },
        /* LD SP,d16 */ 0x31 => { "LD SP,d16".to_string() },
        /* POP BC */ 0xC1 => { "POP BC".to_string() },
        /* POP DE */ 0xD1 => { "POP DE".to_string() },
        /* POP HL */ 0xE1 => { "POP HL".to_string() },
        /* POP AF */ 0xF1 => { "POP AF".to_string() },
        /* PUSH BC */ 0xC5 => { "PUSH BC".to_string() },
        /* PUSH DE */ 0xD5 => { "PUSH DE".to_string() },
        /* PUSH HL */ 0xE5 => { "PUSH HL".to_string() },
        /* PUSH AF */ 0xF5 => { "PUSH AF".to_string() },
        /* LD HL,SP+r8 */ 0xF8 => { "LD HL,SP+r8".to_string() },
        /* LD SP,HL */ 0xF9 => { "LD SP,HL".to_string() },
        /* INC BC */ 0x03 => { "INC BC".to_string() },
        /* INC DE */ 0x13 => { "INC DE".to_string() },
        /* INC HL */ 0x23 => { "INC HL".to_string() },
        /* INC SP */ 0x33 => { "INC SP".to_string() },
        /* ADD HL,BC */ 0x09 => { "ADD HL,BC".to_string() },
        /* ADD HL,DE */ 0x19 => { "ADD HL,DE".to_string() },
        /* ADD HL,HL */ 0x29 => { "ADD HL,HL".to_string() },
        /* ADD HL,SP */ 0x39 => { "ADD HL,SP".to_string() },
        /* DEC BC */ 0x0B => { "DEC BC".to_string() },
        /* DEC DE */ 0x1B => { "DEC DE".to_string() },
        /* DEC HL */ 0x2B => { "DEC HL".to_string() },
        /* DEC SP */ 0x3B => { "DEC SP".to_string() },
        /* ADD SP,r8 */ 0xE8 => { "ADD SP,r8".to_string() },
        /* JR Z,r8 */ 0x28 => { "JR Z,r8".to_string() },
        /* JR C,r8 */ 0x38 => { "JR C,r8".to_string() },
        /* JR NZ,r8 */ 0x20 => { "JR NZ,r8".to_string() },
        /* JR NC,r8 */ 0x30 => { "JR NC,r8".to_string() },
        /* JR r8 */ 0x18 => { "JR r8".to_string() },
        /* JP Z,a16 */ 0xCA => { "JP Z,a16".to_string() },
        /* JP C,a16 */ 0xDA => { "JP C,a16".to_string() },
        /* JP NZ,a16 */ 0xC2 => { "JP NZ,a16".to_string() },
        /* JP NC,a16 */ 0xD2 => { "JP NC,a16".to_string() },
        /* JP a16 */ 0xC3 => { "JP a16".to_string() },
        /* JP (HL) */ 0xE9 => { "JP (HL)".to_string() },
        /* CALL Z,a16 */ 0xCC => { "CALL Z,a16".to_string() },
        /* CALL C,a16 */ 0xDC => { "CALL C,a16".to_string() },
        /* CALL NZ,a16 */ 0xC4 => { "CALL NZ,a16".to_string() },
        /* CALL NC,a16 */ 0xD4 => { "CALL NC,a16".to_string() },
        /* CALL a16 */ 0xCD => { "CALL a16".to_string() },
        /* RET Z */ 0xC8 => { "RET Z".to_string() },
        /* RET C */ 0xD8 => { "RET C".to_string() },
        /* RET NZ */ 0xC0 => { "RET NZ".to_string() },
        /* RET NC */ 0xD0 => { "RET NC".to_string() },
        /* RET */ 0xC9 => { "RET".to_string() },
        /* RETI */ 0xD9 => { "RETI".to_string() },
        /* RST 00H */ 0xC7 => { "RST 00H".to_string() },
        /* RST 10H */ 0xD7 => { "RST 10H".to_string() },
        /* RST 20H */ 0xE7 => { "RST 20H".to_string() },
        /* RST 30H */ 0xF7 => { "RST 30H".to_string() },
        /* RST 08H */ 0xCF => { "RST 08H".to_string() },
        /* RST 18H */ 0xDF => { "RST 18H".to_string() },
        /* RST 28H */ 0xEF => { "RST 28H".to_string() },
        /* RST 38H */ 0xFF => { "RST 38H".to_string() },
        /* LDH (a8),A */ 0xE0 => { "LDH (a8),A".to_string() },
        /* LDH A,(a8) */ 0xF0 => { "LDH A,(a8)".to_string() },
        /* LD (C),A */ 0xE2 => { "LD (C),A".to_string() },
        /* LD A,(C) */ 0xF2 => { "LD A,(C)".to_string() },
        /* LD (a16),A */ 0xEA => { "LD (a16),A".to_string() },
        /* LD A,(a16) */ 0xFA => { "LD A,(a16)".to_string() },
        /* LD B,B */ 0x40 => { "LD B,B".to_string() },
        /* LD B,C */ 0x41 => { "LD B,C".to_string() },
        /* LD B,D */ 0x42 => { "LD B,D".to_string() },
        /* LD B,E */ 0x43 => { "LD B,E".to_string() },
        /* LD B,H */ 0x44 => { "LD B,H".to_string() },
        /* LD B,L */ 0x45 => { "LD B,L".to_string() },
        /* LD B,(HL) */ 0x46 => { "LD B,(HL)".to_string() },
        /* LD B,A */ 0x47 => { "LD B,A".to_string() },
        /* LD C,B */ 0x48 => { "LD C,B".to_string() },
        /* LD C,C */ 0x49 => { "LD C,C".to_string() },
        /* LD C,D */ 0x4A => { "LD C,D".to_string() },
        /* LD C,E */ 0x4B => { "LD C,E".to_string() },
        /* LD C,H */ 0x4C => { "LD C,H".to_string() },
        /* LD C,L */ 0x4D => { "LD C,L".to_string() },
        /* LD C,(HL) */ 0x4E => { "LD C,(HL)".to_string() },
        /* LD C,A */ 0x4F => { "LD C,A".to_string() },
        /* LD D,B */ 0x50 => { "LD D,B".to_string() },
        /* LD D,C */ 0x51 => { "LD D,C".to_string() },
        /* LD D,D */ 0x52 => { "LD D,D".to_string() },
        /* LD D,E */ 0x53 => { "LD D,E".to_string() },
        /* LD D,H */ 0x54 => { "LD D,H".to_string() },
        /* LD D,L */ 0x55 => { "LD D,L".to_string() },
        /* LD D,(HL) */ 0x56 => { "LD D,(HL)".to_string() },
        /* LD D,A */ 0x57 => { "LD D,A".to_string() },
        /* LD E,B */ 0x58 => { "LD E,B".to_string() },
        /* LD E,C */ 0x59 => { "LD E,C".to_string() },
        /* LD E,D */ 0x5A => { "LD E,D".to_string() },
        /* LD E,E */ 0x5B => { "LD E,E".to_string() },
        /* LD E,H */ 0x5C => { "LD E,H".to_string() },
        /* LD E,L */ 0x5D => { "LD E,L".to_string() },
        /* LD E,(HL) */ 0x5E => { "LD E,(HL)".to_string() },
        /* LD E,A */ 0x5F => { "LD E,A".to_string() },
        /* LD H,B */ 0x60 => { "LD H,B".to_string() },
        /* LD H,C */ 0x61 => { "LD H,C".to_string() },
        /* LD H,D */ 0x62 => { "LD H,D".to_string() },
        /* LD H,E */ 0x63 => { "LD H,E".to_string() },
        /* LD H,H */ 0x64 => { "LD H,H".to_string() },
        /* LD H,L */ 0x65 => { "LD H,L".to_string() },
        /* LD H,(HL) */ 0x66 => { "LD H,(HL)".to_string() },
        /* LD H,A */ 0x67 => { "LD H,A".to_string() },
        /* LD L,B */ 0x68 => { "LD L,B".to_string() },
        /* LD L,C */ 0x69 => { "LD L,C".to_string() },
        /* LD L,D */ 0x6A => { "LD L,D".to_string() },
        /* LD L,E */ 0x6B => { "LD L,E".to_string() },
        /* LD L,H */ 0x6C => { "LD L,H".to_string() },
        /* LD L,L */ 0x6D => { "LD L,L".to_string() },
        /* LD L,(HL) */ 0x6E => { "LD L,(HL)".to_string() },
        /* LD L,A */ 0x6F => { "LD L,A".to_string() },
        /* LD (HL),B */ 0x70 => { "LD (HL),B".to_string() },
        /* LD (HL),C */ 0x71 => { "LD (HL),C".to_string() },
        /* LD (HL),D */ 0x72 => { "LD (HL),D".to_string() },
        /* LD (HL),E */ 0x73 => { "LD (HL),E".to_string() },
        /* LD (HL),H */ 0x74 => { "LD (HL),H".to_string() },
        /* LD (HL),L */ 0x75 => { "LD (HL),L".to_string() },
        /* LD (HL),A */ 0x77 => { "LD (HL),A".to_string() },
        /* LD A,B */ 0x78 => { "LD A,B".to_string() },
        /* LD A,C */ 0x79 => { "LD A,C".to_string() },
        /* LD A,D */ 0x7A => { "LD A,D".to_string() },
        /* LD A,E */ 0x7B => { "LD A,E".to_string() },
        /* LD A,H */ 0x7C => { "LD A,H".to_string() },
        /* LD A,L */ 0x7D => { "LD A,L".to_string() },
        /* LD A,(HL) */ 0x7E => { "LD A,(HL)".to_string() },
        /* LD A,A */ 0x7F => { "LD A,A".to_string() },
        /* LD (BC),A */ 0x02 => { "LD (BC),A".to_string() },
        /* LD (DE),A */ 0x12 => { "LD (DE),A".to_string() },
        /* LD (HL+),A */ 0x22 => { "LD (HL+),A".to_string() },
        /* LD (HL-),A */ 0x32 => { "LD (HL-),A".to_string() },
        /* LD B,d8 */ 0x06 => { "LD B,d8".to_string() },
        /* LD D,d8 */ 0x16 => { "LD D,d8".to_string() },
        /* LD H,d8 */ 0x26 => { "LD H,d8".to_string() },
        /* LD (HL),d8 */ 0x36 => { "LD (HL),d8".to_string() },
        /* LD A,(BC) */ 0x0A => { "LD A,(BC)".to_string() },
        /* LD A,(DE) */ 0x1A => { "LD A,(DE)".to_string() },
        /* LD A,(HL+) */ 0x2A => { "LD A,(HL+)".to_string() },
        /* LD A,(HL-) */ 0x3A => { "LD A,(HL-)".to_string() },
        /* LD C,d8 */ 0x0E => { "LD C,d8".to_string() },
        /* LD E,d8 */ 0x1E => { "LD E,d8".to_string() },
        /* LD L,d8 */ 0x2E => { "LD L,d8".to_string() },
        /* LD A,d8 */ 0x3E => { "LD A,d8".to_string() },
        /* INC B */ 0x04 => { "INC B".to_string() },
        /* INC C */ 0x0C => { "INC C".to_string() },
        /* INC D */ 0x14 => { "INC D".to_string() },
        /* INC E */ 0x1C => { "INC E".to_string() },
        /* INC H */ 0x24 => { "INC H".to_string() },
        /* INC L */ 0x2C => { "INC L".to_string() },
        /* INC (HL) */ 0x34 => { "INC (HL)".to_string() },
        /* INC A */ 0x3C => { "INC A".to_string() },
        /* DEC B */ 0x05 => { "DEC B".to_string() },
        /* DEC C */ 0x0D => { "DEC C".to_string() },
        /* DEC D */ 0x15 => { "DEC D".to_string() },
        /* DEC E */ 0x1D => { "DEC E".to_string() },
        /* DEC H */ 0x25 => { "DEC H".to_string() },
        /* DEC L */ 0x2D => { "DEC L".to_string() },
        /* DEC (HL) */ 0x35 => { "DEC (HL)".to_string() },
        /* ADD A,B */ 0x80 => { "ADD A,B".to_string() },
        /* ADD A,C */ 0x81 => { "ADD A,C".to_string() },
        /* ADD A,D */ 0x82 => { "ADD A,D".to_string() },
        /* ADD A,E */ 0x83 => { "ADD A,E".to_string() },
        /* ADD A,H */ 0x84 => { "ADD A,H".to_string() },
        /* ADD A,L */ 0x85 => { "ADD A,L".to_string() },
        /* ADD A,(HL) */ 0x86 => { "ADD A,(HL)".to_string() },
        /* ADD A,A */ 0x87 => { "ADD A,A".to_string() },
        /* ADC A,B */ 0x88 => { "ADC A,B".to_string() },
        /* ADC A,C */ 0x89 => { "ADC A,C".to_string() },
        /* ADC A,D */ 0x8A => { "ADC A,D".to_string() },
        /* ADC A,E */ 0x8B => { "ADC A,E".to_string() },
        /* ADC A,H */ 0x8C => { "ADC A,H".to_string() },
        /* ADC A,L */ 0x8D => { "ADC A,L".to_string() },
        /* ADC A,(HL) */ 0x8E => { "ADC A,(HL)".to_string() },
        /* ADC A,A */ 0x8F => { "ADC A,A".to_string() },
        /* SUB B */ 0x90 => { "SUB B".to_string() },
        /* SUB C */ 0x91 => { "SUB C".to_string() },
        /* SUB D */ 0x92 => { "SUB D".to_string() },
        /* SUB E */ 0x93 => { "SUB E".to_string() },
        /* SUB H */ 0x94 => { "SUB H".to_string() },
        /* SUB L */ 0x95 => { "SUB L".to_string() },
        /* SUB (HL) */ 0x96 => { "SUB (HL)".to_string() },
        /* SUB A */ 0x97 => { "SUB A".to_string() },
        /* SBC A,B */ 0x98 => { "SBC A,B".to_string() },
        /* SBC A,C */ 0x99 => { "SBC A,C".to_string() },
        /* SBC A,D */ 0x9A => { "SBC A,D".to_string() },
        /* SBC A,E */ 0x9B => { "SBC A,E".to_string() },
        /* SBC A,H */ 0x9C => { "SBC A,H".to_string() },
        /* SBC A,L */ 0x9D => { "SBC A,L".to_string() },
        /* SBC A,(HL) */ 0x9E => { "SBC A,(HL)".to_string() },
        /* SBC A,A */ 0x9F => { "SBC A,A".to_string() },
        /* AND B */ 0xA0 => { "AND B".to_string() },
        /* AND C */ 0xA1 => { "AND C".to_string() },
        /* AND D */ 0xA2 => { "AND D".to_string() },
        /* AND E */ 0xA3 => { "AND E".to_string() },
        /* AND H */ 0xA4 => { "AND H".to_string() },
        /* AND L */ 0xA5 => { "AND L".to_string() },
        /* AND (HL) */ 0xA6 => { "AND (HL)".to_string() },
        /* AND A */ 0xA7 => { "AND A".to_string() },
        /* XOR B */ 0xA8 => { "XOR B".to_string() },
        /* XOR C */ 0xA9 => { "XOR C".to_string() },
        /* XOR D */ 0xAA => { "XOR D".to_string() },
        /* XOR E */ 0xAB => { "XOR E".to_string() },
        /* XOR H */ 0xAC => { "XOR H".to_string() },
        /* XOR L */ 0xAD => { "XOR L".to_string() },
        /* XOR (HL) */ 0xAE => { "XOR (HL)".to_string() },
        /* XOR A */ 0xAF => { "XOR A".to_string() },
        /* OR B */ 0xB0 => { "OR B".to_string() },
        /* OR C */ 0xB1 => { "OR C".to_string() },
        /* OR D */ 0xB2 => { "OR D".to_string() },
        /* OR E */ 0xB3 => { "OR E".to_string() },
        /* OR H */ 0xB4 => { "OR H".to_string() },
        /* OR L */ 0xB5 => { "OR L".to_string() },
        /* OR (HL) */ 0xB6 => { "OR (HL)".to_string() },
        /* OR A */ 0xB7 => { "OR A".to_string() },
        /* CP B */ 0xB8 => { "CP B".to_string() },
        /* CP C */ 0xB9 => { "CP C".to_string() },
        /* CP D */ 0xBA => { "CP D".to_string() },
        /* CP E */ 0xBB => { "CP E".to_string() },
        /* CP H */ 0xBC => { "CP H".to_string() },
        /* CP L */ 0xBD => { "CP L".to_string() },
        /* CP (HL) */ 0xBE => { "CP (HL)".to_string() },
        /* CP A */ 0xBF => { "CP A".to_string() },
        /* DEC A */ 0x3D => { "DEC A".to_string() },
        /* ADD A,d8 */ 0xC6 => { "ADD A,d8".to_string() },
        /* ADC A,d8 */ 0xCE => { "ADC A,d8".to_string() },
        /* SUB d8 */ 0xD6 => { "SUB d8".to_string() },
        /* SBC A,d8 */ 0xDE => { "SBC A,d8".to_string() },
        /* AND d8 */ 0xE6 => { "AND d8".to_string() },
        /* XOR d8 */ 0xEE => { "XOR d8".to_string() },
        /* OR d8 */ 0xF6 => { "OR d8".to_string() },
        /* CP d8 */ 0xFE => { "CP d8".to_string() },
        /* DAA */ 0x27 => { "DAA".to_string() },
        /* CPL */ 0x2F => { "CPL".to_string() },
        /* SCF */ 0x37 => { "SCF".to_string() },
        /* CCF */ 0x3F => { "CCF".to_string() },
        /* RLCA */ 0x07 => { "RLCA".to_string() },
        /* RLA */ 0x17 => { "RLA".to_string() },
        /* RRCA */ 0x0F => { "RRCA".to_string() },
        /* RRA */ 0x1F => { "RRA".to_string() },
        /* PREFIX CB */ 0xCB => { "PREFIX CB".to_string() },
        _ => panic!("Opcode unknown: ${:02X}", opcode)
    }
}