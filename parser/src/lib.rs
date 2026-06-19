use std::{collections::HashMap, fmt::Display};

use lexer::{Token, TokenType};
use anyhow::{Result, anyhow};


#[derive(Debug, Clone)]
pub enum Literal {
    NumberInt(u64),
    Variable(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumberInt(x) => write!(f, "{}", x),
            Self::Variable(x) => write!(f, "var{{}}{}", x)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>
    },
    Literal {
        value: Literal,
        token: Token
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    PrintStatement(Expression),
    VariableDeclaration {
        name: Token,
        var_type: TokenType,
        value: Expression
    },
    FunctionDeclaration {
        name: Token,
        arguments: HashMap<String, Token>,
        return_type: Token,
        body: Vec<Box<Statement>>
    }
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current_index: usize,
    pub statements: Option<Vec<Statement>>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_index: 0,
            statements: None
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.current_index < self.tokens.len() - 1 {
            return Some(&self.tokens[self.current_index+1]);
        }
        None
    }

    fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_index += 1;
        }
    }

    fn can_move_forward(&self) -> bool {
        self.current_index < self.tokens.len()
    }

    fn consume(&mut self, tt: TokenType) -> Result<()> {
        if self.can_move_forward() {
            if self.get_current_token().tt == tt {
                self.advance();
                Ok(())
            } else {
                Err(anyhow!("Expected {:?} found {:?}", tt, self.get_current_token()))
            }
        } else {
            Err(anyhow!("Expected {:?} found EOF", tt))
        }
    }

    fn consume_and_return(&mut self, tt: TokenType) -> Result<Token> {
        if self.can_move_forward() {
            if self.get_current_token().tt == tt {
                let t = self.get_current_token().clone();
                self.advance();
                Ok(t)
            } else {
                Err(anyhow!("Expected {:?} found {:?}", tt, self.get_current_token()))
            }
        } else {
            Err(anyhow!("Expected {:?} found EOF", tt))
        }
    }

    fn get_current_token(&self) -> &Token {
        &self.tokens[self.current_index]
    }

    pub fn parse(&mut self) -> Result<()> {
        let mut statments: Vec<Statement> = Vec::new();
        while self.peek().is_some() {
            match self.parse_statement() {
                Ok(t) => statments.push(t),
                Err(e) => return Err(e)
            }
        }
        self.statements = Some(statments);
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        let current_token = self.get_current_token();
        match current_token.tt {
            TokenType::Print => {
                self.advance();
                let exp = self.parse_expression()?;
                self.consume(TokenType::SemiColon)?;
                Ok(Statement::PrintStatement(exp))
            }
            TokenType::Dec => {
                self.parse_var_declaration()
            }
            TokenType::At => {
                self.parse_function_declaration()
            }
            _ => {
                match self.parse_expression() {
                    Ok(expression) => Ok(Statement::Expression(expression)),
                    Err(e) => Err(e)
                }
            }
        }
    }

    fn parse_function_declaration(&mut self) -> Result<Statement> {
        self.consume(TokenType::At)?;
        let name = self.consume_and_return(TokenType::Identifier)?;
        self.consume(TokenType::LParen)?;
        let mut args: HashMap<String, Token> = HashMap::new();
        while self.consume_and_return(TokenType::RParen).is_err() {
            let arg_name = self.consume_and_return(TokenType::Identifier)?;
            let arg_type = self.get_current_token().clone();
            self.advance();
            if self.get_current_token().tt != TokenType::Comma && self.get_current_token().tt != TokenType::RParen {
                return Err(anyhow!("Either expected a comma or a closing paren {}:{}", 
                        self.get_current_token().line, self.get_current_token().column))
            }
            let _ = self.consume_and_return(TokenType::Comma);
            args.insert(arg_name.lexeme, arg_type);
        }
        
        let return_token = self.get_current_token().clone();
        self.advance();
        self.consume(TokenType::LBrace)?;
        let mut stmts: Vec<Box<Statement>> = Vec::new();

        while self.consume_and_return(TokenType::RBrace).is_err() {
            let stmt = self.parse_statement()?;
            stmts.push(Box::new(stmt));
        }

        Ok(Statement::FunctionDeclaration { name, arguments: args, return_type: return_token, body: stmts })
    }

    fn parse_var_declaration(&mut self) -> Result<Statement> {
        self.advance();
        let name_token = self.consume_and_return(TokenType::Identifier)?;
        for dt in [TokenType::Int, TokenType::String] {
            if let Ok(var_dt) = self.consume_and_return(dt) {
                self.consume(TokenType::Equal)?;
                let value = self.parse_expression()?;
                self.consume(TokenType::SemiColon)?;
                return Ok(Statement::VariableDeclaration { name: name_token, var_type: var_dt.tt, value });
            }
        }
        Err(anyhow!("Expected a 'DataType' Token after variable declaration's name. {:?}:{:?}", name_token.line, name_token.column))
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_term()
    }


    fn parse_term(&mut self) -> Result<Expression> {
        self.create_binary_expression(&[TokenType::Plus, TokenType::Minus], Self::parse_factor)
    }

    fn parse_factor(&mut self) -> Result<Expression> {
        self.create_binary_expression(&[TokenType::Star, TokenType::Slash], Self::parse_literal)
    }

    fn parse_literal(&mut self) -> Result<Expression> {
        if self.get_current_token().tt == TokenType::NumberInt {
            let res = Ok(Expression::Literal
                    { value: Literal::NumberInt(self.get_current_token().lexeme.parse()?),
                      token: self.get_current_token().clone()
                    });
            self.advance();
            return res;
        } else if self.get_current_token().tt == TokenType::Identifier {
            let res = Ok(Expression::Literal(Literal::Variable(self.get_current_token().lexeme.parse()?)));
            self.advance();
            return res;
        };
        Err(anyhow!("This type of literal is not supported {:?}:{:?}", self.get_current_token().line, self.get_current_token().column))
    }

    fn create_binary_expression(&mut self, 
        match_token_types: &[TokenType],
        precedent_function: fn(&mut Self) -> Result<Expression>) -> Result<Expression> 
    {
        let mut lhs = precedent_function(self)?;
        while self.can_move_forward() && match_token_types.contains(&self.get_current_token().tt) {
            let operator = self.get_current_token().clone();
            self.advance();
            let rhs = precedent_function(self)?;
            lhs = Expression::Binary { left: Box::new(lhs), operator, right: Box::new(rhs) }
        }
        Ok(lhs)
    }
}
