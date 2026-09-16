use std::io;
use libc::{c_void, PROT_EXEC, PROT_WRITE, MAP_ANONYMOUS, MAP_PRIVATE, MAP_FAILED};

use crate::ops::{Op, OpType};

pub const TAPE_SIZE: usize = 10_000;
const TAPE_MASK: u32 = (TAPE_SIZE - 1) as u32;

// Register convention for tape:
//   r12 = tape base ptr
//   r13 = head (for indexing)
// [r12 + r13]

const INIT: [u8; 14] = [
    0x41, 0x54,             // push r12
    0x41, 0x55,             // push r13
    0x48, 0x83, 0xec, 0x08, // sub rsp, 8
    0x49, 0x89, 0xfc,       // mov r12, rdi
    0x4d,  0x31, 0xed,      // xor r13, r13
];

const CLEANUP: [u8; 9] = [
    0x48, 0x83, 0xc4, 0x08, // add rsp, 8
    0x41, 0x5d,             // pop r13
    0x41, 0x5c,             // pop r12
    0xc3,                   // ret
];

fn op_size(op: &Op) -> usize {
    match op.op_type {
        OpType::Inc | OpType::Dec => 5,
        OpType::Right | OpType::Left => 14,
        OpType::Output => 10 * op.operand,
        OpType::Input => 17 * op.operand,
        OpType::JmpIfZero | OpType::JmpIfNonzero => 11,
    }
}

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
        // TODO: calculate hex length properly
        let len = 4096;
        let mut code: Vec<u8> = Vec::with_capacity(len);
        code.extend_from_slice(&INIT);
        // TODO: add addr, base

        let addr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                len,
                PROT_EXEC | PROT_WRITE,
                MAP_ANONYMOUS | MAP_PRIVATE,
                -1,
                0
            )
        };

        if addr == MAP_FAILED {
            return Err(io::Error::last_os_error());
        }

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

        code.extend_from_slice(&CLEANUP);

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
