mod opcode;
use opcode::Opcode;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
const PC_START_VALUE: usize = 512; // byte 0x200
const FONTSET_START_VALUE: usize = 80; // byte 0x050


#[derive(Debug)]
pub struct CPU {
    // memory
    memory: [u8; 4096],
    pc: usize,
    
    // registers
    opcode: u16,
    V: [u8; 16],
    I: u16,

    // io
    gfx: [bool; 64* 32],
    sound_timer: u8,
    key: [u8; 16],

    // stack
    stack: Vec<u16>,
    sp: u8,

    delay_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            opcode: 0,
            memory: [0; 4096],
            pc: PC_START_VALUE, // Les programmes CHIP-8 commencent à l'adresse 0x200
            V: [0; 16],
            I: 0,
            gfx: [false; 64 * 32],
            sound_timer: 0,
            key: [0; 16],
            stack: Vec::new(),
            sp: 0,
            delay_timer: 0,
        };
        cpu.load_fontset();
        cpu
    }
    pub fn load_program(&mut self, program: &[u8]) -> Result<(), String> {
        let program_adr = PC_START_VALUE;
        if program_adr + program.len() > self.memory.len() {
            return Err(format!("Program too long; max length is {}", self.memory.len() - program_adr));
        }
        self.memory[program_adr..program_adr + program.len()].copy_from_slice(program);
        Ok(())
    }

    fn load_fontset(&mut self) {
        let font_adr = FONTSET_START_VALUE;
        self.memory[font_adr..font_adr + FONTSET.len()].copy_from_slice(&FONTSET);
    }
    fn get_opcode(&mut self) -> u16 {
        ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16)
    }
    fn execute_opcode(&mut self, opcode: Opcode) {
        match opcode {
            _ => todo!()
        }
    }
    pub fn emulate_cycle(&mut self) {
        self.opcode = self.get_opcode();
        let decoded_opcode = Opcode::decode(self.opcode);
        self.execute_opcode(decoded_opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            todo!("play sound");
            self.sound_timer -= 1;
        }
    }
}