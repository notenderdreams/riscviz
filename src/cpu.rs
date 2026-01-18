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
    regs: [i32; 32],
    memory: Memory,
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
        };
        cpu.regs[2] = cpu.memory.size() as i32;
        cpu
    }

    pub fn execute(&mut self, inst: Instruction) -> Result<(), CpuError> {
        match inst {
            Instruction::Add { rd, rs1, rs2 } => self.regs[rd] = self.regs[rs1] + self.regs[rs2],
            Instruction::Addi { rd, rs1, imm } => self.regs[rd] = self.regs[rs1] + imm,
            Instruction::Sub { rd, rs1, rs2 } => self.regs[rd] = self.regs[rs1] - self.regs[rs2],
            Instruction::Mul { rd, rs1, rs2 } => self.regs[rd] = self.regs[rs1] * self.regs[rs2],
            Instruction::Div { rd, rs1, rs2 } => {
                if self.regs[rs2] == 0 {
                    return Err(CpuError::DivisionByZero);
                }
                self.regs[rd] = self.regs[rs1] / self.regs[rs2]
            }
            Instruction::Print { rs } => print!(": {}", self.regs[rs]),
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = (self.regs[rs2] + imm) as u32;
                self.memory.write_word(addr, self.regs[rs1])?
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = (self.regs[rs1] + imm) as u32;
                self.regs[rd] = self.memory.read_word(addr)?
            }
        }
        self.regs[0] = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_by_zero() {
        let mut cpu = Cpu::default();
        let result = cpu.execute(Instruction::Div {
            rd: 1, rs1: 5, rs2: 0
        });
        assert!(matches!(result, Err(CpuError::DivisionByZero)));
    }
}