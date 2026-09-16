use std::io;
use libc::{c_void, PROT_EXEC, PROT_WRITE, MAP_ANONYMOUS, MAP_PRIVATE};

use crate::ops::{Op, OpType};

pub const TAPE_SIZE: usize = 10_000;
const TAPE_MASK: u32 = (TAPE_SIZE - 1) as u32;

// Register convention for tape:
//   r12 = tape base ptr
//   r13 = head (for indexing)
// [r12 + r13]

fn emit_inc(code: &mut Vec<u8>, val: u8) {
    code.extend_from_slice(&[0x43, 0x80, 0x04, 0x2c, val]);
}

fn emit_dec(code: &mut Vec<u8>, val: u8) {
    code.extend_from_slice(&[0x43, 0x80, 0x2c, 0x2c, val]);
}

pub struct Jit {
    code: *mut u8,
    len: usize,
}

impl Jit {
    pub fn compile(ops: &[Op]) -> io::Result<Self> {
        let len = 4096;
        let mut code: Vec<u8> = Vec::with_capacity(len);
        // TODO: add addr, base

        let addr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                len,
                PROT_EXEC | PROT_WRITE,
                MAP_ANONYMOUS | MAP_PRIVATE,
                -1,
                0
            );
        };

        for (i, op) in ops.iter().enumerate() {
            match op.op_type {
                OpType::Inc => {
                    emit_inc(&mut code, op.operand as u8);
                }
                OpType::Dec => {
                    emit_dec(&mut code, op.operand as u8);
                }
                _ => {}
            }
        }
        todo!("implement in-mem compiler");
    }
}

impl Drop for Jit {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.code as *mut c_void, self.len);
        }
    }
}
