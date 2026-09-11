use std::env;
use std::fs;
use std::process;
use std::io::{self, BufReader, BufWriter, Read, Write};

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

fn main() -> io::Result<()> {
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
    let mut stack: Vec<usize> = Vec::new();
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
                let addr = ops.len();
                let op = Op {
                    op_type,
                    operand: 0,
                };
                ops.push(op);
                stack.push(addr);

                ch = lexer.next();
            } else if op_type == OpType::JmpIfNonzero {
                if let Some(addr) = stack.pop() {
                    ops.push(Op {
                        op_type,
                        operand: addr + 1,
                    });
                    ops[addr].operand = ops.len();
                } else {
                    eprintln!("[Error] Unbalanced brackets");
                    process::exit(1);
                }

                ch = lexer.next();
            }
        } else {
            ch = lexer.next();
        }
    }

    const TAPE_SIZE: usize = 10_000;
    let mut memory: Vec<u8> = vec![0; TAPE_SIZE];   // circular tape
    let mut ip: usize = 0;
    let mut head: usize = 0;
    let mut stdin = BufReader::new(io::stdin().lock());
    let mut stdout = BufWriter::new(io::stdout().lock());

    while ip < ops.len() {
        let op = &ops[ip];

        match op.op_type {
            OpType::Inc => {
                memory[head] = memory[head].wrapping_add(op.operand as u8);
                ip += 1;
            }
            OpType::Dec => {
                memory[head] = memory[head].wrapping_sub(op.operand as u8);
                ip += 1;
            }
            OpType::Right => {
                head = (head + op.operand) % TAPE_SIZE;
                ip += 1;
            }
            OpType::Left => {
                head = (head + TAPE_SIZE - (op.operand % TAPE_SIZE)) % TAPE_SIZE;
                ip += 1;
            }
            OpType::Output => {
                for _ in 0..op.operand {
                    stdout.write_all(&[memory[head]])?;
                }
                ip += 1;
            }
            OpType::Input => {
                stdout.flush()?;

                let mut buf = [0u8; 1];
                for _ in 0..op.operand {
                    if stdin.read(&mut buf)? == 0 {
                        memory[head] = 0;   // EOF
                    } else {
                        memory[head] = buf[0];
                    }
                }
                ip += 1;
            }
            OpType::JmpIfZero => {
                if memory[head] == 0 {
                    ip = op.operand;
                } else {
                    ip += 1;
                }
            }
            OpType::JmpIfNonzero => {
                if memory[head] != 0 {
                    ip = op.operand;
                } else {
                    ip += 1;
                }
            }
        }
    }
    stdout.flush()?;

    Ok(())
}
