use std::fs;
use anyhow::{Context, Result};
use lexer::Lexer;

fn main() -> Result<()> {
    let source_code = fs::read_to_string("./examples/test.su").context("File probably does not exist")?;
    let mut l = Lexer::new(source_code);
    l.lex()?;
    println!("{:?}", l.tokens);
    Ok(())
}
