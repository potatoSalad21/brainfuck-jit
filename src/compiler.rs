use std::io;
use libc::{c_void};

use crate::ops::{Op, OpType};

pub const TAPE_SIZE: usize = 10_000;
const TAPE_MASK: u32 = (TAPE_SIZE - 1) as u32;

// Register convention for tape:
//   r12 = tape base ptr
//   r13 = head (for indexing)
// [r12 + r13]

pub struct Jit {
    code: *mut u8,
    len: usize,
}

impl Jit {
    pub fn compile(ops: &[Op]) -> io::Result<Self> {
        todo!("implement in-memory compiler");
    }
}

