#[derive(Debug)]
pub enum Instruction {
    // R-Format
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Mul { rd: usize, rs1: usize, rs2: usize },
    Mulh { rd: usize, rs1: usize, rs2: usize },
    Mulhsu { rd: usize, rs1: usize, rs2: usize },
    Mulhu { rd: usize, rs1: usize, rs2: usize },
    Div { rd: usize, rs1: usize, rs2: usize },
    Divu { rd: usize, rs1: usize, rs2: usize },
    Rem { rd: usize, rs1: usize, rs2: usize },
    Remu { rd: usize, rs1: usize, rs2: usize },
    And { rd: usize, rs1: usize, rs2: usize },
    Or { rd: usize, rs1: usize, rs2: usize },
    Xor { rd: usize, rs1: usize, rs2: usize },
    Sll { rd: usize, rs1: usize, rs2: usize },
    Srl { rd: usize, rs1: usize, rs2: usize },
    Sra { rd: usize, rs1: usize, rs2: usize },
    Slt { rd: usize, rs1: usize, rs2: usize },
    Sltu { rd: usize, rs1: usize, rs2: usize },

    // I-Format
    Addi { rd: usize, rs1: usize, imm: i32 },
    Andi { rd: usize, rs1: usize, imm: i32 },
    Ori { rd: usize, rs1: usize, imm: i32 },
    Xori { rd: usize, rs1: usize, imm: i32 },
    Slli { rd: usize, rs1: usize, imm: i32 },
    Srli { rd: usize, rs1: usize, imm: i32 },
    Srai { rd: usize, rs1: usize, imm: i32 },
    Slti { rd: usize, rs1: usize, imm: i32 },
    Lw { rd: usize, rs1: usize, imm: i32 },
    Jalr { rd: usize, rs1: usize, imm: i32 },
    Lb { rd: usize, rs1: usize, imm: i32 },
    Lh { rd: usize, rs1: usize, imm: i32 },
    Lbu { rd: usize, rs1: usize, imm: i32 },
    Lhu { rd: usize, rs1: usize, imm: i32 },
    Sltiu { rd: usize, rs1: usize, imm: i32 },

    // S-Format
    Sw { rs1: usize, rs2: usize, imm: i32 },
    Sb { rs1: usize, rs2: usize, imm: i32 },
    Sh { rs1: usize, rs2: usize, imm: i32 },

    // B-type
    Beq { rs1: usize, rs2: usize, offset: i32 },
    Bne { rs1: usize, rs2: usize, offset: i32 },
    Blt { rs1: usize, rs2: usize, offset: i32 },
    Bltu { rs1: usize, rs2: usize, offset: i32 },
    Bge { rs1: usize, rs2: usize, offset: i32 },
    Bgeu { rs1: usize, rs2: usize, offset: i32 },

    // J-Format
    Jal { rd: usize, offset: i32 },

    // U-Format
    Lui { rd: usize, imm: i32 },
    Auipc { rd: usize, imm: i32 },

    // For Debug
    Print { rs: usize },
}
