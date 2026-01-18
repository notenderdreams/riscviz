#[derive(Debug)]
pub enum Instruction {
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
