use crate::{optables::Format, Directive};
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AddrMode {
    Simple = 0x01,
    Immediate = 0x02,
    Indirect = 0x04,
    Index = 0x08,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FormatDirective {
    Format(Format),
    Directive(Directive),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Line {
    memory: u32,
    symbol: Option<String>,
    op: String,
    operand1: Option<String>,
    operand2: Option<String>,
    code: u16,
    fmt: FormatDirective,
    address_mode: AddrMode,
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
impl Line {
    pub fn new(
        memory: u32,
        symbol: Option<String>,
        op: String,
        operand1: Option<String>,
        operand2: Option<String>,
        code: u16,
        fmt: FormatDirective,
        address_mode: AddrMode,
    ) -> Self {
        Line {
            memory,
            symbol,
            op,
            operand1,
            operand2,
            code,
            fmt: fmt.clone(),
            address_mode,
        }
    }
    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = Some(symbol);
    }
    pub fn set_op(&mut self, op: String) {
        self.op = op;
    }
    pub fn set_operand1(&mut self, operand1: String) {
        self.operand1 = Some(operand1);
    }
    pub fn set_operand2(&mut self, operand2: String) {
        self.operand2 = Some(operand2);
    }
    pub fn set_code(&mut self, code: u16) {
        self.code = code;
    }
    pub fn set_fmt(&mut self, fmt: FormatDirective) {
        self.fmt = fmt;
    }
    pub fn set_address_mode(&mut self, address_mode: AddrMode) {
        self.address_mode = address_mode;
    }
    pub fn get_symbol(&self) -> Option<&String> {
        self.symbol.as_ref()
    }
    pub fn get_memory(&self) -> u32 {
        self.memory
    }
    pub fn get_op(&self) -> &String {
        &self.op
    }
    pub fn get_operand1(&self) -> Option<&String> {
        self.operand1.as_ref()
    }
    pub fn get_operand2(&self) -> Option<&String> {
        self.operand2.as_ref()
    }
    pub fn get_code(&self) -> u16 {
        self.code
    }
    pub fn get_fmt(&self) -> &FormatDirective {
        &self.fmt
    }
    pub fn get_address_mode(&self) -> &AddrMode {
        &self.address_mode
    }
}
