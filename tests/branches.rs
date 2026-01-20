use riscviz::cpu::Cpu;
use riscviz::instruction::Instruction;
use riscviz::run_program;

#[test]
fn test_branch_not_equal_loop() {
    let mut cpu = Cpu::default();
    cpu.load_program(vec![
        Instruction::Addi { rd: 5, rs1: 0, imm: 3 },
        Instruction::Addi { rd: 5, rs1: 5, imm: -1 },
        Instruction::Bne { rs1: 5, rs2: 0, offset: -1 },
    ]);

    let mut steps = 0;
    while cpu.execute_next().unwrap() && steps < 20 { steps += 1; }
    assert_eq!(cpu.regs[5], 0);
}

#[test]
fn test_branch_equal() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 5 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
        Instruction::Beq { rs1: 1, rs2: 2, offset: 2 },
        Instruction::Addi { rd: 3, rs1: 0, imm: 99 },
        Instruction::Addi { rd: 4, rs1: 0, imm: 42 },
    ]);
    assert_eq!(cpu.regs[3], 0);
    assert_eq!(cpu.regs[4], 42);
}
#[test]
fn test_bltu_bgeu() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: -1 },    // r1 = 0xFFFFFFFF
        Instruction::Addi { rd: 2, rs1: 0, imm: 5 },     // r2 = 5
        Instruction::Bltu { rs1: 1, rs2: 2, offset: 2 }, // 0xFFFFFFFF < 5? no, skip
        Instruction::Addi { rd: 3, rs1: 0, imm: 100 },   // r3 = 100 (executed)
        Instruction::Bltu { rs1: 2, rs2: 1, offset: 2 }, // 5 < 0xFFFFFFFF? yes, jump
        Instruction::Addi { rd: 4, rs1: 0, imm: 100 },   // r4 = 100 (skipped)
        Instruction::Addi { rd: 5, rs1: 0, imm: 200 },   // r5 = 200 (executed)
    ]);

    assert_eq!(cpu.regs[3], 100);
    assert_eq!(cpu.regs[4], 0);
    assert_eq!(cpu.regs[5], 200);
}

#[test]
fn test_bgeu() {
    let cpu = run_program!(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 10 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
        Instruction::Bgeu { rs1: 1, rs2: 2, offset: 2 }, // 10 >= 5? yes, jump
        Instruction::Addi { rd: 3, rs1: 0, imm: 99 },    // skipped
        Instruction::Addi { rd: 4, rs1: 0, imm: 42 },
    ]);

    assert_eq!(cpu.regs[3], 0);
    assert_eq!(cpu.regs[4], 42);
}