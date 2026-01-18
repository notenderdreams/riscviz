use riscviz::cpu::Cpu;
use riscviz::instruction::Instruction;

fn main() {
    let mut cpu = Cpu::default();
    let program = vec![
        Instruction::Addi {
            rd: 6,
            rs1: 0,
            imm: 20,
        },
        Instruction::Addi {
            rd: 7,
            rs1: 0,
            imm: 30,
        },
        Instruction::Add {
            rd: 5,
            rs1: 6,
            rs2: 7,
        },
        Instruction::Sub {
            rd: 8,
            rs1: 6,
            rs2: 7,
        },
        Instruction::Mul {
            rd: 9,
            rs1: 6,
            rs2: 7,
        },
        Instruction::Div {
            rd: 10,
            rs1: 6,
            rs2: 7,
        },
        Instruction::Print { rs: 5 },
        Instruction::Print { rs: 8 },
        Instruction::Print { rs: 9 },
        Instruction::Print { rs: 10 },
        Instruction::Addi {
            rd: 2,
            rs1: 2,
            imm: -4,
        },
        Instruction::Print { rs: 2 },
        Instruction::Sw {
            rs1: 5,
            rs2: 2,
            imm: 0,
        },
        Instruction::Lw {
            rd: 12,
            rs1: 2,
            imm: 0,
        },
        Instruction::Addi {
            rd: 2,
            rs1: 2,
            imm: 4,
        },
        Instruction::Print { rs: 12 },
        Instruction::Print { rs: 2 },
        Instruction::Sw {rs1:12, rs2:2, imm: 0},
    ];

    for inst in program {
        print!("{:?}   \t", inst);
        if let Err(e) = cpu.execute(inst) {
            eprintln!("Error executing instruction: {}", e);
            break;
        }
        println!();
    }
}
