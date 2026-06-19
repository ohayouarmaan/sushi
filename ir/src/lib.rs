use std::{collections::HashMap, fmt::Display};
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

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: String,
    pub arg_type: Token
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub body: Vec<Instruction>,
    pub return_type: Token
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
    },
    LoadVar {
        name: String,
        dst: Temp
    }
}


#[derive(Debug)]
pub struct IRGenerator {
    pub statements: Vec<Statement>,
    pub current_position: usize,
    pub instructions: Vec<Instruction>,
    pub temp_counter: usize,
    pub var_type_holder: HashMap<String, Type>,
    pub functions: Vec<Function>
}

impl IRGenerator {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
            current_position: 0,
            instructions: Vec::new(),
            temp_counter: 0,
            var_type_holder: HashMap::new(),
            functions: Vec::new()
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

    fn get_current_statement(&self) -> Option<&Statement> {
        if self.current_position < self.statements.len() {
            return Some(&self.statements[self.current_position]);
        }
        None
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
        while let Some(stmt) = self.get_current_statement().cloned() {
            let generated = self.generate_ir_from_statement(&stmt)?;
            self.instructions.extend(generated);
            self.current_position += 1;
        }
        Ok(())
    }

    fn generate_ir_from_statement(&mut self, stmt: &Statement) -> Result<Vec<Instruction>> {
        match stmt {
            Statement::Expression(e) => {
                Ok(self.generate_expression(&e.clone())?)
            }
            Statement::VariableDeclaration {
                name,
                var_type,
                value,
            } => {
                Ok(self.generate_var_declaration_statement(
                        name.clone(),
                        *var_type,
                        value.clone(),
                )?)
            }
            Statement::PrintStatement(e) => {
                let insts = self.generate_print_statement(&e.clone())?;
                dbg!(&insts);
                Ok(insts)
            }
            Statement::FunctionDeclaration { name, arguments, return_type, body } => {
                let mut insts = vec![];
                let mut parsed_args: Vec<FunctionArgument> = vec![];
                dbg!(&name);
                for arg in arguments {
                    parsed_args.push(FunctionArgument { name: arg.0.clone(), arg_type: arg.1.clone() });
                }
                dbg!(&parsed_args);
                for stmt in body {
                    let instructions = self.generate_ir_from_statement(stmt)?;
                    insts.extend(instructions);
                }
                let f = Function { name: name.lexeme.clone(), arguments: parsed_args, body: insts, return_type: return_type.clone() };
                dbg!(&f);

                self.functions.push(f);
                Ok(vec![])
            },
        }
    }

    fn generate_var_declaration_statement(&mut self, name: Token, var_type: TokenType, value: Expression) -> Result<Vec<Instruction>> {
        let ty = self.token_type_to_type(var_type)?;
        let mut insts = vec![]; 
        let lowered_expr = self.lower_expression(&value)?;
        insts.extend(lowered_expr.0.clone());
        insts.push(Instruction::StoreVar { name: name.lexeme.clone(), dst: lowered_expr.1.clone() });
        self.ensure_type(ty.clone(), lowered_expr.1.1.clone(), None)?;
        self.var_type_holder.insert(name.lexeme.to_string(), ty);
        Ok(insts)
    }

    fn generate_expression(&mut self, e: &Expression) -> Result<Vec<Instruction>> {
        let lowered = self.lower_expression(e)?;
        Ok(lowered.0)
    }

    fn generate_print_statement(&mut self, e: &Expression) -> Result<Vec<Instruction>> {
        let lowered_expr = self.lower_expression(e)?;
        let mut insts: Vec<Instruction> = vec![];
        insts.extend(lowered_expr.0);
        insts.push(Instruction::Print { dst: lowered_expr.1 });
        Ok(insts)
    }


    fn lower_expression(&mut self, e: &Expression) -> Result<(Vec<Instruction>, Temp)> {
        let mut insts: Vec<Instruction> = vec![];
        match e {
            parser::Expression::Binary { left, operator, right } => {
                let left = self.lower_expression(left)?;
                insts.extend(left.0);
                let right = self.lower_expression(right)?;
                insts.extend(right.0);
                match operator.tt {
                    TokenType::Plus => {
                        if let Ok(resolved_type) = self.resolve_types_binary_expr(left.1.1.clone(), right.1.1.clone()) {
                            let dst = self.generate_new_temp(resolved_type);
                            insts.push(Instruction::Add { left: left.1, right: right.1, dst: dst.clone() });
                            return Ok((insts, dst));
                        } else {
                            Err(anyhow!("Can not resolve {:?} and {:?}", left.1, right.1))
                        }
                    },
                    TokenType::Star => {
                        self.ensure_type(left.1.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        self.ensure_type(right.1.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        let dst = self.generate_new_temp(Type::I64);
                        insts.push(Instruction::Multiply { left: left.1, right: right.1, dst: dst.clone() });
                        Ok((insts, dst))
                    },
                    TokenType::Slash => {
                        self.ensure_type(left.1.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        self.ensure_type(right.1.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        let dst = self.generate_new_temp(Type::I64);
                        insts.push(Instruction::Divide { left: left.1, right: right.1, dst: dst.clone() });
                        Ok((insts, dst))
                    },
                    TokenType::Minus => {
                        self.ensure_type(left.1.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        self.ensure_type(right.1.1.clone(), Type::I64, Some((operator.line, operator.column)))?;
                        let dst = self.generate_new_temp(Type::I64);
                        insts.push(Instruction::Subtract { left: left.1, right: right.1, dst: dst.clone() });
                        Ok((insts, dst))
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
                        insts.push(Instruction::LoadImmediate { literal: literal.clone(), dst: dst.clone() });
                        Ok((insts, dst))
                    }
                    Literal::Variable(var_name) => {
                        let var_type = self.var_type_holder.get(var_name).ok_or(anyhow!("variable {var_name} not found"));
                        match var_type {
                            Ok(c) => {
                                let dst = self.generate_new_temp(c.clone());
                                insts.push(Instruction::LoadVar { name: var_name.clone(), dst: dst.clone() });
                                Ok((insts, dst))
                            },
                            Err(e) => {
                                Err(e)
                            },
                        }
                    },
                }
            }
        }
    }
}

