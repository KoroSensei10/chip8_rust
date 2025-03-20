mod cpu;

use cpu::CPU;

fn main() -> Result<(), String> {
    let mut cpu = CPU::new();

    cpu.load_program(&[0x00, 0xE0])?;

    loop {
        cpu.emulate_cycle();
    }
}
