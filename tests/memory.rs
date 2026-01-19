use riscviz::cpu::Cpu;
use riscviz::instruction::Instruction;

#[test]
fn test_memory_ops() {
    let mut cpu = Cpu::default();
    cpu.load_program(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 100 },
        Instruction::Addi { rd: 2, rs1: 0, imm: 0 },
        Instruction::Sw { rs1: 1, rs2: 2, imm: 0 },
        Instruction::Lw { rd: 3, rs1: 2, imm: 0 },
    ]);
    while cpu.execute_next().unwrap() {}
    assert_eq!(cpu.regs[3], 100);
}
