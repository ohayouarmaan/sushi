use std::fmt::Display;
use lexer::TokenType;
use parser::{Literal, Statement, Expression};

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



#[derive(Debug, Clone)]
pub struct Temp(pub usize);

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


    pub fn generate(&mut self) {
        let stmt = self.get_current_statement().clone();
        match stmt {
            Statement::Expression(e) => {
                self.generate_expression(&e);
            },
            Statement::PrintStatement(e) => self.generate_print_statement(&e),
        }
    }

    fn generate_expression(&mut self, e: &Expression) {
        self.lower_expression(e);
    }

    fn generate_print_statement(&mut self, e: &Expression) {
        let lowered_expr = self.lower_expression(e);
        self.instructions.push(Instruction::Print { dst: lowered_expr });
    }

    fn generate_new_temp(&mut self) -> Temp {
        let t = self.temp_counter;
        self.temp_counter += 1;
        Temp(t)
    }

    fn lower_expression(&mut self, e: &Expression) -> Temp {
        match e {
            parser::Expression::Binary { left, operator, right } => {
                let left = self.lower_expression(left);
                let right = self.lower_expression(right);
                match operator.tt {
                    TokenType::Plus => {
                        let dst = self.generate_new_temp();
                        self.instructions.push(Instruction::Add { left, right, dst: dst.clone() });
                        dst
                    },
                    TokenType::Star => {
                        let dst = self.generate_new_temp();
                        self.instructions.push(Instruction::Multiply { left, right, dst: dst.clone() });
                        dst
                    },
                    TokenType::Slash => {
                        let dst = self.generate_new_temp();
                        self.instructions.push(Instruction::Divide { left, right, dst: dst.clone() });
                        dst
                    },
                    TokenType::Minus => {
                        let dst = self.generate_new_temp();
                        self.instructions.push(Instruction::Subtract { left, right, dst: dst.clone() });
                        dst
                    },
                    _ => {
                        todo!("operator {:?} not implemented yet", operator.tt);
                    },
                }
            },
            parser::Expression::Literal(literal) => {
                match literal {
                    Literal::NumberInt(_) => {
                        let dst = self.generate_new_temp();
                        self.instructions.push(Instruction::LoadImmediate { literal: literal.clone(), dst: dst.clone() });
                        dst
                    }
                }
            }
        }
    }
}

