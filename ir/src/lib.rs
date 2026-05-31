use lexer::TokenType;
use parser::{Literal, Statement, Expression};

#[derive(Debug)]
pub enum Instruction {
    LoadImmediate {
        literal: Literal,
        dst: usize
    },
    Add {
        left: usize,
        right: usize,
        dst: usize
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
            }
        }
    }

    fn generate_expression(&mut self, e: &Expression) {
        self.lower_expression(e);
    }

    fn generate_new_temp(&mut self) -> usize {
        let t = self.temp_counter;
        self.temp_counter += 1;
        t
    }

    fn lower_expression(&mut self, e: &Expression) -> usize {
        match e {
            parser::Expression::Binary { left, operator, right } => {
                let left = self.lower_expression(left);
                let right = self.lower_expression(right);
                match operator.tt {
                    TokenType::Plus => {
                        let dst = self.generate_new_temp();
                        self.instructions.push(Instruction::Add { left, right, dst });
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
                        self.instructions.push(Instruction::LoadImmediate { literal: literal.clone(), dst });
                        dst
                    }
                }
            }
        }
    }
}

