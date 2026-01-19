use riscviz::instruction::Instruction;
use riscviz::run_program;

#[test]
fn test_slti_basic() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 5 },   // x1 = 5
        Instruction::Slti { rd: 2, rs1: 1, imm: 10 },  // 5 < 10 → x2 = 1
        Instruction::Slti { rd: 3, rs1: 1, imm: 3 },   // 5 < 3 → x3 = 0
    ]);

    assert_eq!(cpu.regs[2], 1);
    assert_eq!(cpu.regs[3], 0);
}
#[test]
fn test_sll_slli() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 5 },      // x1 = 5
        Instruction::Addi { rd: 2, rs1: 0, imm: 3 },      // x2 = 3
        Instruction::Sll  { rd: 3, rs1: 1, rs2: 2 },      // x3 = 5 << 3 = 40
        Instruction::Slli { rd: 4, rs1: 1, imm: 2 },    // x4 = 5 << 2 = 20
    ]);

    assert_eq!(cpu.regs[3], 40);
    assert_eq!(cpu.regs[4], 20);
}

#[test]
fn test_srl_srli() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 40 },     // x1 = 40
        Instruction::Addi { rd: 2, rs1: 0, imm: 3 },      // x2 = 3
        Instruction::Srl  { rd: 3, rs1: 1, rs2: 2 },      // x3 = 40 >> 3 = 5
        Instruction::Srli { rd: 4, rs1: 1, imm: 2 },    // x4 = 40 >> 2 = 10
    ]);

    assert_eq!(cpu.regs[3], 5);
    assert_eq!(cpu.regs[4], 10);
}

#[test]
fn test_sra_srai() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -16 },    // x1 = -16
        Instruction::Addi { rd: 2, rs1: 0, imm: 2 },      // x2 = 2
        Instruction::Sra  { rd: 3, rs1: 1, rs2: 2 },      // x3 = -16 >> 2 = -4
        Instruction::Srai { rd: 4, rs1: 1, imm: 3 },    // x4 = -16 >> 3 = -2
    ]);

    assert_eq!(cpu.regs[3], -4);
    assert_eq!(cpu.regs[4], -2);
}
