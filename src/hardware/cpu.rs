/// The CPU Registers
///
/// The Flag Register Bits
/// 7   6   5   4   3   2   1   0
/// Z   N   H   C   0   0   0   0
/// Z       Zero Flag
///         It is set when the result of a math operation is zero; or two
///         values match when using the CP instruction
///
/// N       Subtract Flag
///         It is set if a subtraction was performed in the last math
///         instruction
///
/// H       Half Carry Flag (H)
///         It is set if a carry occurred from the lower nibble in the last
///         math operation
///
/// C       Carry Flag (C)
///         It is set if a carry occurred from the last math operation;
///         or if the register A is the smaller value when executing the CP
///         instruction
///
/// The Program Counter (PC) Register
/// On Power Up, the Game Boy Program Counter (PC) is initialized to 0x0100
/// and the instruction found in this location in ROM is executed. The PC is
/// controlled indirectly hereforth by the program instructions
///
/// The Stack Pointer (SP) Register
/// The Stack Pointer (SP) is directly defined by the operator (programmer),
/// but initialized on Power Up to 0xFFFE.
///
/// Register Organization
///
/// |-----|-----|
/// |  A  |  F  | -> 16 bit total (8 bit each)
/// |-----------|
/// |  B  |  C  | -> 16 bit total (8 bit each)
/// |-----------|
/// |  D  |  E  | -> 16 bit total (8 bit each)
/// |-----------|
/// |  H  |  L  | -> 16 bit total (8 bit each)
/// |-----------|
/// |    S P    | -> 16 bit total
/// |-----------|
/// |    P C    | -> 16 bit total
/// |-----------|
///
pub struct CPURegisters {
    r_a: u8,
    /// Flag Register
    r_f: u8,
    r_b: u8,
    r_c: u8,
    r_d: u8,
    r_e: u8,
    r_h: u8,
    r_l: u8,
    /// Stack Pointer Register
    r_sp: u16,
    /// Program Counter Register
    r_pc: u16,
}

pub struct CPU {
    registers: CPURegisters,
}

impl CPU {

    pub fn new() -> CPU {
        let registers = CPURegisters {
            r_a: 0x00,
            r_f: 0x00,
            r_b: 0x00,
            r_c: 0x00,
            r_d: 0x00,
            r_e: 0x00,
            r_h: 0x00,
            r_l: 0x00,
            r_sp: 0x0000,
            r_pc: 0x0000,
        };

        CPU {
            registers: registers,
        }
    }

    pub fn boot(&mut self) {
        self.registers.r_a = 0x01;
        self.registers.r_f = 0xB0;
        self.registers.r_b = 0x00;
        self.registers.r_c = 0x13;
        self.registers.r_d = 0x00;
        self.registers.r_e = 0xD8;
        self.registers.r_h = 0x01;
        self.registers.r_l = 0x4D;
        self.registers.r_sp = 0xFFFE;
        self.registers.r_pc = 0x0100;
    }

    /// This ignates the fetch–decode–execute cycle (or instruction cycle)
    pub fn start(&mut self) {
        // 'cycle: {
        //     Fetch the instruction in the memory
        //         Memory Address Register <- from the PC register
        //         Memory Data Register <- Loads the data from the memory
        //         Current Instruction Register <- Copy from the MDR (Memory Data Register)
        //     Decode
        //         Check what instruction should be executed
        //     Execute
        //     Interruption Handler
        //     Restart from 'cycle
        // }
    }

}
