#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    CallMachineRoutine { addr: u16 },      // 0NNN
    ClearScreen,                           // 00E0
    Return,                                // 00EE
    Jump { addr: u16 },                    // 1NNN
    CallSubroutine { addr: u16 },          // 2NNN
    SkipIfEqual { x: u8, nn: u8 },         // 3XNN
    SkipIfNotEqual { x: u8, nn: u8 },      // 4XNN
    SkipIfRegistersEqual { x: u8, y: u8 }, // 5XY0
    SetRegister { x: u8, nn: u8 },         // 6XNN
    AddToRegister { x: u8, nn: u8 },       // 7XNN
    AssignRegister { x: u8, y: u8 },       // 8XY0
    Or { x: u8, y: u8 },                   // 8XY1
    And { x: u8, y: u8 },                  // 8XY2
    Xor { x: u8, y: u8 },                  // 8XY3
    AddRegisters { x: u8, y: u8 },         // 8XY4
    SubtractXY { x: u8, y: u8 },           // 8XY5
    ShiftRight { x: u8, y: u8 },           // 8XY6
    SubtractYX { x: u8, y: u8 },           // 8XY7
    ShiftLeft { x: u8, y: u8 },            // 8XYE
    SkipIfRegistersNotEqual { x: u8, y: u8 }, // 9XY0
    SetIndex { addr: u16 },                // ANNN
    JumpWithOffset { addr: u16 },          // BNNN
    Random { x: u8, nn: u8 },              // CXNN
    Draw { x: u8, y: u8, height: u8 },     // DXYN
    SkipIfKeyPressed { x: u8 },            // EX9E
    SkipIfKeyNotPressed { x: u8 },         // EXA1
    GetDelayTimer { x: u8 },               // FX07
    WaitForKey { x: u8 },                  // FX0A
    SetDelayTimer { x: u8 },               // FX15
    SetSoundTimer { x: u8 },               // FX18
    AddToIndex { x: u8 },                  // FX1E
    SetIndexToSprite { x: u8 },            // FX29
    StoreBCD { x: u8 },                    // FX33
    StoreRegisters { x: u8 },              // FX55
    LoadRegisters { x: u8 },               // FX65
}

impl Opcode {
    pub fn decode(opcode: u16) -> Self {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8; // also named height
        let nn = (opcode & 0x00FF) as u8;
        let addr = opcode & 0x0FFF; // also named NNN
        
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Opcode::ClearScreen,
                0x00EE => Opcode::Return,
                _ => Opcode::CallMachineRoutine { addr },
            },
            0x1000 => Opcode::Jump { addr },
            0x2000 => Opcode::CallSubroutine { addr },
            0x3000 => Opcode::SkipIfEqual { x, nn },
            0x4000 => Opcode::SkipIfNotEqual { x, nn },
            0x5000 => Opcode::SkipIfRegistersEqual { x, y },
            0x6000 => Opcode::SetRegister { x, nn },
            0x7000 => Opcode::AddToRegister { x, nn },
            0x8000 => match opcode & 0x000F {
                0x0000 => Opcode::AssignRegister { x, y },
                0x0001 => Opcode::Or { x, y },
                0x0002 => Opcode::And { x, y },
                0x0003 => Opcode::Xor { x, y },
                0x0004 => Opcode::AddRegisters { x, y },
                0x0005 => Opcode::SubtractXY { x, y },
                0x0006 => Opcode::ShiftRight { x, y },
                0x0007 => Opcode::SubtractYX { x, y },
                0x000E => Opcode::ShiftLeft { x, y },
                _ => panic!("Opcode inconnu : {:#X}", opcode)
            },
            0x9000 => Opcode::SkipIfRegistersNotEqual { x, y },
            0xA000 => Opcode::SetIndex { addr },
            0xB000 => Opcode::JumpWithOffset { addr },
            0xC000 => Opcode::Random { x, nn },
            0xD000 => Opcode::Draw { x, y, height: n },
            0xE000 => match opcode & 0x00FF {
                0x009E => Opcode::SkipIfKeyPressed { x },
                0x00A1 => Opcode::SkipIfKeyNotPressed { x },
                _ => panic!("Opcode inconnu : {:#X}", opcode)
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => Opcode::GetDelayTimer { x },
                0x000A => Opcode::WaitForKey { x },
                0x0015 => Opcode::SetDelayTimer { x },
                0x0018 => Opcode::SetSoundTimer { x },
                0x001E => Opcode::AddToIndex { x },
                0x0029 => Opcode::SetIndexToSprite { x },
                0x0033 => Opcode::StoreBCD { x },
                0x0055 => Opcode::StoreRegisters { x },
                0x0065 => Opcode::LoadRegisters { x },
                _ => panic!("Opcode inconnu : {:#X}", opcode)
            },
            _ => panic!("Opcode inconnu : {:#X}", opcode)
        }
    }
}