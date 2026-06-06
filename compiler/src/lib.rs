use std::collections::HashMap;
use anyhow::{Result, anyhow};

use ir::Instruction;
use parser::Literal;

pub struct Compiler {
    pub instructions: Vec<Instruction>,
    pub assembly: String,
    pub temp_stack_map: HashMap<usize, usize>,
    pub current_stack_ptr: usize
}

impl Compiler {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            assembly: String::new(),
            temp_stack_map: HashMap::new(),
            current_stack_ptr: 0,
        }
    }

    fn emit(&mut self, assembly: &str) {
        self.assembly.push_str(assembly);
        self.assembly.push('\n');
    }

    pub fn compile(&mut self) -> Result<()> {
        let instructions = self.instructions.clone();
        self.emit("global _start");
        self.emit("extern print_int");
        self.emit("extern exit");
        self.emit("section .text");
        self.emit("_start:");
        self.emit(" push rbp");
        self.emit(" mov rbp, rsp"); 

        for instruction in instructions {
            match instruction {
                Instruction::LoadImmediate { literal, dst } => {
                    match literal {
                        Literal::NumberInt(x) => {
                            self.emit(" sub rsp, 8");
                            self.current_stack_ptr += 8;
                            let csp = self.current_stack_ptr;
                            self.emit(&format!(" mov qword [rbp-{csp}], {x}").to_string());
                            self.temp_stack_map.insert(dst.0, self.current_stack_ptr);
                        }
                    }
                },
                Instruction::Add { left, right, dst } => {
                    let l_sp = *self.temp_stack_map.get(&left.0).ok_or(anyhow!("Error while getting the immediate sp"))?;
                    let r_sp = *self.temp_stack_map.get(&right.0).ok_or(anyhow!("Error while getting the immediate sp"))?;
                    self.emit(&format!(" mov rax, [rbp-{l_sp}]").to_string());
                    self.emit(&format!(" mov rcx, [rbp-{r_sp}]").to_string());
                    self.emit(" add rax, rcx");
                    self.current_stack_ptr += 8;
                    let csp = self.current_stack_ptr;
                    self.emit(&format!(" mov qword [rbp-{csp}], rax").to_string());
                    self.temp_stack_map.insert(dst.0, self.current_stack_ptr);
                },
                Instruction::Subtract { left, right, dst } => todo!(),
                Instruction::Multiply { left, right, dst } => todo!(),
                Instruction::Divide { left, right, dst } => todo!(),
                Instruction::Print { dst } => {
                    if let Some(imm_sp) = self.temp_stack_map.get(&dst.0) {
                        self.emit(&format!(" mov rdi, [rbp-{imm_sp}]").to_string());
                        self.emit(" call print_int");
                    }
                },
                Instruction::StoreVar { name, dst } => todo!(),
            }
        }

        self.emit(" xor rdi, rdi");
        self.emit(" call exit");
        Ok(())
    }
}

