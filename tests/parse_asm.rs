use riscviz::asm_parser::parse_line;
use riscviz::instruction::Instruction;

#[test]
fn test_r_type_instructions() {
    // Arithmetic
    assert!(matches!(parse_line("add x1, x2, x3").unwrap(), Instruction::Add { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("sub x10, x11, x12").unwrap(), Instruction::Sub { rd: 10, rs1: 11, rs2: 12 }));
    assert!(matches!(parse_line("mul x1, x2, x3").unwrap(), Instruction::Mul { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("mulh x1, x2, x3").unwrap(), Instruction::Mulh { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("mulhsu x1, x2, x3").unwrap(), Instruction::Mulhsu { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("mulhu x1, x2, x3").unwrap(), Instruction::Mulhu { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("div x1, x2, x3").unwrap(), Instruction::Div { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("divu x1, x2, x3").unwrap(), Instruction::Divu { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("rem x1, x2, x3").unwrap(), Instruction::Rem { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("remu x1, x2, x3").unwrap(), Instruction::Remu { rd: 1, rs1: 2, rs2: 3 }));

    // Logical
    assert!(matches!(parse_line("and x1, x2, x3").unwrap(), Instruction::And { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("or x1, x2, x3").unwrap(), Instruction::Or { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("xor x1, x2, x3").unwrap(), Instruction::Xor { rd: 1, rs1: 2, rs2: 3 }));

    // Shift
    assert!(matches!(parse_line("sll x1, x2, x3").unwrap(), Instruction::Sll { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("srl x1, x2, x3").unwrap(), Instruction::Srl { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("sra x1, x2, x3").unwrap(), Instruction::Sra { rd: 1, rs1: 2, rs2: 3 }));

    // Comparison
    assert!(matches!(parse_line("slt x1, x2, x3").unwrap(), Instruction::Slt { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("sltu x1, x2, x3").unwrap(), Instruction::Sltu { rd: 1, rs1: 2, rs2: 3 }));
}

#[test]
fn test_i_type_arithmetic() {
    assert!(matches!(parse_line("addi x1, x2, -10").unwrap(), Instruction::Addi { rd: 1, rs1: 2, imm: -10 }));
    assert!(matches!(parse_line("addi x1, x0, 0xFF").unwrap(), Instruction::Addi { rd: 1, rs1: 0, imm: 255 }));
    assert!(matches!(parse_line("andi x1, x2, 0x0F").unwrap(), Instruction::Andi { rd: 1, rs1: 2, imm: 15 }));
    assert!(matches!(parse_line("ori x1, x2, 100").unwrap(), Instruction::Ori { rd: 1, rs1: 2, imm: 100 }));
    assert!(matches!(parse_line("xori x1, x2, 50").unwrap(), Instruction::Xori { rd: 1, rs1: 2, imm: 50 }));
    assert!(matches!(parse_line("slli x1, x2, 5").unwrap(), Instruction::Slli { rd: 1, rs1: 2, imm: 5 }));
    assert!(matches!(parse_line("srli x1, x2, 3").unwrap(), Instruction::Srli { rd: 1, rs1: 2, imm: 3 }));
    assert!(matches!(parse_line("srai x1, x2, 4").unwrap(), Instruction::Srai { rd: 1, rs1: 2, imm: 4 }));
    assert!(matches!(parse_line("slti x1, x2, -5").unwrap(), Instruction::Slti { rd: 1, rs1: 2, imm: -5 }));
    assert!(matches!(parse_line("sltiu x1, x2, 10").unwrap(), Instruction::Sltiu { rd: 1, rs1: 2, imm: 10 }));
}

#[test]
fn test_load_instructions() {
    assert!(matches!(parse_line("lw x1, 8(x2)").unwrap(), Instruction::Lw { rd: 1, rs1: 2, imm: 8 }));
    assert!(matches!(parse_line("lw x1, (x2)").unwrap(), Instruction::Lw { rd: 1, rs1: 2, imm: 0 }));
    assert!(matches!(parse_line("lw x1, -4(x2)").unwrap(), Instruction::Lw { rd: 1, rs1: 2, imm: -4 }));
    assert!(matches!(parse_line("lb x1, 0(x2)").unwrap(), Instruction::Lb { rd: 1, rs1: 2, imm: 0 }));
    assert!(matches!(parse_line("lh x1, 2(x2)").unwrap(), Instruction::Lh { rd: 1, rs1: 2, imm: 2 }));
    assert!(matches!(parse_line("lbu x1, 1(x2)").unwrap(), Instruction::Lbu { rd: 1, rs1: 2, imm: 1 }));
    assert!(matches!(parse_line("lhu x1, 4(x2)").unwrap(), Instruction::Lhu { rd: 1, rs1: 2, imm: 4 }));
}

#[test]
fn test_jalr_instruction() {
    assert!(matches!(parse_line("jalr x1, x2, 0").unwrap(), Instruction::Jalr { rd: 1, rs1: 2, imm: 0 }));
    assert!(matches!(parse_line("jalr x0, x1, 4").unwrap(), Instruction::Jalr { rd: 0, rs1: 1, imm: 4 }));
}

#[test]
fn test_store_instructions() {
    assert!(matches!(parse_line("sw x1, 4(x2)").unwrap(), Instruction::Sw { rs1: 2, rs2: 1, imm: 4 }));
    assert!(matches!(parse_line("sw x1, -8(x2)").unwrap(), Instruction::Sw { rs1: 2, rs2: 1, imm: -8 }));
    assert!(matches!(parse_line("sw x1, (x2)").unwrap(), Instruction::Sw { rs1: 2, rs2: 1, imm: 0 }));
    assert!(matches!(parse_line("sb x1, 0(x2)").unwrap(), Instruction::Sb { rs1: 2, rs2: 1, imm: 0 }));
    assert!(matches!(parse_line("sh x1, 2(x2)").unwrap(), Instruction::Sh { rs1: 2, rs2: 1, imm: 2 }));
}

#[test]
fn test_branch_instructions() {
    assert!(matches!(parse_line("beq x1, x2, -4").unwrap(), Instruction::Beq { rs1: 1, rs2: 2, offset: -4 }));
    assert!(matches!(parse_line("bne x1, x2, 8").unwrap(), Instruction::Bne { rs1: 1, rs2: 2, offset: 8 }));
    assert!(matches!(parse_line("blt x1, x2, 12").unwrap(), Instruction::Blt { rs1: 1, rs2: 2, offset: 12 }));
    assert!(matches!(parse_line("bltu x1, x2, 16").unwrap(), Instruction::Bltu { rs1: 1, rs2: 2, offset: 16 }));
    assert!(matches!(parse_line("bge x1, x2, -8").unwrap(), Instruction::Bge { rs1: 1, rs2: 2, offset: -8 }));
    assert!(matches!(parse_line("bgeu x1, x2, 20").unwrap(), Instruction::Bgeu { rs1: 1, rs2: 2, offset: 20 }));
}

#[test]
fn test_jal_instruction() {
    assert!(matches!(parse_line("jal x1, 100").unwrap(), Instruction::Jal { rd: 1, offset: 100 }));
    assert!(matches!(parse_line("jal x1, -20").unwrap(), Instruction::Jal { rd: 1, offset: -20 }));
    assert!(matches!(parse_line("jal x0, 50").unwrap(), Instruction::Jal { rd: 0, offset: 50 }));
}

#[test]
fn test_u_type_instructions() {
    assert!(matches!(parse_line("lui x1, 0x12345").unwrap(), Instruction::Lui { rd: 1, imm: 0x12345 }));
    assert!(matches!(parse_line("lui x1, 1000").unwrap(), Instruction::Lui { rd: 1, imm: 1000 }));
    assert!(matches!(parse_line("auipc x1, 0x1000").unwrap(), Instruction::Auipc { rd: 1, imm: 0x1000 }));
    assert!(matches!(parse_line("auipc x1, 0").unwrap(), Instruction::Auipc { rd: 1, imm: 0 }));
}

#[test]
fn test_print_instruction() {
    assert!(matches!(parse_line("print x5").unwrap(), Instruction::Print { rs: 5 }));
    assert!(matches!(parse_line("print x0").unwrap(), Instruction::Print { rs: 0 }));
}

#[test]
fn test_various_formatting() {
    assert!(matches!(parse_line("addi x1, x0, 5 # load 5 into x1").unwrap(), Instruction::Addi { rd: 1, rs1: 0, imm: 5 }));
    assert!(matches!(parse_line("  add   x1,   x2,   x3  ").unwrap(), Instruction::Add { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("add\tx1,\tx2,\tx3").unwrap(), Instruction::Add { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("add x1,x2,x3").unwrap(), Instruction::Add { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("ADD x1, x2, x3").unwrap(), Instruction::Add { rd: 1, rs1: 2, rs2: 3 }));
    assert!(matches!(parse_line("AdDi x1, x2, 10").unwrap(), Instruction::Addi { rd: 1, rs1: 2, imm: 10 }));
    assert!(matches!(parse_line("addi x1, x0, -0x10").unwrap(), Instruction::Addi { rd: 1, rs1: 0, imm: -16 }));
}

#[test]
fn test_invalid_inputs() {
    assert!(parse_line("").is_none());
    assert!(parse_line("   ").is_none());
    assert!(parse_line("# this is a comment").is_none());
    assert!(parse_line("unknown x1, x2, x3").is_none());
    assert!(parse_line("add y1, x2, x3").is_none());
    assert!(parse_line("add x1, x2").is_none());
    assert!(parse_line("addi x1, x2, abc").is_none());
    assert!(parse_line("lw x1, 4[x2]").is_none());
    assert!(parse_line("lw x1, 4(x2").is_none());
}