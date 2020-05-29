/// The Timer (TIMA TMA TAC) Component

use super::memory::Memory;
use super::memory_map;
use super::interrupt::{Flag as InterruptFlag};

const DIV_THRESHOLD: usize = 256;  // 16384 Hz
const CLOCK0_CYCLES: isize = 1024; // 4096 Hz
const CLOCK1_CYCLES: isize = 16;   // 262144 Hz
const CLOCK2_CYCLES: isize = 64;   // 65536 Hz
const CLOCK3_CYCLES: isize = 256;  // 16384 Hz

pub struct Timer {
    pub divider_accumulated_cycles: usize,
    pub accumulated_cycles: isize,
    pub frequency: u8,
}

impl Timer {

    pub fn new() -> Timer {
        Timer {
            divider_accumulated_cycles: 0,
            accumulated_cycles: 1024,
            frequency: 0x00,
        }
    }

    pub fn updater(&mut self, memory: &mut Memory, cycles: usize) {
        let timer_enabled: bool = (memory.fetch(memory_map::TAC) & 0x04) > 0;
        let timer_frequency_control: u8 = memory.fetch(memory_map::TAC) & 0x03;
        let timer_frequency_cycles: isize = match timer_frequency_control {
            0x00 => CLOCK0_CYCLES,
            0x01 => CLOCK1_CYCLES,
            0x02 => CLOCK2_CYCLES,
            0x03 => CLOCK3_CYCLES,
            _    => panic!("Oops!... There's a bug at timer"),
        };

        self.divider_accumulated_cycles += cycles;

        if self.divider_accumulated_cycles >= DIV_THRESHOLD {
            let current_div = memory.fetch(memory_map::DIV);
            self.divider_accumulated_cycles = 0;
            memory.write(memory_map::DIV, current_div.wrapping_add(1));
        }

        if self.frequency != timer_frequency_control {
            self.frequency = timer_frequency_control;
            self.accumulated_cycles = timer_frequency_cycles;
        }

        if timer_enabled {
            self.accumulated_cycles -= cycles as isize;
            if self.accumulated_cycles <= 0 {
                let timer_counter: u8 = memory.fetch(memory_map::TIMA);
                let timer_modulo: u8 = memory.fetch(memory_map::TMA);
                self.accumulated_cycles = timer_frequency_cycles;

                // Reached overflow and it must request for interruption
                if timer_counter == 255 {
                    memory.write(memory_map::TIMA, timer_modulo);
                    request_interrupt(memory, InterruptFlag::TIMER);
                } else {
                    memory.write(memory_map::TIMA, timer_counter.wrapping_add(1));
                }
            }
        }
    }

}

fn request_interrupt(memory: &mut Memory, flag: InterruptFlag) {
    let interrupt_request: u8 = memory.fetch(memory_map::IF) as u8 | (flag as u8);
    memory.write(memory_map::IF, interrupt_request);
}
