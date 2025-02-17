use std::env;
use std::fs::File;
use std::io::prelude::*;

mod interpreter;

use interpreter::{eval, Expr};

/// Compile a source program into a string of x86-64 assembly
fn compile(program: String) -> String {
    let num = program.trim().parse::<i32>().unwrap();
    return format!("mov rax, {}", num);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let result = compile(in_contents);

    let asm_program = format!(
        "
        section .text
        global our_code_starts_here
        our_code_starts_here:
            {}
            ret
        ",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    // Run interpreter test
    let expr = Expr::Add1(Box::new(Expr::Sub1(Box::new(Expr::Num(5)))));
    let interpreter_result = eval(&expr);
    println!("Interpreter Result: {}", interpreter_result);

    Ok(())
}
