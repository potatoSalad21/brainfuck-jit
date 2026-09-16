mod ops;
mod lexer;
mod parser;
mod interpreter;
mod compiler;

use std::env;
use std::fs;
use std::process;

use interpreter::Interpreter;

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

    let mut compiler = Compiler::new();
    if let Err(err) = compiler.run(&ops) {
        eprintln!("[Error] {err}");
        process::exit(1);
    }
}
