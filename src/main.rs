mod ops;
mod lexer;
mod parser;
mod interpreter;
mod compiler;

use std::env;
use std::fs;
use std::process;

use interpreter::Interpreter;
use compiler::{Jit, TAPE_SIZE};

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

    let ops = parser::parse(buf).unwrap_or_else(|err| {
        eprintln!("[Error] {err}");
        process::exit(1);
    });

    //let mut interpreter = Interpreter::new();
    //if let Err(err) = interpreter.run(&ops) {
    //    eprintln!("[Error] {err}");
    //    process::exit(1);
    //}

    let jit = Jit::compile(&ops).unwrap_or_else(|err| {
        eprintln!("[Error] Jit compilation failed: {err}");
        process::exit(1);
    });
    let mut tape = vec![0u8; TAPE_SIZE];
    jit.run(&mut tape);
}
