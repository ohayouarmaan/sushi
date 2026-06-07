use std::collections::HashMap;
use anyhow::{Result, anyhow};

use ir::Instruction;
use parser::{Expression, Literal};

pub struct Label(HashMap<String, Vec<String>>);

pub struct Header(Vec<String>);
pub struct Text(Vec<String>);
pub struct Data(Vec<String>);
pub struct Bss(Vec<String>);

pub struct Compiler {
    pub instructions: Vec<Instruction>,
    pub labels: Label,
    pub header: Header,
    pub text: Text,
    pub data: Data,
    pub bss: Bss,
    pub temp_stack_map: HashMap<usize, usize>,
    pub variables: HashMap<String, usize>,
    pub current_stack_ptr: usize
}

impl Compiler {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            temp_stack_map: HashMap::new(),
            variables: HashMap::new(),
            current_stack_ptr: 0,
            labels: Label(HashMap::new()),
            header: Header(Vec::new()),
            text: Text(Vec::new()),
            data: Data(Vec::new()),
            bss: Bss(Vec::new()),
        }
    }

    fn emit_header(&mut self, s: &str) {
        self.header.0.push(s.to_string());
    }

    fn emit_text(&mut self, s: &str) {
        self.text.0.push(s.to_string());
    }

    fn emit_data(&mut self, s: &str) {
        self.data.0.push(s.to_string());
    }

    fn emit_bss(&mut self, s: &str) {
        self.bss.0.push(s.to_string());
    }

    fn emit_label(&mut self, label: &str, assembly: &str) {
        self.labels.0
            .entry(label.to_string())
            .or_default()
            .push(assembly.to_string());
    }

    pub fn compile(&mut self) -> Result<()> {
        let instructions = self.instructions.clone();
        self.emit_header("global _start");
        self.emit_header("extern print_int");
        self.emit_header("extern exit");

        self.emit_text("section .text");

        self.emit_label("_start", " push rbp");
        self.emit_label("_start", " mov rbp, rsp");

        let current_label = "_start";
        for instruction in instructions {
            match instruction {
                Instruction::LoadImmediate { literal, dst } => {
                    self.emit_label(current_label, " ; Loading immediate");
                    match literal {
                        Literal::NumberInt(x) => {
                            self.emit_label(current_label, " sub rsp, 8");
                            self.current_stack_ptr += 8;
                            let csp = self.current_stack_ptr;
                            self.emit_label(current_label, &format!(" mov qword [rbp-{csp}], {x}").to_string());
                            self.temp_stack_map.insert(dst.0, self.current_stack_ptr);
                        }
                        Literal::Variable(_) => return Err(anyhow!("To load variables there is another instruction")),
                    }
                    self.emit_label(current_label, "");
                },
                Instruction::Add { left, right, dst } => {
                    self.emit_label(current_label, "; Adding");
                    let l_sp = *self.temp_stack_map.get(&left.0).ok_or(anyhow!("Error while getting the immediate sp"))?;
                    let r_sp = *self.temp_stack_map.get(&right.0).ok_or(anyhow!("Error while getting the immediate sp"))?;
                    self.emit_label(current_label, &format!(" mov rax, [rbp-{l_sp}]").to_string());
                    self.emit_label(current_label, &format!(" mov rcx, [rbp-{r_sp}]").to_string());
                    self.emit_label(current_label, " add rax, rcx");
                    self.current_stack_ptr += 8;
                    self.emit_label(current_label, " sub rsp, 8");
                    let csp = self.current_stack_ptr;
                    self.emit_label(current_label, &format!(" mov qword [rbp-{csp}], rax").to_string());
                    self.temp_stack_map.insert(dst.0, self.current_stack_ptr);
                    self.emit_label(current_label, "");
                },
                Instruction::Subtract { left, right, dst } => todo!(),
                Instruction::Multiply { left, right, dst } => todo!(),
                Instruction::Divide { left, right, dst } => todo!(),
                Instruction::Print { dst } => {
                    self.emit_label(current_label, " ; Printing");
                    if let Some(imm_sp) = self.temp_stack_map.get(&dst.0) {
                        self.emit_label(current_label, &format!(" mov rdi, [rbp-{imm_sp}]").to_string());
                        self.emit_label(current_label, " call print_int");
                    }
                    self.emit_label(current_label, "");
                },
                Instruction::StoreVar { name, dst } => {
                    self.emit_label(current_label, " ; Storing variable");
                    let stack_offset = self.temp_stack_map.get(&dst.0.clone()).ok_or(anyhow!("Invalid stack offset"))?;
                    self.variables.insert(name, *stack_offset);
                    dbg!(&self.variables);
                    self.emit_label(current_label, "");
                },
                Instruction::LoadVar { name, dst } => {
                    self.emit_label(current_label, " ; Loading variable");
                    self.current_stack_ptr += 8;
                    self.emit_label(current_label, " sub rsp, 8");
                    let dst_offset = self.current_stack_ptr;
                    self.temp_stack_map.insert(dst.0, dst_offset);
                    let var_offset = *self.variables
                        .get(&name)
                        .ok_or(anyhow!("variable not found"))?;
                    self.emit_label(
                        current_label,
                        &format!(" mov rax, [rbp-{var_offset}]")
                    );
                    self.emit_label(
                        current_label,
                        &format!(" mov [rbp-{dst_offset}], rax")
                    );
                    self.emit_label(current_label, "");
                }
            }
        }

        self.emit_label(current_label, " xor rdi, rdi");
        self.emit_label(current_label, " call exit");
        Ok(())
    }

    pub fn build(&self) -> String {
        let mut output = String::new();

        for line in &self.header.0 {
            output.push_str(line);
            output.push('\n');
        }

        output.push('\n');

        if !self.data.0.is_empty() {
            output.push_str("section .data\n");

            for line in &self.data.0 {
                output.push_str(line);
                output.push('\n');
            }
        }

        if !self.bss.0.is_empty() {
            output.push_str("section .bss\n");

            for line in &self.bss.0 {
                output.push_str(line);
                output.push('\n');
            }
        }

        output.push_str("section .text\n");

        for (label, body) in &self.labels.0 {
            output.push_str(&format!("{label}:\n"));

            for line in body {
                println!("{}", line);
                output.push_str(line);
                output.push('\n');
            }

            output.push('\n');
        }

        output
    }
}

