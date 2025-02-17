mod compiler;
mod interpreter;

use compiler::compile_expr;
use interpreter::parse_expr;
use sexp::parse;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input-file> <output-file>", args[0]);
        std::process::exit(1);
    }

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    // Run interpreter test
    let expr = parse_expr(&parse(&in_contents).unwrap());
    let result = compile_expr(&expr);

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

    Ok(())
}
