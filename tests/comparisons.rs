use riscviz::instruction::Instruction;
use riscviz::run_program;

#[test]
fn test_slt_sltu() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -1 },  // r1 = -1
        Instruction::Addi { rd: 2, rs1: 0, imm: 5 },   // r2 = 5
        Instruction::Slt { rd: 3, rs1: 1, rs2: 2 },    // -1 < 5 (signed) = 1
        Instruction::Sltu { rd: 4, rs1: 1, rs2: 2 },   // 0xFFFFFFFF < 5 (unsigned) = 0
        Instruction::Slt { rd: 5, rs1: 2, rs2: 1 },    // 5 < -1 (signed) = 0
        Instruction::Sltu { rd: 6, rs1: 2, rs2: 1 },   // 5 < 0xFFFFFFFF (unsigned) = 1
    ]);

    assert_eq!(cpu.regs[3], 1);
    assert_eq!(cpu.regs[4], 0);
    assert_eq!(cpu.regs[5], 0);
    assert_eq!(cpu.regs[6], 1);
}

#[test]
fn test_slti_sltiu() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -1 },
        Instruction::Slti { rd: 2, rs1: 1, imm: 5 },     // -1 < 5 (signed) = 1
        Instruction::Sltiu { rd: 3, rs1: 1, imm: 5 },    // 0xFFFFFFFF < 5 (unsigned) = 0
        Instruction::Addi { rd: 4, rs1: 0, imm: 3 },
        Instruction::Slti { rd: 5, rs1: 4, imm: -1 },    // 3 < -1 (signed) = 0
        Instruction::Sltiu { rd: 6, rs1: 4, imm: -1 },   // 3 < 0xFFFFFFFF (unsigned) = 1
    ]);

    assert_eq!(cpu.regs[2], 1);
    assert_eq!(cpu.regs[3], 0);
    assert_eq!(cpu.regs[5], 0);
    assert_eq!(cpu.regs[6], 1);
}