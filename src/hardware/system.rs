use super::cpu::CPU;
use super::ppu::PPU;
use super::timer::Timer;
use super::memory::Memory;
use super::memory_map;

pub struct System {
    cpu: CPU,
    ppu: PPU,
    timer: Timer,
    memory: Memory,
}

impl System {

    pub fn new(file_path: String) -> System {
        let cpu = CPU::new();
        let ppu = PPU::new();
        let timer = Timer::new();
        let memory = Memory::new(file_path.to_owned());

        System {
            cpu: cpu,
            ppu: ppu,
            timer: timer,
            memory: memory,
        }
    }

    pub fn boot(&mut self) {
        self.cpu.boot();
    }

    pub fn cycle(&mut self) -> usize {
        let cpu_cycles: usize = self.cpu.cycle(&mut self.memory);
        self.timer.updater(&mut self.memory, cpu_cycles);
        self.ppu.cycle(&mut self.memory, cpu_cycles);
        cpu_cycles
    }

    pub fn video_buffer(&mut self) -> Vec<u8> {
        let buffer = self.ppu.video.frame_buffer.clone();
        buffer
    }

    pub fn video_mode(&mut self) -> u8 {
        (self.memory.fetch(memory_map::LCDC) & 0x03) & 0xFF
    }

    pub fn has_stopped(&mut self) -> bool {
        self.cpu.stopped
    }

}
