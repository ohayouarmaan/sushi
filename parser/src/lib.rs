use lexer::{Token, TokenType};
use anyhow::{Result, anyhow};


#[derive(Debug)]
pub enum Literal {
    NumberInt(u64)
}

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>
    },
    Literal(Literal)
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression)
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
        if self.current_index < self.tokens.len() {
            return Some(&self.tokens[self.current_index+1]);
        }
        None
    }

    fn advance(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_index += 1;
        }
    }

    fn get_current_token(&self) -> &Token {
        &self.tokens[self.current_index]
    }

    pub fn parse(&mut self) -> Result<()> {
        let mut statments: Vec<Statement> = Vec::new();
        if self.peek().is_some() {
            match self.parse_statement() {
                Ok(t) => statments.push(t),
                Err(e) => return Err(e)
            }
        }
        self.statements = Some(statments);
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        //! RIGHT NOW IT WILL JUST BE AN EXPRESSION BECAUSE WE DON'T HAVE ANY OTHER STATEMENT
        //! DEFINED
        
        match self.parse_expression() {
            Ok(expression) => Ok(Statement::Expression(expression)),
            Err(e) => Err(e)
        }
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
            let res = Ok(Expression::Literal(Literal::NumberInt(self.get_current_token().lexeme.parse()?)));
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

        while match_token_types.contains(&self.get_current_token().tt) {
            let operator = self.get_current_token().clone();
            self.advance();

            let rhs = precedent_function(self)?;
            lhs = Expression::Binary { left: Box::new(lhs), operator, right: Box::new(rhs) }
        }
        Ok(lhs)
    }
}
