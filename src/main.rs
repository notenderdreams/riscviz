struct Cpu {
    regs: [i32; 32],
    ic: usize,
    memory: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            regs: [0; 32],
            ic: 0,
            memory: vec![0; 1024], //Default size = 1024 bytes
        };
        cpu.regs[2] = 1024;
        cpu
    }

    pub fn execute(&mut self, program: &Program) {
        let length = program.instructions.len();
        while self.ic < length {
            let curr_ins = program.instructions.get(self.ic).unwrap();
            print!("{:?}   \t", curr_ins);

            match curr_ins {
                Instruction::Add { rd, rs1, rs2 } => {
                    self.regs[*rd] = self.regs[*rs1] + self.regs[*rs2]
                }
                Instruction::Addi { rd, rs1, imm } => self.regs[*rd] = self.regs[*rs1] + imm,
                Instruction::Sub { rd, rs1, rs2 } => {
                    self.regs[*rd] = self.regs[*rs1] - self.regs[*rs2]
                }
                Instruction::Mul { rd, rs1, rs2 } => {
                    self.regs[*rd] = self.regs[*rs1] * self.regs[*rs2]
                }
                Instruction::Div { rd, rs1, rs2 } => {
                    self.regs[*rd] = self.regs[*rs1] / self.regs[*rs2]
                }
                Instruction::Print { rs } => print!(": {}", self.regs[*rs]),
                Instruction::Sw { rs1, rs2, imm } => {
                    let mem = &mut self.memory;
                    let index = (self.regs[*rs2] + imm) as usize;
                    let value = self.regs[*rs1];
                    mem[index] = (value & 0xFF) as u8;
                    mem[index + 1] = (value >> 8 & 0xFF) as u8;
                    mem[index + 2] = (value >> 16 & 0xFF) as u8;
                    mem[index + 3] = (value >> 24 & 0xFF) as u8;
                }
                Instruction::Lw { rd, rs1, imm } => {
                    let index = (self.regs[*rs1] + imm) as usize;
                    self.regs[*rd] = (self.memory[index] as i32)
                        | (self.memory[index + 1] as i32) << 8
                        | (self.memory[index + 2] as i32) << 16
                        | (self.memory[index + 3] as i32) << 24;
                }
            }
            println!();
            self.ic += 1;
        }
    }
}
#[derive(Debug)]
enum Instruction {
    // R-Format
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Mul { rd: usize, rs1: usize, rs2: usize },
    Div { rd: usize, rs1: usize, rs2: usize },
    // I-Format
    Addi { rd: usize, rs1: usize, imm: i32 },
    Lw { rd: usize, rs1: usize, imm: i32 },
    // S- Format
    Sw { rs1: usize, rs2: usize, imm: i32 },
    // For Debug
    Print { rs: usize },
}

struct Program {
    instructions: Vec<Instruction>,
}

fn main() {
    let mut cpu = Cpu::new();
    let program = Program {
        instructions: vec![
            Instruction::Addi {
                rd: 6,
                rs1: 0,
                imm: 20,
            },
            Instruction::Addi {
                rd: 7,
                rs1: 0,
                imm: 30,
            },
            Instruction::Add {
                rd: 5,
                rs1: 6,
                rs2: 7,
            },
            Instruction::Sub {
                rd: 8,
                rs1: 6,
                rs2: 7,
            },
            Instruction::Mul {
                rd: 9,
                rs1: 6,
                rs2: 7,
            },
            Instruction::Div {
                rd: 10,
                rs1: 6,
                rs2: 7,
            },
            Instruction::Print { rs: 5 },
            Instruction::Print { rs: 8 },
            Instruction::Print { rs: 9 },
            Instruction::Print { rs: 10 },
            Instruction::Addi {
                rd: 2,
                rs1: 2,
                imm: -4,
            },
            Instruction::Print { rs: 2 },
            Instruction::Sw {
                rs1: 5,
                rs2: 2,
                imm: 0,
            },
            Instruction::Lw {
                rd: 12,
                rs1: 2,
                imm: 0,
            },
            Instruction::Addi {
                rd: 2,
                rs1: 2,
                imm: 4,
            },
            Instruction::Print { rs: 12 },
            Instruction::Print { rs: 2 },
        ],
    };

    cpu.execute(&program);
}
