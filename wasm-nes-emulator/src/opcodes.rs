use crate::cpu::AddressingMode;
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub name: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub address_mode: AddressingMode
}

impl OpCode {
    pub fn new(code: u8, name: &'static str, bytes: u8, cycles: u8, address_mode: AddressingMode) -> Self {
        OpCode {code: code, name: name, bytes: bytes, cycles: cycles, address_mode: address_mode}
    }
}


lazy_static! {
pub static ref CPU_OPS_CODES: Vec<OpCode> = vec![
OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),

OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
OpCode::new(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),
OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPage_Y),
OpCode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),

OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_Y),
OpCode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),

OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
OpCode::new(0xBD, "LDA", 3, 4, AddressingMode::Absolute_X),
OpCode::new(0xB9, "LDA", 3, 4, AddressingMode::Absolute_Y),
OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X),
OpCode::new(0xB1, "LDA", 2, 5, AddressingMode::Indirect_Y),

OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
OpCode::new(0xBE, "LDX", 3, 4, AddressingMode::Absolute_Y),

OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
OpCode::new(0xBC, "LDY", 3, 4, AddressingMode::Absolute_X),

OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::NoneAddressing),
OpCode::new(0xCA, "DEX", 1, 2, AddressingMode::NoneAddressing),
OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),
OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::NoneAddressing),
OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing),
OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),
OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing),

OpCode::new(0xEA, "NOP", 1, 2, AddressingMode::NoneAddressing),

OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
OpCode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
OpCode::new(0x3D, "AND", 3, 4, AddressingMode::Absolute_X),
OpCode::new(0x39, "AND", 3, 4, AddressingMode::Absolute_Y),
OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
OpCode::new(0x31, "AND", 2, 5, AddressingMode::Indirect_Y),

OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X),
OpCode::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute),
OpCode::new(0x5D, "EOR", 3, 4, AddressingMode::Absolute_X),
OpCode::new(0x59, "EOR", 3, 4, AddressingMode::Indirect_X),
OpCode::new(0x51, "EOR", 2, 5, AddressingMode::Indirect_Y),

OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPage_X),
OpCode::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute),
OpCode::new(0x1D, "ORA", 3, 4, AddressingMode::Absolute_X),
OpCode::new(0x19, "ORA", 3, 4, AddressingMode::Absolute_Y),
OpCode::new(0x01, "ORA", 2, 6, AddressingMode::Indirect_X),
OpCode::new(0x11, "ORA", 2, 5, AddressingMode::Indirect_Y),

OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
OpCode::new(0x7D, "ADC", 3, 4, AddressingMode::Absolute_X),
OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),
OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),

OpCode::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate),
OpCode::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
OpCode::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPage_X),
OpCode::new(0xED, "SBC", 3, 4, AddressingMode::Absolute),
OpCode::new(0xFD, "SBC", 3, 4, AddressingMode::Absolute_X),
OpCode::new(0xF9, "SBC", 3, 4, AddressingMode::Absolute_Y),
OpCode::new(0xE1, "SBC", 2, 6, AddressingMode::Indirect_X),
OpCode::new(0xF1, "SBC", 2, 5, AddressingMode::Indirect_Y),
];

// BNE CPM CPY CPX

pub static ref OPSCODES_MAP: HashMap<u8, &'static OpCode> = {
    let mut map = HashMap::new();
    for op in &*CPU_OPS_CODES {
        map.insert(op.code, op);
    }
    map
};
}