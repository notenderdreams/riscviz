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
    Jalr { rd: usize, rs1: usize, imm: i32 },
    // S-Format
    Sw { rs1: usize, rs2: usize, imm: i32 },
    // B-type
    Beq { rs1: usize, rs2: usize, offset: i32 }, // equal
    Bne { rs1: usize, rs2: usize, offset: i32 }, // not equal
    Blt { rs1: usize, rs2: usize, offset: i32 }, // less than
    Bge { rs1: usize, rs2: usize, offset: i32 }, // greater/equal
    // J-Format
    Jal { rd: usize, offset: i32 },
    // For Debug
    Print { rs: usize },
}
