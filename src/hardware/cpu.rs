use std::env;

use super::instruction_set;
use super::memory::Memory;
use super::disassembler;

/// # The CPU Registers
///
/// ## The Flag Register (F) Bits
///
/// 7   6   5   4   3   2   1   0
/// Z   N   H  C Y  0   0   0   0
///
/// ### According to the "GAME BOY Programming Manual Version 1.1"
///
/// Consists of 4 flags that are set and reset according to the results of
/// instruction execution. Flags CY and Z are tested by various conditional
/// branch instructions.
/// Z: Set to 1 when the result of an operation is 0; otherwise reset.
/// N: Set to 1 following execution of the substruction instruction,
/// regardless of the result.
/// H: Set to 1 when an operation results in carrying from or borrowing to bit 3.
/// CY: Set to 1 when an operation results in carrying from or borrowing to bit 7.
///
/// ### According to the "GAME BOY CPU Manual Version 1.01 by DP"
///
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
/// CY      Carry Flag (CY)
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
    pub r_a: u8,
    /// Flag Register
    pub r_f: u8,
    pub r_b: u8,
    pub r_c: u8,
    pub r_d: u8,
    pub r_e: u8,
    pub r_h: u8,
    pub r_l: u8,
    /// Stack Pointer Register
    pub stack_pointer: u16,
    /// Program Counter Register
    pub program_counter: u16,
    // Memory Address Register (MAR)
    address_register: u16,
    // Memory Data Register (MDR)
    data_register: u8,
    // Current Instruction Register
    instruction_register: u8,
}

pub struct CPU {
    pub registers: CPURegisters,
    // Interrupt Master Enable Flag (IME)
    pub interruption_enabled: bool,
    pub stopped: bool,
    pub halted: bool,
    pub debug_mode: bool,
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
            stack_pointer: 0x0000,
            program_counter: 0x0000,
            address_register: 0x0000,
            data_register: 0x00,
            instruction_register: 0x00,
        };

        let debug_mode: bool = debug_mode!();

        CPU {
            registers: registers,
            interruption_enabled: true,
            stopped: false,
            halted: false,
            debug_mode: debug_mode,
        }
    }

    pub fn boot(&mut self) {
        self.registers.r_a = 0x00;
        self.registers.r_f = 0x00;
        self.registers.r_b = 0x00;
        self.registers.r_c = 0x00;
        self.registers.r_d = 0x00;
        self.registers.r_e = 0x00;
        self.registers.r_h = 0x00;
        self.registers.r_l = 0x00;
        self.registers.stack_pointer = 0xFFFF;
        self.registers.program_counter = 0x0000;
        self.registers.address_register = 0x0000;
        self.registers.data_register = 0x00;
        self.registers.instruction_register = 0x00;
    }

    pub fn boot_expected(&mut self) {
        self.registers.r_a = 0x01;
        self.registers.r_f = 0xB0;
        self.registers.r_b = 0x00;
        self.registers.r_c = 0x13;
        self.registers.r_d = 0x00;
        self.registers.r_e = 0xD8;
        self.registers.r_h = 0x01;
        self.registers.r_l = 0x4D;
        self.registers.stack_pointer = 0xFFFE;
        self.registers.program_counter = 0x0000;
        self.registers.address_register = 0x0000;
        self.registers.data_register = 0x00;
        self.registers.instruction_register = 0x00;
    }

    /// This represents the fetch–decode–execute cycle (or instruction cycle).
    pub fn cycle(&mut self, memory: &mut Memory) -> usize {
        // Fetch the instruction in the memory
        //     Memory Address Register (MAR) <- from the PC register
        //     Memory Data Register (MDR) <- Loads the data from the memory
        //     Current Instruction Register (CIR) <- Copy from the MDR (Memory Data Register)
        self.registers.address_register = self.registers.program_counter;
        self.registers.data_register = memory.fetch(self.registers.address_register);
        self.registers.instruction_register = self.registers.data_register;
        // The PC is set to point to the next instruction or operand. This is
        // necessary for all jumps/calls instructions
        self.registers.program_counter += 1;
        // Decode
        //     Check what instruction should be executed
        // Execute
        debug_system!(format!("{}", disassembler::decode(self.registers.instruction_register)), self.debug_mode);
        instruction_set::execute(self, memory, self.registers.instruction_register)
        //     Interruption Handler
    }

    /// Used to fetch operands for a given instruction.
    /// It keeps the program counter in a safe state (the instruction set executor doens't have to change it,
    /// exceptionally when the opcode states that).
    pub fn fetch_operand(&mut self, memory: &mut Memory) -> u8 {
        // The PC is already pointing to the operand
        self.registers.address_register = self.registers.program_counter;
        self.registers.data_register = memory.fetch(self.registers.address_register);
        // The PC is set to point to the next instruction or operand
        self.registers.program_counter += 1;
        debug_system!(format!("op={:#04X}", self.registers.data_register), self.debug_mode);
        self.registers.data_register
    }

    pub fn fetch_data(&mut self, memory: &mut Memory, address: u16) -> u8 {
        self.registers.address_register = address;
        self.registers.data_register = memory.fetch(self.registers.address_register);
        debug_system!(format!("fetch={:#04X}", self.registers.data_register), self.debug_mode);
        self.registers.data_register
    }

    pub fn write_data(&mut self, memory: &mut Memory, address: u16, word: u8) {
        self.registers.address_register = address;
        self.registers.data_register = word;
        memory.write(address, word);
    }

}
