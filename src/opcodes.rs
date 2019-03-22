#[derive(Copy, Clone, Debug)]
pub enum AddressingModes {
    A,
    IMPL,
    IMM,
    ZPG,
    ZPGX,
    ZPGY,
    IND,
    INDX,
    INDY,
    ABS,
    ABSX,
    ABSY,
    REL,
}
#[derive(Copy, Clone, Debug)]
pub struct Opcode {
    pub name: &'static str,
    pub code: u8,
    pub mode: AddressingModes,
    pub size: u16,
    pub time: u8,
}
pub fn find<F: Fn(&Opcode) -> bool>(f: F) -> Option<Opcode> {
    for opcode in OPCODES.iter() {
        if f(opcode) {
            return Some(opcode.clone());
        }
    }
    return None;
}
//#region OPCODES
static OPCODES: [Opcode; 151] = [
    Opcode {
        name: "ADC",
        code: 0x61,
        mode: AddressingModes::INDX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "ADC",
        code: 0x71,
        mode: AddressingModes::INDY,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "ADC",
        code: 0x65,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "ADC",
        code: 0x75,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "ADC",
        code: 0x69,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "ADC",
        code: 0x79,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "ADC",
        code: 0x6D,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "ADC",
        code: 0x7D,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "AND",
        code: 0x21,
        mode: AddressingModes::INDX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "AND",
        code: 0x31,
        mode: AddressingModes::INDY,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "AND",
        code: 0x25,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "AND",
        code: 0x35,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "AND",
        code: 0x29,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "AND",
        code: 0x39,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "AND",
        code: 0x2D,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "AND",
        code: 0x3D,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "ASL",
        code: 0x06,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "ASL",
        code: 0x16,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "ASL",
        code: 0x0A,
        mode: AddressingModes::A,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "ASL",
        code: 0x0E,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 6u8,
    },
    Opcode {
        name: "ASL",
        code: 0x1E,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 7u8,
    },
    Opcode {
        name: "BCC",
        code: 0x90,
        mode: AddressingModes::REL,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "BCS",
        code: 0xB0,
        mode: AddressingModes::REL,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "BEQ",
        code: 0xF0,
        mode: AddressingModes::REL,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "BIT",
        code: 0x24,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "BIT",
        code: 0x2C,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "BMI",
        code: 0x30,
        mode: AddressingModes::REL,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "BNE",
        code: 0xD0,
        mode: AddressingModes::REL,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "BPL",
        code: 0x10,
        mode: AddressingModes::REL,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "BRK",
        code: 0x00,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 7u8,
    },
    Opcode {
        name: "BVC",
        code: 0x50,
        mode: AddressingModes::REL,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "BVS",
        code: 0x70,
        mode: AddressingModes::REL,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "CLC",
        code: 0x18,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "CLD",
        code: 0xD8,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "CLI",
        code: 0x58,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "CLV",
        code: 0xB8,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "CMP",
        code: 0xC1,
        mode: AddressingModes::INDX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "CMP",
        code: 0xD1,
        mode: AddressingModes::INDY,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "CMP",
        code: 0xC5,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "CMP",
        code: 0xD5,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "CMP",
        code: 0xC9,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "CMP",
        code: 0xD9,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "CMP",
        code: 0xCD,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "CMP",
        code: 0xDD,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "CPX",
        code: 0xE0,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "CPX",
        code: 0xE4,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "CPX",
        code: 0xEC,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "CPY",
        code: 0xC0,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "CPY",
        code: 0xC4,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "CPY",
        code: 0xCC,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "DEC",
        code: 0xC6,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "DEC",
        code: 0xD6,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "DEC",
        code: 0xCE,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 6u8,
    },
    Opcode {
        name: "DEC",
        code: 0xDE,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 7u8,
    },
    Opcode {
        name: "DEX",
        code: 0xCA,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "DEY",
        code: 0x88,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "EOR",
        code: 0x41,
        mode: AddressingModes::INDX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "EOR",
        code: 0x51,
        mode: AddressingModes::INDY,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "EOR",
        code: 0x45,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "EOR",
        code: 0x55,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "EOR",
        code: 0x49,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "EOR",
        code: 0x59,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "EOR",
        code: 0x4D,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "EOR",
        code: 0x5D,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "INC",
        code: 0xE6,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "INC",
        code: 0xF6,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "INC",
        code: 0xEE,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 6u8,
    },
    Opcode {
        name: "INC",
        code: 0xFE,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 7u8,
    },
    Opcode {
        name: "INX",
        code: 0xE8,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "INY",
        code: 0xC8,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "JMP",
        code: 0x4C,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 3u8,
    },
    Opcode {
        name: "JMP",
        code: 0x6C,
        mode: AddressingModes::IND,
        size: 3u16,
        time: 5u8,
    },
    Opcode {
        name: "JSR",
        code: 0x20,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 6u8,
    },
    Opcode {
        name: "LDA",
        code: 0xA1,
        mode: AddressingModes::INDX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "LDA",
        code: 0xB1,
        mode: AddressingModes::INDY,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "LDA",
        code: 0xA5,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "LDA",
        code: 0xB5,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "LDA",
        code: 0xA9,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "LDA",
        code: 0xB9,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "LDA",
        code: 0xAD,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "LDA",
        code: 0xBD,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "LDX",
        code: 0xA2,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "LDX",
        code: 0xA6,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "LDX",
        code: 0xB6,
        mode: AddressingModes::ZPGY,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "LDX",
        code: 0xAE,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "LDX",
        code: 0xBE,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "LDY",
        code: 0xA0,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "LDY",
        code: 0xA4,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "LDY",
        code: 0xB4,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "LDY",
        code: 0xAC,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "LDY",
        code: 0xBC,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "LSR",
        code: 0x46,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "LSR",
        code: 0x56,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "LSR",
        code: 0x4A,
        mode: AddressingModes::A,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "LSR",
        code: 0x4E,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 6u8,
    },
    Opcode {
        name: "LSR",
        code: 0x5E,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 7u8,
    },
    Opcode {
        name: "NOP",
        code: 0xEA,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "ORA",
        code: 0x01,
        mode: AddressingModes::INDX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "ORA",
        code: 0x11,
        mode: AddressingModes::INDY,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "ORA",
        code: 0x05,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "ORA",
        code: 0x15,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "ORA",
        code: 0x09,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "ORA",
        code: 0x19,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "ORA",
        code: 0x0D,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "ORA",
        code: 0x1D,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "PHA",
        code: 0x48,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 3u8,
    },
    Opcode {
        name: "PHP",
        code: 0x08,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 3u8,
    },
    Opcode {
        name: "PLA",
        code: 0x68,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 4u8,
    },
    Opcode {
        name: "PLP",
        code: 0x28,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 4u8,
    },
    Opcode {
        name: "ROL",
        code: 0x26,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "ROL",
        code: 0x36,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "ROL",
        code: 0x2A,
        mode: AddressingModes::A,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "ROL",
        code: 0x2E,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 6u8,
    },
    Opcode {
        name: "ROL",
        code: 0x3E,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 7u8,
    },
    Opcode {
        name: "ROR",
        code: 0x66,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "ROR",
        code: 0x76,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "ROR",
        code: 0x6A,
        mode: AddressingModes::A,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "ROR",
        code: 0x6E,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 6u8,
    },
    Opcode {
        name: "ROR",
        code: 0x7E,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 7u8,
    },
    Opcode {
        name: "RTI",
        code: 0x40,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 6u8,
    },
    Opcode {
        name: "RTS",
        code: 0x60,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 6u8,
    },
    Opcode {
        name: "SBC",
        code: 0xE1,
        mode: AddressingModes::INDX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "SBC",
        code: 0xF1,
        mode: AddressingModes::INDY,
        size: 2u16,
        time: 5u8,
    },
    Opcode {
        name: "SBC",
        code: 0xE5,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "SBC",
        code: 0xF5,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "SBC",
        code: 0xE9,
        mode: AddressingModes::IMM,
        size: 2u16,
        time: 2u8,
    },
    Opcode {
        name: "SBC",
        code: 0xF9,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "SBC",
        code: 0xED,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "SBC",
        code: 0xFD,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "SEC",
        code: 0x38,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "SED",
        code: 0xF8,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "SEI",
        code: 0x78,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "STA",
        code: 0x81,
        mode: AddressingModes::INDX,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "STA",
        code: 0x91,
        mode: AddressingModes::INDY,
        size: 2u16,
        time: 6u8,
    },
    Opcode {
        name: "STA",
        code: 0x85,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "STA",
        code: 0x95,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "STA",
        code: 0x99,
        mode: AddressingModes::ABSY,
        size: 3u16,
        time: 5u8,
    },
    Opcode {
        name: "STA",
        code: 0x8D,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "STA",
        code: 0x9D,
        mode: AddressingModes::ABSX,
        size: 3u16,
        time: 5u8,
    },
    Opcode {
        name: "STX",
        code: 0x86,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "STX",
        code: 0x96,
        mode: AddressingModes::ZPGY,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "STX",
        code: 0x8E,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "STY",
        code: 0x84,
        mode: AddressingModes::ZPG,
        size: 2u16,
        time: 3u8,
    },
    Opcode {
        name: "STY",
        code: 0x94,
        mode: AddressingModes::ZPGX,
        size: 2u16,
        time: 4u8,
    },
    Opcode {
        name: "STY",
        code: 0x8C,
        mode: AddressingModes::ABS,
        size: 3u16,
        time: 4u8,
    },
    Opcode {
        name: "TAX",
        code: 0xAA,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "TAY",
        code: 0xA8,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "TSX",
        code: 0xBA,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "TXA",
        code: 0x8A,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "TXS",
        code: 0x9A,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
    Opcode {
        name: "TYA",
        code: 0x98,
        mode: AddressingModes::IMPL,
        size: 1u16,
        time: 2u8,
    },
];
//#endregion
