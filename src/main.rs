use riscviz::cpu::Cpu;
use riscviz::instruction::Instruction;

fn main() {
    let mut cpu = Cpu::default();
    let program = vec![
        Instruction::Addi {
            rd: 5,
            rs1: 0,
            imm: 5,
        },
        Instruction::Print { rs: 5 },
        Instruction::Addi {
            rd: 5,
            rs1: 5,
            imm: -1,
        },
        Instruction::Bne {
            rs1: 5,
            rs2: 0,
            offset: -2,
        },
        Instruction::Print { rs: 5 },
    ];

    cpu.load_program(program);

    loop {
        match cpu.execute_next() {
            Ok(true) => continue,
            Ok(false) => {
                println!("\nProgram finished");
                break;
            }
            Err(e) => {
                eprintln!("\nError: {}", e);
                break;
            }
        }
    }
}
