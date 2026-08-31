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
        matches!(
            self,
            OpType::Right | OpType::Left | OpType::Inc | OpType::Dec)
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
}

impl Iterator for Lexer {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.buf.len() {
            let ch = self.buf.as_bytes()[self.pos] as char;
            self.pos += 1;

            if let Some(op_type) = OpType::from_char(ch) {
                let mut count = 1;

                if op_type.is_repeatable() {
                    while self.pos < self.buf.len() {
                        let next_ch = self.buf.as_bytes()[self.pos] as char;
                        match OpType::from_char(next_ch) {
                            Some(next_op) if next_op == op_type => {
                                count += 1;
                                self.pos += 1;
                            }
                            Some(_) => break,
                            None => {
                                self.pos += 1;
                            }
                        }
                    }
                }

                return Some(Op {
                    op_type,
                    operand: count,
                });
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

    let lexer = Lexer::new(buf);
    let ops: Vec<Op> = lexer.collect();

    println!("parsed {} instructions", ops.len());
    for op in ops {
        println!("OpType: {:?}", op.op_type);
        println!("operand: {}", op.operand);
        println!();
    }
}
