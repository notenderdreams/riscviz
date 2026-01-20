use riscviz::instruction::Instruction;
use riscviz::run_program;

#[test]
fn test_div_by_zero() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 5, rs1: 0, imm: 10 },
        Instruction::Div { rd: 1, rs1: 5, rs2: 0 }
    ]);
    assert_eq!(cpu.regs[1], -1); // DIV by zero returns -1
}

#[test]
fn test_divu_by_zero() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 5, rs1: 0, imm: 10 },
        Instruction::Divu { rd: 1, rs1: 5, rs2: 0 }
    ]);
    assert_eq!(cpu.regs[1], -1); // DIVU by zero returns -1 (0xFFFFFFFF)
}

#[test]
fn test_rem_by_zero() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 5, rs1: 0, imm: 10 },
        Instruction::Rem { rd: 1, rs1: 5, rs2: 0 }
    ]);
    assert_eq!(cpu.regs[1], 10); // REM by zero returns dividend
}

#[test]
fn test_remu_by_zero() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 5, rs1: 0, imm: 10 },
        Instruction::Remu { rd: 1, rs1: 5, rs2: 0 }
    ]);
    assert_eq!(cpu.regs[1], 10); // REMU by zero returns dividend
}

#[test]
fn test_mul() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 10 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
        Instruction::Mul { rd: 3, rs1: 1, rs2: 2 },
    ]);
    assert_eq!(cpu.regs[3], 50);
}

#[test]
fn test_mulh_positive() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 0x7FFF }, // 32767
        Instruction::Addi { rd: 2, rs1: 0, imm: 0x8000 }, // 32768
        Instruction::Mulh { rd: 3, rs1: 1, rs2: 2 },
    ]);
    // 32767 * 32768 = 1,073,709,056 = 0x3FFF8000
    // Upper 32 bits = 0
    assert_eq!(cpu.regs[3], 0);
}

#[test]
fn test_mulh_negative() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -100 },
        Instruction::Addi { rd: 2, rs1: 0, imm: -200 },
        Instruction::Mulh { rd: 3, rs1: 1, rs2: 2 },
    ]);
    // -100 * -200 = 20000, upper bits = 0
    assert_eq!(cpu.regs[3], 0);
}

#[test]
fn test_mulhsu() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -1 }, // -1 (signed)
        Instruction::Addi { rd: 2, rs1: 0, imm: -1 }, // 0xFFFFFFFF (as unsigned)
        Instruction::Mulhsu { rd: 3, rs1: 1, rs2: 2 },
    ]);
    // -1 * 0xFFFFFFFF = -0xFFFFFFFF
    // Upper 32 bits = -1
    assert_eq!(cpu.regs[3], -1);
}

#[test]
fn test_mulhu() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -1 }, // 0xFFFFFFFF as unsigned
        Instruction::Addi { rd: 2, rs1: 0, imm: -1 }, // 0xFFFFFFFF as unsigned
        Instruction::Mulhu { rd: 3, rs1: 1, rs2: 2 },
    ]);
    // 0xFFFFFFFF * 0xFFFFFFFF = 0xFFFFFFFE00000001
    // Upper 32 bits = 0xFFFFFFFE = -2 as i32
    assert_eq!(cpu.regs[3], -2);
}

#[test]
fn test_div() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 20 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 6 },
        Instruction::Div { rd: 3, rs1: 1, rs2: 2 },
    ]);
    assert_eq!(cpu.regs[3], 3); // 20 / 6 = 3
}

#[test]
fn test_div_negative() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -20 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 6 },
        Instruction::Div { rd: 3, rs1: 1, rs2: 2 },
    ]);
    assert_eq!(cpu.regs[3], -3); // -20 / 6 = -3 (rounds toward zero)
}

#[test]
fn test_divu() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -1 }, // 0xFFFFFFFF as unsigned
        Instruction::Addi { rd: 2, rs1: 0, imm: 2 },
        Instruction::Divu { rd: 3, rs1: 1, rs2: 2 },
    ]);
    // 0xFFFFFFFF / 2 = 0x7FFFFFFF = 2147483647
    assert_eq!(cpu.regs[3], 0x7FFFFFFF);
}

#[test]
fn test_rem() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 20 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 6 },
        Instruction::Rem { rd: 3, rs1: 1, rs2: 2 },
    ]);
    assert_eq!(cpu.regs[3], 2); // 20 % 6 = 2
}

#[test]
fn test_rem_negative() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -20 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 6 },
        Instruction::Rem { rd: 3, rs1: 1, rs2: 2 },
    ]);
    assert_eq!(cpu.regs[3], -2); // -20 % 6 = -2 (sign follows dividend)
}

#[test]
fn test_remu() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -1 }, // 0xFFFFFFFF as unsigned
        Instruction::Addi { rd: 2, rs1: 0, imm: 10 },
        Instruction::Remu { rd: 3, rs1: 1, rs2: 2 },
    ]);
    // 0xFFFFFFFF % 10 = 5
    assert_eq!(cpu.regs[3], 5);
}

#[test]
fn test_arithmetic_ops() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 10 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
        Instruction::Add { rd: 3, rs1: 1, rs2: 2 },
        Instruction::Sub { rd: 4, rs1: 1, rs2: 2 },
        Instruction::Mul { rd: 5, rs1: 1, rs2: 2 },
        Instruction::Div { rd: 6, rs1: 1, rs2: 2 },
        Instruction::And { rd: 7, rs1: 1, rs2: 2 },
        Instruction::Or  { rd: 8, rs1: 1, rs2: 2 },
        Instruction::Xor { rd: 9, rs1: 1, rs2: 2 },
        Instruction::Sll { rd: 10, rs1: 1, rs2: 2 },
        Instruction::Srl { rd: 11, rs1: 1, rs2: 2 },
        Instruction::Sra { rd: 12, rs1: 1, rs2: 2 },
    ]);

    assert_eq!(cpu.regs[3], 15);
    assert_eq!(cpu.regs[4], 5);
    assert_eq!(cpu.regs[5], 50);
    assert_eq!(cpu.regs[6], 2);
    assert_eq!(cpu.regs[7], 0);  // 10 & 5
    assert_eq!(cpu.regs[8], 15); // 10 | 5
    assert_eq!(cpu.regs[9], 15); // 10 ^ 5
    assert_eq!(cpu.regs[10], 320); // 10 << 5
    assert_eq!(cpu.regs[11], 0);  // 10 >> 5 logical
    assert_eq!(cpu.regs[12], 0);  // 10 >> 5 arithmetic
}

#[test]
fn test_x0_immutable() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 0, rs1: 0, imm: 999 },
    ]);
    assert_eq!(cpu.regs[0], 0);
}