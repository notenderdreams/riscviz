use riscviz::instruction::Instruction;
use riscviz::run_program;

#[test]
fn test_lui() {
    let cpu = run_program!(vec![
        Instruction::Lui { rd: 1, imm: 0x12345 },
        Instruction::Lui { rd: 2, imm: 0xFFFFF },
        Instruction::Lui { rd: 3, imm: 1 },
    ]);

    assert_eq!(cpu.regs[1], 0x12345000);
    assert_eq!(cpu.regs[2], 0xFFFFF000u32 as i32);
    assert_eq!(cpu.regs[3], 0x1000);
}

#[test]
fn test_auipc() {
    let cpu = run_program!(vec![
        Instruction::Auipc { rd: 1, imm: 0 },
        Instruction::Auipc { rd: 2, imm: 1 },
        Instruction::Auipc { rd: 3, imm: -1 },
    ]);

    assert_eq!(cpu.regs[1], 0);
    assert_eq!(cpu.regs[2], 0x1000 + 1);
    assert_eq!(cpu.regs[3], (0xFFFFF000u32 as i32) + 2);
}

#[test]
fn test_lui_auipc_combo() {
    let cpu = run_program!(vec![
        Instruction::Lui { rd: 1, imm: 0x10000 },      // upper 20 bits
        Instruction::Addi { rd: 1, rs1: 1, imm: 0x234 }, // lower 12 bits
        Instruction::Auipc { rd: 2, imm: 0 },
        Instruction::Addi { rd: 2, rs1: 2, imm: 8 },
    ]);

    assert_eq!(cpu.regs[1], 0x10000234);
    assert_eq!(cpu.regs[2], 2 + 8);
}