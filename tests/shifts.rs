use riscviz::cpu::Cpu;
use riscviz::instruction::Instruction;

#[test]
fn test_slti_basic() {
    let mut cpu = Cpu::default();
    cpu.load_program(vec![
        Instruction::Addi { rd: 1, rs1: 0, imm: 5 },   // x1 = 5
        Instruction::Slti { rd: 2, rs1: 1, imm: 10 },  // 5 < 10 → x2 = 1
        Instruction::Slti { rd: 3, rs1: 1, imm: 3 },   // 5 < 3 → x3 = 0
    ]);

    while cpu.execute_next().unwrap() {}

    assert_eq!(cpu.regs[2], 1);
    assert_eq!(cpu.regs[3], 0);
}
