use riscviz::instruction::Instruction;
use riscviz::run_program;

#[test]
fn test_word_load_store() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 100 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 0 },
        Instruction::Sw { rs1: 1, rs2: 2, imm: 0 },
        Instruction::Lw { rd: 3, rs1: 2, imm: 0 },
    ]);
    assert_eq!(cpu.regs[3], 100);
}
#[test]
fn test_halfword_load_store_signed_unsigned() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -300 }, // value to store (i16)
        Instruction::Addi { rd: 2, rs1: 0, imm: 4 },    // base addr
        Instruction::Sh { rs1: 1, rs2: 2, imm: 0 },    // store halfword
        Instruction::Lh { rd: 3, rs1: 2, imm: 0 },     // load signed halfword
        Instruction::Lhu { rd: 4, rs1: 2, imm: 0 },    // load unsigned halfword
    ]);
    assert_eq!(cpu.regs[3], -300); // sign-extended
    assert_eq!(cpu.regs[4], 0xFED4); // zero-extended
}
#[test]
fn test_byte_load_store_signed_unsigned() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -50 }, // value to store (i8)
        Instruction::Addi { rd: 2, rs1: 0, imm: 10 },  // base addr
        Instruction::Sb { rs1: 1, rs2: 2, imm: 0 },   // store byte
        Instruction::Lb { rd: 3, rs1: 2, imm: 0 },    // load signed byte
        Instruction::Lbu { rd: 4, rs1: 2, imm: 0 },   // load unsigned byte
    ]);
    assert_eq!(cpu.regs[3], -50); // sign-extended
    assert_eq!(cpu.regs[4], 206); // 0xCE zero-extended
}
#[test]
fn test_multiple_memory_ops() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 0x12345678 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 20 },
        Instruction::Sw { rs1: 1, rs2: 2, imm: 0 },
        Instruction::Lw { rd: 3, rs1: 2, imm: 0 },
        Instruction::Sh { rs1: 3, rs2: 2, imm: 4 },
        Instruction::Lh { rd: 4, rs1: 2, imm: 4 },
        Instruction::Sb { rs1: 3, rs2: 2, imm: 6 },
        Instruction::Lb { rd: 5, rs1: 2, imm: 6 },
    ]);

    assert_eq!(cpu.regs[3], 0x12345678);
    assert_eq!(cpu.regs[4], 0x5678_i16 as i32);
    assert_eq!(cpu.regs[5], 0x78_i8 as i32);
}
