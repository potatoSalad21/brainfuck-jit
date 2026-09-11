use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::ops::{Op, OpType};

const TAPE_SIZE: usize = 10_000;

pub struct Interpreter {
    memory: Vec<u8>,
    ip: usize,
    head: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: vec![0; TAPE_SIZE],
            ip: 0,
            head: 0,
        }
    }

    pub fn run(&mut self, ops: &[Op]) -> io::Result<()> {
        let mut stdin = BufReader::new(io::stdin().lock());
        let mut stdout = BufWriter::new(io::stdout().lock());

        while self.ip < ops.len() {
            let op = &ops[self.ip];

            match op.op_type {
                OpType::Inc => {
                    self.memory[self.head] = self.memory[self.head].wrapping_add(op.operand as u8);
                    self.ip += 1;
                }
                OpType::Dec => {
                    self.memory[self.head] = self.memory[self.head].wrapping_sub(op.operand as u8);
                    self.ip += 1;
                }
                OpType::Right => {
                    self.head = (self.head + op.operand) % TAPE_SIZE;
                    self.ip += 1;
                }
                OpType::Left => {
                    self.head = (self.head + TAPE_SIZE - (op.operand % TAPE_SIZE)) % TAPE_SIZE;
                    self.ip += 1;
                }
                OpType::Output => {
                    for _ in 0..op.operand {
                        stdout.write_all(&[self.memory[self.head]])?;
                    }
                    self.ip += 1;
                }
                OpType::Input => {
                    stdout.flush()?;

                    let mut buf = [0u8; 1];
                    for _ in 0..op.operand {
                        if stdin.read(&mut buf)? == 0 {
                            self.memory[self.head] = 0;   // EOF
                        } else {
                            self.memory[self.head] = buf[0];
                        }
                    }
                    self.ip += 1;
                }
                OpType::JmpIfZero => {
                    if self.memory[self.head] == 0 {
                        self.ip = op.operand;
                    } else {
                        self.ip += 1;
                    }
                }
                OpType::JmpIfNonzero => {
                    if self.memory[self.head] != 0 {
                        self.ip = op.operand;
                    } else {
                        self.ip += 1;
                    }
                }
            }
        }
        stdout.flush()?;

        Ok(())
    }
}
