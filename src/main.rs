use std::env;
use std::fs;
use std::process;

#[derive(Debug, PartialEq)]
enum OpType {
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
    fn from_char(ch: char) -> Option<Self> {
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

    fn is_repeatable(&self) -> bool {
        !matches!(
            self,
            OpType::JmpIfZero | OpType::JmpIfNonzero)
    }
}

struct Op {
    op_type: OpType,
    operand: usize
}

struct Lexer {
    buf: String,
    pos: usize
}

impl Lexer {
    fn new(buf: String) -> Self {
        Self { buf, pos: 0 }
    }

    fn next(&mut self) -> Option<char> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file.bf>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let buf = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading '{file_path}': {err}");
        process::exit(1);
    });

    let mut lexer = Lexer::new(buf);
    let mut ops: Vec<Op> = Vec::new();

    let mut ch = lexer.next();
    while let Some(curr) = ch {
        if let Some(op_type) = OpType::from_char(curr) {
            if op_type.is_repeatable() {
                let mut count: usize = 1;
                let mut next = lexer.next();

                while next == Some(curr) {
                    count += 1;
                    next = lexer.next();
                }

                ops.push(Op {
                    op_type,
                    operand: count,
                });
                ch = next;
            } else if op_type == OpType::JmpIfZero {
                todo!("implement jmp forward");
            } else if op_type == OpType::JmpIfNonzero {
                todo!("implement jmp backwards");
            }
        } else {
            ch = lexer.next();
        }
    }

    for i in 0..ops.len() {
        let op = ops.get(i).unwrap();
        println!("{}: {:?} ({})", i, op.op_type, op.operand);
    }
}
