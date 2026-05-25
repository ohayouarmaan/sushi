use anyhow::{Result};

static IGNORE: &[char] = &['\n', '\t'];

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Function,
    Extern,
    If,
    Int,
    LBrace,
    RBrace,
    LParen,
    RParen,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tt: TokenType,
    pub lexeme_start: usize,
    pub lexeme_end: usize,
    pub lexeme: String
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub source_code: String,
    pub tokens: Vec<Token>,
    pub current_position: usize,
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Self {
            source_code,
            tokens: Vec::new(),
            current_position: 0
        }
    }

    fn build_keyword(&self, lexeme: &str) -> Option<TokenType> {
        match lexeme {
            "function" => Some(TokenType::Function),
            "extern" => Some(TokenType::Extern),
            "if" => Some(TokenType::If),
            "int" => Some(TokenType::Int),
            _ => None
        }
    }

    fn advance(&mut self) {
        if self.current_position < self.source_code.len() - 1 {
            self.current_position += 1;
        }
    }

    fn generate_punctuation_token(&mut self, current_char: &char) -> Option<Token> {
        let lexeme_start = self.current_position;
        self.advance();
        let lexeme_end = self.current_position;
        match current_char {
            '{' => Some(Token {
                lexeme_end,
                lexeme_start,
                tt: TokenType::LBrace,
                lexeme: "{".into()
            }),
            '}' => Some(Token {
                lexeme_end,
                lexeme_start,
                tt: TokenType::RBrace,
                lexeme: "}".into()
            }),
            '(' => Some(Token {
                lexeme_end,
                lexeme_start,
                tt: TokenType::LParen,
                lexeme: "}".into()
            }),
            ')' => Some(Token {
                lexeme_end,
                lexeme_start,
                tt: TokenType::RParen,
                lexeme: "}".into()
            }),
            _ => {
                self.current_position -= 1;
                unreachable!("INVALID PUNCTUATION");
            }
        }
    }

    pub fn lex(&mut self) -> Result<()> {
        let source_code = self.source_code.clone();
        let src = source_code.as_bytes();
        while self.current_position < (self.source_code.len() - 1) {
            let mut current_char = src[self.current_position] as char;
            match current_char {
                c if c.is_ascii_alphabetic() => {
                    let mut word = String::new();
                    let lexeme_start = self.current_position;
                    while current_char.is_alphanumeric() {
                        word.push(current_char);
                        self.advance();
                        if self.current_position >= src.len() {
                            break;
                        }
                        current_char = src[self.current_position] as char;
                    }

                    if let Some(tt) = self.build_keyword(&word) {
                        self.tokens.push(Token {
                            lexeme: word,
                            lexeme_end: self.current_position,
                            lexeme_start,
                            tt,
                        });
                    }
                }

                c if ['{', '}', '(', ')'].contains(&c) => {
                    if let Some(t) = self.generate_punctuation_token(&c) {
                        self.tokens.push(t);
                        self.advance();
                    }
                }

                c if IGNORE.contains(&c) => {
                    self.advance();
                    continue;
                }

                _ => {
                    self.advance();
                }
            }
        }
        Ok(())
    }
}
