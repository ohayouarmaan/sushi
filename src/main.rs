use std::fs;
use anyhow::{Context, Result};
use ir::IRGenerator;
use lexer::Lexer;
use argh::FromArgs;
use parser::Parser;

#[derive(FromArgs)]
/// Run sushi
pub struct Options {

    #[argh(option)]
    /// file path(input)
    input: String,

    #[argh(option, short = 'd')]
    /// path to dump the ir
    dump_ir: Option<String>
}

fn main() -> Result<()> {
    let opts: Options = argh::from_env();
    let source_code = fs::read_to_string(opts.input).context("File probably does not exist")?;
    let mut l = Lexer::new(source_code);
    l.lex()?;
    let mut p = Parser::new(l.tokens);
    match p.parse() {
        Ok(()) => {
            let mut ir = IRGenerator::new(p.statements.expect("UNREACHABLE"));
            ir.generate()?;
            let mut insts = String::new();
            for instr in ir.instructions.iter().clone() {
                println!("{instr}");
                insts.push_str(&instr.to_string());
                insts.push('\n');
            }

            let mut c = compiler::Compiler::new(ir.instructions, ir.functions);
            let _ = c.compile();
            let assembly = c.build();
            let _ = fs::write("./examples/test.s", assembly);
            if let Some(t) = opts.dump_ir { fs::write(t, insts)? };
            
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    Ok(())
}
