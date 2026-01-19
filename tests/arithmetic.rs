use riscviz::cpu::Cpu;
use riscviz::instruction::Instruction;
use riscviz::cpu::CpuError;

#[test]
fn test_div_by_zero() {
    let mut cpu = Cpu::default();
    cpu.load_program(vec![
        Instruction::Div { rd: 1, rs1: 5, rs2: 0 }
    ]);
    let result = cpu.execute_next();
    assert!(matches!(result, Err(CpuError::DivisionByZero)));
}

#[test]
fn test_arithmetic_ops() {
    let mut cpu = Cpu::default();
    cpu.load_program(vec![
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

    while cpu.execute_next().unwrap() {}

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
    let mut cpu = Cpu::default();
    cpu.load_program(vec![
        Instruction::Addi { rd: 0, rs1: 0, imm: 999 },
    ]);
    cpu.execute_next().unwrap();
    assert_eq!(cpu.regs[0], 0);
}
