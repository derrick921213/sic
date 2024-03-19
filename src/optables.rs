use std::collections::HashMap;
use std::ops::BitOr;

use crate::FormatDirective;
impl BitOr for Format {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u8) | (rhs as u8)
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Format {
    FMT0 = 0x00,   /* SIC Assembler Directive */
    FMT1 = 0x01,   /* Format 1 */
    FMT2 = 0x02,   /* Format 2 */
    FMT3 = 0x04,   /* Format 3 */
    FMT4 = 0x08,   /* Format 4 */
    FMT3_4 = 0x0C, /* Format 3/4 */
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum Directive {
    BYTE = 0x101,
    WORD = 0x102,
    RESB = 0x103,
    RESW = 0x104,
    BASE = 0x105,
    NOBASE = 0x106,
    START = 0x107,
    END = 0x108,
}

#[derive(Debug)]
pub struct Instruction {
    pub fmt: FormatDirective,
    pub code: u16,
}

pub struct Optab {
    instructions: HashMap<&'static str, Instruction>,
}

impl Optab {
    pub fn new() -> Self {
        let mut instructions = HashMap::new();
        instructions.insert(
            "ADD",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x18,
            },
        );
        instructions.insert(
            "ADDF",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x58,
            },
        );
        instructions.insert(
            "ADDR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0x90,
            },
        );
        instructions.insert(
            "BASE",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT0),
                code: Directive::BASE as u16,
            },
        );
        instructions.insert(
            "BYTE",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT0),
                code: Directive::BYTE as u16,
            },
        );
        instructions.insert(
            "CLEAR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0xB4,
            },
        );
        instructions.insert(
            "COMP",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x28,
            },
        );
        instructions.insert(
            "COMPF",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x88,
            },
        );
        instructions.insert(
            "COMPR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0xA0,
            },
        );
        instructions.insert(
            "DIV",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x24,
            },
        );

        instructions.insert(
            "DIVF",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x64,
            },
        );
        instructions.insert(
            "DIVR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0x9C,
            },
        );
        instructions.insert(
            "END",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT0),
                code: Directive::END as u16,
            },
        );
        instructions.insert(
            "FIX",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT1),
                code: 0xC4,
            },
        );
        instructions.insert(
            "FLOAT",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT1),
                code: 0xC0,
            },
        );
        instructions.insert(
            "HIO",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT1),
                code: 0xF4,
            },
        );
        instructions.insert(
            "J",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x3C,
            },
        );
        instructions.insert(
            "JEQ",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x30,
            },
        );
        instructions.insert(
            "JGT",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x34,
            },
        );
        instructions.insert(
            "JLT",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x38,
            },
        );
        instructions.insert(
            "JSUB",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x48,
            },
        );
        instructions.insert(
            "LDA",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x00,
            },
        );
        instructions.insert(
            "LDB",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x68,
            },
        );
        instructions.insert(
            "LDCH",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x50,
            },
        );
        instructions.insert(
            "LDF",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x70,
            },
        );
        instructions.insert(
            "LDL",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x08,
            },
        );
        instructions.insert(
            "LDS",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x6C,
            },
        );
        instructions.insert(
            "LDT",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x74,
            },
        );
        instructions.insert(
            "LDX",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x04,
            },
        );
        instructions.insert(
            "LPS",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0xD0,
            },
        );
        instructions.insert(
            "MUL",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x20,
            },
        );
        instructions.insert(
            "MULF",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x60,
            },
        );
        instructions.insert(
            "MULR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0x98,
            },
        );
        instructions.insert(
            "NOBASE",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT0),
                code: Directive::NOBASE as u16,
            },
        );
        instructions.insert(
            "NORM",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT1),
                code: 0xC8,
            },
        );
        instructions.insert(
            "OR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x44,
            },
        );
        instructions.insert(
            "RD",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0xD8,
            },
        );
        instructions.insert(
            "RESB",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT0),
                code: Directive::RESB as u16,
            },
        );
        instructions.insert(
            "RESW",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT0),
                code: Directive::RESW as u16,
            },
        );
        instructions.insert(
            "RMO",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0xAC,
            },
        );
        instructions.insert(
            "RSUB",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x4C,
            },
        );
        instructions.insert(
            "SHIFTL",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0xA4,
            },
        );
        instructions.insert(
            "SHIFTR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0xA8,
            },
        );
        instructions.insert(
            "SIO",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT1),
                code: 0xF0,
            },
        );
        instructions.insert(
            "SSK",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0xEC,
            },
        );
        instructions.insert(
            "STA",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x0C,
            },
        );
        instructions.insert(
            "START",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT0),
                code: Directive::START as u16,
            },
        );
        instructions.insert(
            "STB",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x78,
            },
        );
        instructions.insert(
            "STCH",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x54,
            },
        );
        instructions.insert(
            "STF",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x80,
            },
        );
        instructions.insert(
            "STI",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0xD4,
            },
        );
        instructions.insert(
            "STL",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x14,
            },
        );
        instructions.insert(
            "STS",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x7C,
            },
        );
        instructions.insert(
            "STSW",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0xE8,
            },
        );
        instructions.insert(
            "STT",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x84,
            },
        );
        instructions.insert(
            "STX",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x10,
            },
        );
        instructions.insert(
            "SUB",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x1C,
            },
        );
        instructions.insert(
            "SUBF",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x5C,
            },
        );
        instructions.insert(
            "SUBR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0x94,
            },
        );
        instructions.insert(
            "SVC",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0xB0,
            },
        );
        instructions.insert(
            "TD",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0xE0,
            },
        );
        instructions.insert(
            "TIO",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT1),
                code: 0xF8,
            },
        );
        instructions.insert(
            "TIX",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0x2C,
            },
        );
        instructions.insert(
            "TIXR",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT2),
                code: 0xB8,
            },
        );
        instructions.insert(
            "WD",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT3_4),
                code: 0xDC,
            },
        );
        instructions.insert(
            "WORD",
            Instruction {
                fmt: FormatDirective::Format(Format::FMT0),
                code: Directive::WORD as u16,
            },
        );

        Optab { instructions }
    }

    pub fn is_opcode(&self, op: &str) -> Option<&Instruction> {
        self.instructions.get(op.to_uppercase().as_str())
    }
}
