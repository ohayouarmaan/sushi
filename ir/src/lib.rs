use std::fmt::Display;
use lexer::{Token, TokenType};
use parser::{Literal, Statement, Expression};
use anyhow::{Result, anyhow};

macro_rules! InstructionSet {
    (
        $(
            $name: ident {
                $( $field: ident: $ty: ty ),*
            }
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub enum Instruction {
            $(
                $name {
                    $( $field: $ty ),*
                }
            ), *
        }

        impl std::fmt::Display for Instruction {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>
            ) -> std::fmt::Result {
                match self {
                    $(
                        Instruction::$name { $( $field ),* } => {
                            write!(f, stringify!($name))?;

                            $(
                                write!(f, " {}", $field)?;
                            )*

                                Ok(())
                        }
                    ),*
                }
            }

        }
    }
}

pub enum IRErrors {
    TypeResolutionError
}


#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I64,
    String
}

#[derive(Debug, Clone)]
pub struct Temp(pub usize, pub Type);

impl Display for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "t{}", self.0)
    }
}


InstructionSet! {
    LoadImmediate {
        literal: Literal,
        dst: Temp
    },
    Add {
        left: Temp,
        right: Temp,
        dst: Temp
    },
    Subtract {
        left: Temp,
        right: Temp,
        dst: Temp
    },
    Multiply {
        left: Temp,
        right: Temp,
        dst: Temp
    },
    Divide {
        left: Temp,
        right: Temp,
        dst: Temp
    },
    Print {
        dst: Temp
    },
    StoreVar {
        name: String,
        dst: Temp
    }
}

#[derive(Debug)]
pub struct IRGenerator {
    pub statements: Vec<Statement>,
    pub current_position: usize,
    pub instructions: Vec<Instruction>,
    pub temp_counter: usize
}


impl IRGenerator {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
            current_position: 0,
            instructions: Vec::new(),
            temp_counter: 0
        }
    }

    // fn can_move(&self) -> bool {
    //     self.current_position < self.statements.len()
    // }
    //
    // fn advance(&mut self) {
    //     if self.can_move() {
    //         self.current_position += 1;
    //     }
    // }

    fn get_current_statement(&self) -> &Statement {
        &self.statements[self.current_position]
    }


    fn generate_new_temp(&mut self, data_type: Type) -> Temp {
        let t = self.temp_counter;
        self.temp_counter += 1;
        Temp(t, data_type)
    }

    fn resolve_types_binary_expr(&self, t1: Type, t2: Type) -> Result<Type, IRErrors> {
        if t1 == Type::String || t2 == Type::String {
            return Ok(Type::String);
        } else if t1 == Type::I64 && t2 == Type::I64 {
            return Ok(Type::I64);
        }
        Err(IRErrors::TypeResolutionError)
    }

    fn ensure_type(&self, current_type: Type, expected_type: Type, ctx: Option<(usize, usize)>) -> Result<()> {
        if current_type == expected_type {
            return Ok(())
        }
        match ctx {
            Some((line, column)) => Err(anyhow!("expected {:?}, got {:?} in {}:{}", expected_type, current_type, line, column)),
            None => Err(anyhow!("expected {:?}, got {:?}", expected_type, current_type))
        }
    }

    fn token_type_to_type(&self, tt: TokenType) -> Result<Type> {
        match tt {
            TokenType::Int => Ok(Type::I64),
            TokenType::String => Ok(Type::String),
            _ => Err(anyhow!("Can not convert {:?} to a valid type", tt))
        }
    }

    pub fn generate(&mut self) -> Result<()> {
        let stmt = self.get_current_statement().clone();
        match stmt {
            Statement::Expression(e) => self.generate_expression(&e),
            Statement::VariableDeclaration { name, var_type, value } => self.generate_var_declaration_statement(name, var_type, value),
            Statement::PrintStatement(e) => self.generate_print_statement(&e),
        }
    }

    fn generate_var_declaration_statement(&mut self, name: Token, var_type: TokenType, value: Expression) -> Result<()> {
        let ty = self.token_type_to_type(var_type)?;
        let value = self.lower_expression(&value)?;
        self.ensure_type(ty, value.1.clone(), None)?;
        self.instructions.push(Instruction::StoreVar { name: name.lexeme, dst: value });
        Ok(())
    }

    fn generate_expression(&mut self, e: &Expression) -> Result<()> {
        self.lower_expression(e)?;
        Ok(())
    }

    fn generate_print_statement(&mut self, e: &Expression) -> Result<()> {
        let lowered_expr = self.lower_expression(e)?;
        self.instructions.push(Instruction::Print { dst: lowered_expr });
        Ok(())
    }


    fn lower_expression(&mut self, e: &Expression) -> Result<Temp> {
        match e {
            parser::Expression::Binary { left, operator, right } => {
                let left = self.lower_expression(left)?;
                let right = self.lower_expression(right)?;
                match operator.tt {
                    TokenType::Plus => {
                        if let Ok(resolved_type) = self.resolve_types_binary_expr(left.1.clone(), right.1.clone()) {
                            let dst = self.generate_new_temp(resolved_type);
                            self.instructions.push(Instruction::Add { left, right, dst: dst.clone() });
                            Ok(dst)
                        } else {
                            Err(anyhow!("Can not resolve {:?} and {:?}", left.1, right.1))
                        }
                    },
                    TokenType::Star => {
                        self.ensure_type(left.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        self.ensure_type(right.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        let dst = self.generate_new_temp(Type::I64);
                        self.instructions.push(Instruction::Multiply { left, right, dst: dst.clone() });
                        Ok(dst)
                    },
                    TokenType::Slash => {
                        self.ensure_type(left.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        self.ensure_type(right.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        let dst = self.generate_new_temp(Type::I64);
                        self.instructions.push(Instruction::Divide { left, right, dst: dst.clone() });
                        Ok(dst)
                    },
                    TokenType::Minus => {
                        self.ensure_type(left.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        self.ensure_type(right.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        let dst = self.generate_new_temp(Type::I64);
                        self.instructions.push(Instruction::Subtract { left, right, dst: dst.clone() });
                        Ok(dst)
                    },
                    _ => {
                        todo!("operator {:?} not implemented yet", operator.tt);
                    },
                }
            },
            parser::Expression::Literal(literal) => {
                match literal {
                    Literal::NumberInt(_) => {
                        let dst = self.generate_new_temp(Type::I64);
                        self.instructions.push(Instruction::LoadImmediate { literal: literal.clone(), dst: dst.clone() });
                        Ok(dst)
                    }
                }
            }
        }
    }
}

