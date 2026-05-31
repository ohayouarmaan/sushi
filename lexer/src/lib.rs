use anyhow::{Result, anyhow};

static IGNORE: &[char] = &['\n', '\t'];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Function,
    Extern,
    If,
    Int,
    LBrace,
    RBrace,
    LParen,
    RParen,
    NumberInt,
    NumberFloat,
    SemiColon,
    Dot,
    DotDotDot,
    At,
    Hash,
    Ampersand,
    AmpersandAmpersand,
    Star,
    StarStar,
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Slash,
    SlashSlash,
    Percent,
    Colon,
    Str,
    String,
    Identifier,
    BangEqual,
    Comma
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tt: TokenType,
    pub lexeme_start: usize,
    pub lexeme_end: usize,
    pub column: usize,
    pub line: usize,
    pub lexeme: String
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub source_code: String,
    pub tokens: Vec<Token>,
    pub current_position: usize,
    pub current_row: usize,
    pub current_column: usize,
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Self {
            source_code,
            tokens: Vec::new(),
            current_position: 0,
            current_row: 1,
            current_column: 0,
        }
    }

    fn build_keyword(&self, lexeme: &str) -> Option<TokenType> {
        match lexeme {
            "function" => Some(TokenType::Function),
            "extern" => Some(TokenType::Extern),
            "if" => Some(TokenType::If),
            "int" => Some(TokenType::Int),
            "str" => Some(TokenType::Str),
            _ => None
        }
    }

    fn advance(&mut self) {
        if self.current_position < self.source_code.len() - 1 {
            self.current_position += 1;
            self.current_column += 1;
        }
    }

    fn peek(&mut self) -> char {
        if self.current_position < self.source_code.len() - 1 {
            return self.source_code.as_bytes()[self.current_position + 1] as char;
        }
        panic!("Can't peek");
    }

    fn get_current_character(&self) -> char {
        self.source_code.as_bytes()[self.current_position] as char
    }

    fn generate_single_character_token(&mut self) -> Option<Token> {
        let lexeme_start = self.current_position;
        let current_character = self.get_current_character();
        self.advance();
        let lexeme_end = self.current_position;
        match current_character {
            '{' => Some(Token { lexeme_end, lexeme_start, tt: TokenType::LBrace, lexeme: "{".into(), column: self.current_column, line: self.current_row }),
            '}' => Some(Token { lexeme_end, lexeme_start, tt: TokenType::RBrace, lexeme: "}".into(), column: self.current_column, line: self.current_row }),
            '(' => Some(Token { lexeme_end, lexeme_start, tt: TokenType::LParen, lexeme: "(".into(), column: self.current_column, line: self.current_row }),
            ')' => Some(Token { lexeme_end, lexeme_start, tt: TokenType::RParen, lexeme: ")".into(), column: self.current_column, line: self.current_row }),
            '.' => Some(Token { lexeme_end, lexeme_start, tt: TokenType::Dot, lexeme: ".".into(), column: self.current_column, line: self.current_row }),
            ';' => Some(Token { lexeme_end, lexeme_start, tt: TokenType::SemiColon, lexeme: ";".into(), column: self.current_column, line: self.current_row }),
            '*' => Some(Token { tt: TokenType::Star, lexeme_start, lexeme_end, lexeme: "*".into(), column: self.current_column, line: self.current_row }),
            '/' => Some(Token { tt: TokenType::Slash, lexeme_start, lexeme_end, lexeme: "/".into(), column: self.current_column, line: self.current_row }),
            '+' => Some(Token { tt: TokenType::Plus, lexeme_start, lexeme_end, lexeme: "+".into(), column: self.current_column, line: self.current_row }),
            '-' => Some(Token { tt: TokenType::Minus, lexeme_start, lexeme_end, lexeme: "-".into(), column: self.current_column, line: self.current_row }),
            '&' => Some(Token { tt: TokenType::Ampersand, lexeme_start, lexeme_end, lexeme: "&".into(), column: self.current_column, line: self.current_row }),
            ':' => Some(Token { tt: TokenType::Colon, lexeme_start, lexeme_end, lexeme: ":".into(), column: self.current_column, line: self.current_row }),
            '%' => Some(Token { tt: TokenType::Percent, lexeme_start, lexeme_end, lexeme: "%".into(), column: self.current_column, line: self.current_row }),
            ',' => Some(Token { tt: TokenType::Comma, lexeme_start, lexeme_end, lexeme: ",".into(), column: self.current_column, line: self.current_row }),
            c => {
                self.current_position -= 1;
                dbg!(c);
                unreachable!("INVALID PUNCTUATION");
            }
        }
    }

    fn generate_numbers(&mut self) -> Result<Token> {
        let mut num_start = String::new();
        let lexeme_start = self.current_position;
        let mut dot_count = 0;
        while ['_', '.'].contains(&self.get_current_character()) || self.get_current_character().is_ascii_digit() {
            if self.get_current_character() != '_' {
                num_start.push(self.get_current_character());
            }
            if self.get_current_character() == '.'{
                dot_count += 1;
                if dot_count > 1 {
                    return Err(anyhow!("Multiple dots. {:?}:{:?}", self.current_row, self.current_column));
                }
            }
            self.advance();
        }

        let lexeme_end = self.current_position;

        if dot_count == 0 {
            Ok(Token { tt: TokenType::NumberInt, lexeme_start, lexeme_end, lexeme: num_start, column: self.current_column, line: self.current_row })
        } else {
            Ok(Token { tt: TokenType::NumberFloat, lexeme_start, lexeme_end, lexeme: num_start, column: self.current_column, line: self.current_row })
        }
    }

    fn generate_string(&mut self) -> Result<Token> {
        let lexeme_start = self.current_position;
        let string_starter = self.get_current_character();
        let mut string_value = String::new();
        self.advance();
        while self.get_current_character() != string_starter {
            if ['\n'].contains(&self.get_current_character()) {
                return Err(anyhow!("String ended with no termination {}:{}", self.current_row, self.current_position));
            };
            if self.get_current_character() == '\\' {
                self.advance();
            }
            string_value.push(self.get_current_character());
            self.advance();
        }
        let lexeme_end = self.current_position;
        self.advance();
        Ok(Token { tt: TokenType::String, lexeme_start, lexeme_end, lexeme: string_value, column: self.current_column, line: self.current_row })
    }

    fn generate_character_token(&mut self) -> Option<Token> {
        let lexeme_start = self.current_position;
        let current_character = self.get_current_character();
        if self.peek() == current_character {
            self.advance();
            let lexeme_end = self.current_position;
            self.advance();
            match current_character {
                '*' => Some(Token { tt: TokenType::StarStar, lexeme_start, lexeme_end, lexeme: "**".into(), column: self.current_column, line: self.current_row }),
                '/' => Some(Token { tt: TokenType::SlashSlash, lexeme_start, lexeme_end, lexeme: "//".into(), column: self.current_column, line: self.current_row }),
                '+' => Some(Token { tt: TokenType::PlusPlus, lexeme_start, lexeme_end, lexeme: "++".into(), column: self.current_column, line: self.current_row }),
                '-' => Some(Token { tt: TokenType::MinusMinus, lexeme_start, lexeme_end, lexeme: "--".into(), column: self.current_column, line: self.current_row }),
                '&' => Some(Token { tt: TokenType::AmpersandAmpersand, lexeme_start, lexeme_end, lexeme: "&&".into(), column: self.current_column, line: self.current_row }),
                '.' => {
                    if self.get_current_character() == current_character {
                        self.advance();
                        Some(Token { tt: TokenType::DotDotDot, lexeme_start, lexeme_end, lexeme: "...".into(), column: self.current_column, line: self.current_row })
                    } else {
                        None
                    }
                }
                _ => unreachable!("Invalid Character")
            }
        } else {
            if current_character == '!' && self.peek() == '=' {
                self.advance();
                let lexeme_end = self.current_position;
                self.advance();
                return Some(Token { tt: TokenType::BangEqual, lexeme_start, lexeme_end, lexeme: "!=".into(), column: self.current_column, line: self.current_row })
            }
            self.generate_single_character_token()
        }
    }


    pub fn lex(&mut self) -> Result<()> {
        let source_code = self.source_code.clone();
        let src = source_code.as_bytes();
        while self.current_position < (self.source_code.len() - 1) {
            let mut current_char = self.get_current_character();
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
                            column: self.current_column,
                            line: self.current_row
                        });
                    } else {
                        self.tokens.push(Token { tt: TokenType::Identifier, lexeme_start, lexeme_end: self.current_position, lexeme: word, column: self.current_column, line: self.current_row });
                    }
                }

                c if c.is_ascii_digit() => {
                    match self.generate_numbers() {
                        Ok(n) => self.tokens.push(n),
                        Err(e) => return Err(e)
                    }
                }

                c if ['{', '}', '(', ')', ';', '.', '*', '/', '+', '-', '&', '%', '#', '@', ':', ','].contains(&c) => {
                    if let Some(t) = self.generate_character_token() {
                        self.tokens.push(t);
                    }
                }

                c if ['"', '\''].contains(&c) => {
                    match self.generate_string() {
                        Ok(t) => self.tokens.push(t),
                        Err(e) => return Err(e)
                    }
                }

                c if IGNORE.contains(&c) => {
                    if c == '\n' {
                        self.current_row += 1;
                        self.current_column = 1;
                    }
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
