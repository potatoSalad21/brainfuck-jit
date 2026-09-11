#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpType {
    Right,
    Left,
    Inc,
    Dec,
    Output,
    Input,
    JmpIfZero,
    JmpIfNonzero,
}

impl OpType {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '>' => Some(OpType::Right),
            '<' => Some(OpType::Left),
            '+' => Some(OpType::Inc),
            '-' => Some(OpType::Dec),
            '.' => Some(OpType::Output),
            ',' => Some(OpType::Input),
            '[' => Some(OpType::JmpIfZero),
            ']' => Some(OpType::JmpIfNonzero),
            _ => None,
        }
    }

    pub fn is_repeatable(&self) -> bool {
        !matches!(
            self,
            OpType::JmpIfZero | OpType::JmpIfNonzero)
    }
}

#[derive(Debug)]
pub struct Op {
    op_type: OpType,
    operand: usize
}

