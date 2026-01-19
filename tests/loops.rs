use riscviz::cpu::Cpu;
use riscviz::instruction::Instruction;

#[test]
fn test_while_loop() {
    let mut cpu = Cpu::default();

    cpu.load_program(vec![
        // int i = 0;
        // while (i < 10) {
        //     i++;
        // }
        Instruction::Addi { rd: 1, rs1: 0, imm: 0 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 10 },
        Instruction::Bge { rs1: 1, rs2: 2, offset: 2 },
        Instruction::Addi { rd: 1, rs1: 1, imm: 1 },
        Instruction::Blt { rs1: 1, rs2: 2, offset: -2 },
    ]);

    while cpu.execute_next().unwrap() {}
    assert_eq!(cpu.regs[1], 10);
}

#[test]
fn test_for_loop() {
    let mut cpu = Cpu::default();

    cpu.load_program(vec![
        // int sum = 0;
        // for (int i = 0; i < 5; i++) {
        //     sum += i;
        // }
        Instruction::Addi { rd: 1, rs1: 0, imm: 0 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 5 },
        Instruction::Addi { rd: 3, rs1: 0, imm: 0 },
        Instruction::Bge { rs1: 1, rs2: 2, offset: 3 },
        Instruction::Add { rd: 3, rs1: 3, rs2: 1 },
        Instruction::Addi { rd: 1, rs1: 1, imm: 1 },
        Instruction::Blt { rs1: 1, rs2: 2, offset: -3 },
    ]);

    while cpu.execute_next().unwrap() {}
    assert_eq!(cpu.regs[3], 10);
}
