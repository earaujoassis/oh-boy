pub fn subdecode(opcode: u8) -> String {
    match opcode {
        /* RLC r/(HL) */ 0x00..=0x07 => {
            let r = match opcode & 0x07 {
                0x07 => "A",
                0x00 => "B",
                0x01 => "C",
                0x02 => "D",
                0x03 => "E",
                0x04 => "H",
                0x05 => "L",
                0x06 => "(HL)",
                _ => "?",
            };
            format!("RLC {}", r)
        },
        /* RRC r/(HL) */ 0x08..=0x0F => {
            let r = match opcode & 0x07 {
                0x07 => "A",
                0x00 => "B",
                0x01 => "C",
                0x02 => "D",
                0x03 => "E",
                0x04 => "H",
                0x05 => "L",
                0x06 => "(HL)",
                _ => "?",
            };
            format!("RRC {}", r)
        },
        /* RL r/(HL) */ 0x10..=0x17 => {
            let r = match opcode & 0x07 {
                0x07 => "A",
                0x00 => "B",
                0x01 => "C",
                0x02 => "D",
                0x03 => "E",
                0x04 => "H",
                0x05 => "L",
                0x06 => "(HL)",
                _ => "?",
            };
            format!("RL {}", r)
        },
        /* RR r/(HL) */ 0x18..=0x1F => {
            let r = match opcode & 0x07 {
                0x07 => "A",
                0x00 => "B",
                0x01 => "C",
                0x02 => "D",
                0x03 => "E",
                0x04 => "H",
                0x05 => "L",
                0x06 => "(HL)",
                _ => "?",
            };
            format!("RR {}", r)
        },
        /* SLA r/(HL) */ 0x20..=0x27 => {
            let r = match opcode & 0x07 {
                0x07 => "A",
                0x00 => "B",
                0x01 => "C",
                0x02 => "D",
                0x03 => "E",
                0x04 => "H",
                0x05 => "L",
                0x06 => "(HL)",
                _ => "?",
            };
            format!("SLA {}", r)
        },
        /* SRA r/(HL) */ 0x28..=0x2F => {
            let r = match opcode & 0x07 {
                0x07 => "A",
                0x00 => "B",
                0x01 => "C",
                0x02 => "D",
                0x03 => "E",
                0x04 => "H",
                0x05 => "L",
                0x06 => "(HL)",
                _ => "?",
            };
            format!("SRA {}", r)
        },
        /* SWAP r/(HL) */ 0x30..=0x37 => {
            let r = match opcode & 0x07 {
                0x07 => "A",
                0x00 => "B",
                0x01 => "C",
                0x02 => "D",
                0x03 => "E",
                0x04 => "H",
                0x05 => "L",
                0x06 => "(HL)",
                _ => "?",
            };
            format!("SWAP {}", r)
        },
        /* SRL r/(HL) */ 0x38..=0x3F => {
            let r = match opcode & 0x07 {
                0x07 => "A",
                0x00 => "B",
                0x01 => "C",
                0x02 => "D",
                0x03 => "E",
                0x04 => "H",
                0x05 => "L",
                0x06 => "(HL)",
                _ => "?",
            };
            format!("SRL {}", r)
        },
        /* BIT b,r/(HL) */ 0x40..=0x7F => {
            let bit = match (opcode >> 3) & 0xFF {
                0x08 => "0",
                0x09 => "1",
                0x0A => "2",
                0x0B => "3",
                0x0C => "4",
                0x0D => "5",
                0x0E => "6",
                0x0F => "7",
                _ => "?",
            };
            let r = match (opcode << 5) & 0xFF {
                0xE0 => "A",
                0x00 => "B",
                0x20 => "C",
                0x40 => "D",
                0x60 => "E",
                0x80 => "H",
                0xA0 => "L",
                0xC0 => "(HL)",
                _ => "?",
            };
            format!("BIT {},{}", bit, r)
        },
        /* RES b,r/(HL) */ 0x80..=0xBF => {
            let bit = match (opcode >> 3) & 0xFF {
                0x10 => "0",
                0x11 => "1",
                0x12 => "2",
                0x13 => "3",
                0x14 => "4",
                0x15 => "5",
                0x16 => "6",
                0x17 => "7",
                _ => "?",
            };
            let r = match (opcode << 5) & 0xFF {
                0xE0 => "A",
                0x00 => "B",
                0x20 => "C",
                0x40 => "D",
                0x60 => "E",
                0x80 => "H",
                0xA0 => "L",
                0xC0 => "(HL)",
                _ => "?",
            };
            format!("RES {},{}", bit, r)
        },
        /* SET b,r/(HL) */ 0xC0..=0xFF => {
            let bit = match (opcode >> 3) & 0xFF {
                0x18 => "0",
                0x19 => "1",
                0x1A => "2",
                0x1B => "3",
                0x1C => "4",
                0x1D => "5",
                0x1E => "6",
                0x1F => "7",
                _ => "?",
            };
            let r = match (opcode << 5) & 0xFF {
                0xE0 => "A",
                0x00 => "B",
                0x20 => "C",
                0x40 => "D",
                0x60 => "E",
                0x80 => "H",
                0xA0 => "L",
                0xC0 => "(HL)",
                _ => "?",
            };
            format!("SET {},{}", bit, r)
        },
    }
}
