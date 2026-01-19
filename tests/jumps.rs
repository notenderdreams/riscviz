use riscviz::instruction::Instruction;
use riscviz::run_program;

#[test]
fn test_non_leaf_function() {
    let cpu = run_program!(vec![
        // int bar() { return 41; }
        Instruction::Addi { rd: 10, rs1: 0, imm: 41 },   // 0
        Instruction::Jalr { rd: 0, rs1: 1, imm: 0 },     // 1: return

        // int foo() { return bar() + 1; }
        Instruction::Addi { rd: 2, rs1: 2, imm: -4 },    // 2: sp -= 4
        Instruction::Sw   { rs1: 1, rs2: 2, imm: 0 },    // 3: save ra
        Instruction::Jal  { rd: 1, offset: -4 },         // 4: call bar (→0)
        Instruction::Addi { rd: 10, rs1: 10, imm: 1 },   // 5: result += 1
        Instruction::Lw   { rd: 1, rs1: 2, imm: 0 },     // 6: restore ra
        Instruction::Addi { rd: 2, rs1: 2, imm: 4 },     // 7: sp += 4
        Instruction::Jalr { rd: 0, rs1: 1, imm: 0 },     // 8: return

        // int main() { foo(); }
        Instruction::Jal  { rd: 1, offset: -7 },         // 9: call foo (→2)
    ],9);

    assert_eq!(cpu.regs[10], 42);
}
