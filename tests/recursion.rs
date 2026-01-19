use riscviz::cpu::Cpu;
use riscviz::instruction::Instruction;

#[test]
fn test_simple_recursion() {
    let mut cpu = Cpu::default();

    cpu.load_program(vec![
        // int f(int n) {
        //     if (n == 0) return 0;
        //     return 1 + f(n - 1);
        // }
        Instruction::Beq  { rs1: 10, rs2: 0, offset: 9 },  // 0: if n==0 goto base
        Instruction::Addi { rd: 2, rs1: 2, imm: -4 },      // 1: sp -= 4
        Instruction::Sw   { rs1: 1, rs2: 2, imm: 0 },      // 2: save ra
        Instruction::Addi { rd: 10, rs1: 10, imm: -1 },    // 3: n -= 1
        Instruction::Jal  { rd: 1, offset: -4 },           // 4: call f(n-1)
        Instruction::Lw   { rd: 1, rs1: 2, imm: 0 },       // 5: restore ra
        Instruction::Addi { rd: 2, rs1: 2, imm: 4 },       // 6: sp += 4
        Instruction::Addi { rd: 10, rs1: 10, imm: 1 },     // 7: result += 1
        Instruction::Jalr { rd: 0, rs1: 1, imm: 0 },       // 8: return

        // base case: return 0
        Instruction::Addi { rd: 10, rs1: 0, imm: 0 },      // 9
        Instruction::Jalr { rd: 0, rs1: 1, imm: 0 },       // 10

        // int main() { f(3); }
        Instruction::Addi { rd: 10, rs1: 0, imm: 3 },      // 11
        Instruction::Jal  { rd: 1, offset: -12 },          // 12: call f
    ]);

    cpu.pc = 11; // entry point
    while cpu.execute_next().unwrap() {}
    assert_eq!(cpu.regs[10], 3);
}
