use crate::instruction::Instruction;
use crate::memory::{Memory, MemoryError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CpuError {
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
    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
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
            Instruction::Mulh { rd, rs1, rs2 } => {
                let result = self.regs[*rs1] as i64 * self.regs[*rs2] as i64;
                self.regs[*rd] = ( result >> 32 )as i32;
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let result = self.regs[*rs1] as i64 * self.regs[*rs2] as u32 as i64;
                self.regs[*rd] = ( result >> 32 )as i32;
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let result = self.regs[*rs1] as u32 as  u64 * self.regs[*rs2] as u32 as u64;
                self.regs[*rd] = ( result >> 32 )as i32;
            }
            Instruction::Div { rd, rs1, rs2 } => {
                self.regs[*rd] = if self.regs[*rs2] != 0 {
                    self.regs[*rs1] / self.regs[*rs2]
                } else {
                    -1
                }
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                self.regs[*rd] = if self.regs[*rs2] != 0 {
                    ((self.regs[*rs1] as u32) / (self.regs[*rs2] as u32)) as i32
                } else {
                    -1
                }
            }
            Instruction::Rem {rd, rs1, rs2} =>{
                self.regs[*rd] = if self.regs[*rs2] == 0 {
                    self.regs[*rs1]
                }else{
                    self.regs[*rs1] % self.regs[*rs2]
                };
            }
            Instruction::Remu {rd, rs1, rs2} =>{
                self.regs[*rd] = if self.regs[*rs2] == 0 {
                    self.regs[*rs1]
                }else{
                    ((self.regs[*rs1] as u32) % (self.regs[*rs2] as u32)) as i32
                };
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
            Instruction::Slt { rd, rs1, rs2 } => {
                self.regs[*rd] = if self.regs[*rs1] < self.regs[*rs2] { 1 }else { 0 }
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                self.regs[*rd] = if (self.regs[*rs1] as u32 )< (self.regs[*rs2] as u32) { 1 }else { 0 }
            }
            Instruction::Addi { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] + imm,
            Instruction::Andi { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] & imm,
            Instruction::Ori { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] | imm,
            Instruction::Xori { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] ^ imm,
            Instruction::Slli { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] << imm,
            Instruction::Srli { rd, rs1, imm } => {
                self.regs[*rd] = ((self.regs[*rs1] as u32) >> imm) as i32
            }
            Instruction::Srai { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] >> imm,
            Instruction::Slti { rd, rs1, imm } => {
                self.regs[*rd] = if self.regs[*rs1] < *imm { 1 } else { 0 };
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                self.regs[*rd] = if (self.regs[*rs1] as u32 )< (*imm as u32) { 1 }else { 0 }
            }

            Instruction::Sb { rs1, rs2, imm } => {
                let addr = (self.regs[*rs2] + imm) as u32;
                self.memory
                    .write_byte(addr, (self.regs[*rs1] & 0xFF) as u8)?
            }
            Instruction::Sh { rs1, rs2, imm } => {
                let addr = (self.regs[*rs2] + imm) as u32;
                self.memory
                    .write_halfword(addr, (self.regs[*rs1] & 0xFFFF) as u16)?
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = (self.regs[*rs2] + imm) as u32;
                self.memory.write_word(addr, self.regs[*rs1])?
            }

            Instruction::Lb { rd, rs1, imm } => {
                let addr = (self.regs[*rs1] + imm) as u32;
                self.regs[*rd] = self.memory.read_byte(addr)? as i32; // sign-extend i8 -> i32
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let addr = (self.regs[*rs1] + imm) as u32;
                self.regs[*rd] = self.memory.read_byte(addr)? as u8 as i32; // zero-extend
            }
            Instruction::Lh { rd, rs1, imm } => {
                let addr = (self.regs[*rs1] + imm) as u32;
                self.regs[*rd] = self.memory.read_halfword(addr)? as i32; // sign-extend i16 -> i32
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let addr = (self.regs[*rs1] + imm) as u32;
                self.regs[*rd] = self.memory.read_halfword(addr)? as u16 as i32; // zero-extend
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = (self.regs[*rs1] + imm) as u32;
                self.regs[*rd] = self.memory.read_word(addr)?
            }

            Instruction::Beq { rs1, rs2, offset } => {
                if self.regs[*rs1] == self.regs[*rs2] {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Bne { rs1, rs2, offset } => {
                if self.regs[*rs1] != self.regs[*rs2] {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Blt { rs1, rs2, offset } => {
                if self.regs[*rs1] < self.regs[*rs2] {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Bltu { rs1, rs2, offset } => {
                if (self.regs[*rs1] as u32) < (self.regs[*rs2] as u32) {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Bge { rs1, rs2, offset } => {
                if self.regs[*rs1] >= self.regs[*rs2] {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Bgeu { rs1, rs2, offset } => {
                if (self.regs[*rs1] as u32) >= (self.regs[*rs2] as u32) {
                    next_pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Jal { rd, offset } => {
                self.regs[*rd] = (self.pc + 1) as i32;
                next_pc = (self.pc as i32 + offset) as usize;
            }
            Instruction::Jalr { rd, rs1, imm } => {
                self.regs[*rd] = (self.pc + 1) as i32;
                next_pc = (self.regs[*rs1] + imm) as usize;
            }
            Instruction::Lui { rd, imm } => {
                self.regs[*rd] = imm << 12 ;
            }
            Instruction::Auipc { rd, imm } => {
                self.regs[*rd] = (self.pc as i32).wrapping_add(imm << 12);
            }
            Instruction::Print { rs } => print!("x{}: {}\n ", rs, self.regs[*rs]),
        }
        self.regs[0] = 0;
        self.pc = next_pc;
        Ok(true)
    }
}
