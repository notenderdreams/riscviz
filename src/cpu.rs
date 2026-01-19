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
    pc: usize,
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
            Instruction::Addi { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] + imm,
            Instruction::Sub { rd, rs1, rs2 } => self.regs[*rd] = self.regs[*rs1] - self.regs[*rs2],
            Instruction::Mul { rd, rs1, rs2 } => self.regs[*rd] = self.regs[*rs1] * self.regs[*rs2],
            Instruction::Div { rd, rs1, rs2 } => {
                if self.regs[*rs2] == 0 {
                    return Err(CpuError::DivisionByZero);
                }
                self.regs[*rd] = self.regs[*rs1] / self.regs[*rs2]
            }
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
                next_pc = (self.regs[*rs1] as i32 + imm) as usize;
            }
        }
        self.regs[0] = 0;
        self.pc = next_pc;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_by_zero() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Div { rd: 1, rs1: 5, rs2: 0 }
        ]);
        let result = cpu.execute_next();
        assert!(matches!(result, Err(CpuError::DivisionByZero)));
    }

    #[test]
    fn test_arithmetic_ops() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Addi { rd: 1, rs1: 0, imm: 10 },
            Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
            Instruction::Add { rd: 3, rs1: 1, rs2: 2 },
            Instruction::Sub { rd: 4, rs1: 1, rs2: 2 },
            Instruction::Mul { rd: 5, rs1: 1, rs2: 2 },
            Instruction::Div { rd: 6, rs1: 1, rs2: 2 },
        ]);

        while cpu.execute_next().unwrap() {}

        assert_eq!(cpu.regs[1], 10);
        assert_eq!(cpu.regs[2], 5);
        assert_eq!(cpu.regs[3], 15);
        assert_eq!(cpu.regs[4], 5);
        assert_eq!(cpu.regs[5], 50);
        assert_eq!(cpu.regs[6], 2);
    }

    #[test]
    fn test_branch_not_equal_loop() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Addi { rd: 5, rs1: 0, imm: 3 },
            Instruction::Addi { rd: 5, rs1: 5, imm: -1 },
            Instruction::Bne { rs1: 5, rs2: 0, offset: -1 },
        ]);

        let mut steps = 0;
        while cpu.execute_next().unwrap() && steps < 20 {
            steps += 1;
        }

        assert_eq!(cpu.regs[5], 0);
    }

    #[test]
    fn test_branch_equal() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Addi { rd: 1, rs1: 0, imm: 5 },
            Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
            Instruction::Beq { rs1: 1, rs2: 2, offset: 2 },
            Instruction::Addi { rd: 3, rs1: 0, imm: 99 },
            Instruction::Addi { rd: 4, rs1: 0, imm: 42 },
        ]);

        while cpu.execute_next().unwrap() {}

        assert_eq!(cpu.regs[3], 0);
        assert_eq!(cpu.regs[4], 42);
    }

    #[test]
    fn test_branch_less_than() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Addi { rd: 1, rs1: 0, imm: 3 },
            Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
            Instruction::Blt { rs1: 1, rs2: 2, offset: 2 },
            Instruction::Addi { rd: 3, rs1: 0, imm: 99 },
            Instruction::Addi { rd: 4, rs1: 0, imm: 42 },
        ]);

        while cpu.execute_next().unwrap() {}

        assert_eq!(cpu.regs[3], 0);
        assert_eq!(cpu.regs[4], 42);
    }

    #[test]
    fn test_branch_greater_equal() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Addi { rd: 1, rs1: 0, imm: 7 },
            Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
            Instruction::Bge { rs1: 1, rs2: 2, offset: 2 },
            Instruction::Addi { rd: 3, rs1: 0, imm: 99 },
            Instruction::Addi { rd: 4, rs1: 0, imm: 42 },
        ]);

        while cpu.execute_next().unwrap() {}

        assert_eq!(cpu.regs[3], 0);
        assert_eq!(cpu.regs[4], 42);
    }

    #[test]
    fn test_memory_ops() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Addi { rd: 1, rs1: 0, imm: 100 },
            Instruction::Addi { rd: 2, rs1: 0, imm: 0 },
            Instruction::Sw { rs1: 1, rs2: 2, imm: 0 },
            Instruction::Lw { rd: 3, rs1: 2, imm: 0 },
        ]);

        while cpu.execute_next().unwrap() {}

        assert_eq!(cpu.regs[3], 100);
    }

    #[test]
    fn test_x0_immutable() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Addi { rd: 0, rs1: 0, imm: 999 },
        ]);

        cpu.execute_next().unwrap();

        assert_eq!(cpu.regs[0], 0);
    }

    #[test]
    fn test_program_counter_end() {
        let mut cpu = Cpu::default();
        cpu.load_program(vec![
            Instruction::Addi { rd: 1, rs1: 0, imm: 1 },
        ]);

        assert_eq!(cpu.execute_next().unwrap(), true);
        assert_eq!(cpu.execute_next().unwrap(), false);
    }
    #[test]
    fn test_while_loop() {
        let mut cpu = Cpu::default();

        cpu.load_program(vec![
            // int i = 0;
            // while (i < 10) {
            //     i++;
            // }
            Instruction::Addi { rd: 1, rs1: 0, imm: 0 },
            Instruction::Addi { rd: 2, rs1: 0, imm: 10 },
            Instruction::Bge { rs1: 1, rs2: 2, offset: 2 },
            Instruction::Addi { rd: 1, rs1: 1, imm: 1 },
            Instruction::Blt { rs1: 1, rs2: 2, offset: -2 },
        ]);

        while cpu.execute_next().unwrap() {}
        assert_eq!(cpu.regs[1], 10);
    }

    #[test]
    fn test_for_loop() {
        let mut cpu = Cpu::default();

        cpu.load_program(vec![
            // int sum = 0;
            // for (int i = 0; i < 5; i++) {
            //     sum += i;
            // }
            Instruction::Addi { rd: 1, rs1: 0, imm: 0 },
            Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
            Instruction::Addi { rd: 3, rs1: 0, imm: 0 },
            Instruction::Bge { rs1: 1, rs2: 2, offset: 3 },
            Instruction::Add { rd: 3, rs1: 3, rs2: 1 },
            Instruction::Addi { rd: 1, rs1: 1, imm: 1 },
            Instruction::Blt { rs1: 1, rs2: 2, offset: -3 },
        ]);

        while cpu.execute_next().unwrap() {}
        assert_eq!(cpu.regs[3], 10);
    }
    #[test]
    fn test_non_leaf_function() {
        let mut cpu = Cpu::default();

        cpu.load_program(vec![
            // int bar() { return 41; }
            Instruction::Addi { rd: 10, rs1: 0, imm: 41 },   // 0
            Instruction::Jalr { rd: 0, rs1: 1, imm: 0 },     // 1: return

            // int foo() { return bar() + 1; }
            Instruction::Addi { rd: 2, rs1: 2, imm: -4 },    // 2: sp -= 4
            Instruction::Sw   { rs1: 1, rs2: 2, imm: 0 },    // 3: save ra
            Instruction::Jal  { rd: 1, offset: -4 },         // 4: call bar (→0)
            Instruction::Addi { rd: 10, rs1: 10, imm: 1 },   // 5: result += 1
            Instruction::Lw   { rd: 1, rs1: 2, imm: 0 },     // 6: restore ra
            Instruction::Addi { rd: 2, rs1: 2, imm: 4 },     // 7: sp += 4
            Instruction::Jalr { rd: 0, rs1: 1, imm: 0 },     // 8: return

            // int main() { foo(); }
            Instruction::Jal  { rd: 1, offset: -7 },         // 9: call foo (→2)
        ]);

        cpu.pc = 9; // entry point
        while cpu.execute_next().unwrap() {}
        assert_eq!(cpu.regs[10], 42);
    }
    #[test]
    fn test_simple_recursion() {
        let mut cpu = Cpu::default();

        cpu.load_program(vec![
            // int f(int n) {
            //     if (n == 0) return 0;
            //     return 1 + f(n - 1);
            // }
            Instruction::Beq  { rs1: 10, rs2: 0, offset: 9 },  // 0: if n==0 goto base
            Instruction::Addi { rd: 2, rs1: 2, imm: -4 },      // 1: sp -= 4
            Instruction::Sw   { rs1: 1, rs2: 2, imm: 0 },      // 2: save ra
            Instruction::Addi { rd: 10, rs1: 10, imm: -1 },    // 3: n -= 1
            Instruction::Jal  { rd: 1, offset: -4 },           // 4: call f(n-1)
            Instruction::Lw   { rd: 1, rs1: 2, imm: 0 },       // 5: restore ra
            Instruction::Addi { rd: 2, rs1: 2, imm: 4 },       // 6: sp += 4
            Instruction::Addi { rd: 10, rs1: 10, imm: 1 },     // 7: result += 1
            Instruction::Jalr { rd: 0, rs1: 1, imm: 0 },       // 8: return

            // base case: return 0
            Instruction::Addi { rd: 10, rs1: 0, imm: 0 },      // 9
            Instruction::Jalr { rd: 0, rs1: 1, imm: 0 },       // 10

            // int main() { f(3); }
            Instruction::Addi { rd: 10, rs1: 0, imm: 3 },      // 11
            Instruction::Jal  { rd: 1, offset: -12 },          // 12: call f
        ]);

        cpu.pc = 11; // entry point
        while cpu.execute_next().unwrap() {}
        assert_eq!(cpu.regs[10], 3);
    }
}