use super::cpu::CPU;
use super::ppu::PPU;
use super::memory::Memory;

pub struct System {
    cpu: CPU,
    ppu: PPU,
    memory: Memory,
}

impl System {

    pub fn new(file_path: String) -> System {
        let cpu = CPU::new();
        let ppu = PPU::new();
        let memory = Memory::new(file_path.to_owned());

        System {
            cpu: cpu,
            ppu: ppu,
            memory: memory,
        }
    }

    pub fn boot(&mut self) {
        self.cpu.boot();
    }

    pub fn cycle(&mut self) -> usize {
        let cpu_cycles: usize = self.cpu.cycle(&mut self.memory);
        // TODO Implement timer
        // self.timer.cycle(&mut self.memory, cycle);
        self.ppu.cycle(&mut self.memory, cpu_cycles);
        cpu_cycles
    }

    pub fn video_buffer(&mut self) -> Vec<u8> {
        let buffer = self.ppu.video.frame_buffer.clone();
        buffer
    }

    pub fn has_stopped(&mut self) -> bool {
        self.cpu.stopped
    }

    pub fn scanline_requested(&mut self) -> bool {
        self.ppu.scanline_requested
    }

}
