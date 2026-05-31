use std::fs;
use anyhow::{Context, Result};
use lexer::Lexer;
use parser::Parser;

fn main() -> Result<()> {
    let source_code = fs::read_to_string("./examples/test.su").context("File probably does not exist")?;
    let mut l = Lexer::new(source_code);
    l.lex()?;
    let mut p = Parser::new(l.tokens);
    match p.parse() {
        Ok(_) => {
            println!("Parser: {:?}", p.statements);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    Ok(())
}
