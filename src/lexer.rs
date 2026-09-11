use crate::ops::OpType;

pub struct Lexer {
    buf: String,
    pos: usize
}

impl Lexer {
    pub fn new(buf: String) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn next(&mut self) -> Option<char> {
        while self.pos < self.buf.len() {
            let ch = self.buf.as_bytes()[self.pos] as char;
            self.pos += 1;
            if OpType::from_char(ch).is_some() {
                return Some(ch);
            }
        }
        None
    }
}

