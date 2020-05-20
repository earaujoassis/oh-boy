use super::cpu::CPU;
use super::memory::Memory;

pub struct System {
    cpu: CPU,
    memory: Memory,
}

impl System {

    pub fn new(file_path: String) -> System {
        let cpu = CPU::new();
        let memory = Memory::new(file_path.to_owned());

        System {
            cpu: cpu,
            memory: memory,
        }
    }

    pub fn boot(&mut self) {
        self.cpu.boot();
    }

    pub fn cycle(&mut self) -> usize {
        self.cpu.cycle(&mut self.memory)
    }

}
