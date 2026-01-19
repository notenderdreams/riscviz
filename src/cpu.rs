use crate::instruction::Instruction;
use crate::memory::{Memory, MemoryError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CpuError {
    #[error("Division by zero")]
    DivisionByZero,
    #[error(transparent)]
    MemoryError(#[from] MemoryError),
}

pub struct Cpu {
    pub regs: [i32; 32],
    memory: Memory,
    pub pc: usize,
    program: Vec<Instruction>,
}
impl Default for Cpu {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl Cpu {
    pub fn new(mem_size: usize) -> Self {
        let mut cpu = Cpu {
            regs: [0; 32],
            memory: Memory::new(mem_size),
            pc: 0,
            program: vec![],
        };
        cpu.regs[2] = cpu.memory.size() as i32;
        cpu
    }
    pub fn load_program(&mut self, program: Vec<Instruction>){
        self.program = program;
        self.pc = 0;
    }

    pub fn execute_next(&mut self) -> Result<bool, CpuError> {
        if self.pc >= self.program.len() {
            return Ok(false);
        }
        let inst = &self.program[self.pc];
        let mut next_pc = self.pc + 1;

        match inst {
            Instruction::Add { rd, rs1, rs2 } => self.regs[*rd] = self.regs[*rs1] + self.regs[*rs2],
            Instruction::Sub { rd, rs1, rs2 } => self.regs[*rd] = self.regs[*rs1] - self.regs[*rs2],
            Instruction::Mul { rd, rs1, rs2 } => self.regs[*rd] = self.regs[*rs1] * self.regs[*rs2],
            Instruction::Div { rd, rs1, rs2 } => {
                if self.regs[*rs2] == 0 {
                    return Err(CpuError::DivisionByZero);
                }
                self.regs[*rd] = self.regs[*rs1] / self.regs[*rs2]
            }
            Instruction::And { rd, rs1, rs2 } => self.regs[*rd] = self.regs[*rs1] & self.regs[*rs2],
            Instruction::Or { rd, rs1, rs2 } => self.regs[*rd] = self.regs[*rs1] | self.regs[*rs2],
            Instruction::Xor { rd, rs1, rs2 } => self.regs[*rd] = self.regs[*rs1] ^ self.regs[*rs2],
            Instruction::Sll { rd, rs1, rs2 } => {
                self.regs[*rd] = self.regs[*rs1] << (self.regs[*rs2] & 0x1F)
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                self.regs[*rd] = ((self.regs[*rs1] as u32) >> (self.regs[*rs2] & 0x1F)) as i32
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                self.regs[*rd] = self.regs[*rs1] >> (self.regs[*rs2] & 0x1F)
            }
            Instruction::Addi { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] + imm,
            Instruction::Andi { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] & imm,
            Instruction::Ori { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] | imm,
            Instruction::Xori { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] ^ imm,
            Instruction::Slti { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] << imm,
            Instruction::Print { rs } => print!("x{}: {}\n ",rs, self.regs[*rs]),
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = (self.regs[*rs2] + imm) as u32;
                self.memory.write_word(addr, self.regs[*rs1])?
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = (self.regs[*rs1] + imm) as u32;
                self.regs[*rd] = self.memory.read_word(addr)?
            }
            Instruction::Beq {rs1, rs2, offset}=>{
                if self.regs[*rs1] == self.regs[*rs2] {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Bne {rs1, rs2, offset}=>{
                if self.regs[*rs1] != self.regs[*rs2] {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Blt {rs1, rs2, offset}=>{
                if self.regs[*rs1] < self.regs[*rs2] {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Bge {rs1, rs2, offset}=>{
                if self.regs[*rs1] >= self.regs[*rs2] {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Jal { rd, offset}=>{
                self.regs[*rd] = (self.pc + 1) as i32 ;
                next_pc = (self.pc as i32 + offset) as usize;
            }
            Instruction::Jalr {rd, rs1, imm}=>{
                self.regs[*rd] = (self.pc +1) as i32 ;
                next_pc = (self.regs[*rs1] + imm) as usize;
            }
        }
        self.regs[0] = 0;
        self.pc = next_pc;
        Ok(true)
    }
}