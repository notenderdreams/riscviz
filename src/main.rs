use std::io::{self, Write};
use riscviz::asm_parser::{load_asm, parse_instruction};
use riscviz::cpu::Cpu;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct RegRow {
    #[tabled(rename = "Reg")]
    reg1: String,
    #[tabled(rename = "Val")]
    val1: i32,
    #[tabled(rename = "Reg")]
    reg2: String,
    #[tabled(rename = "Val")]
    val2: i32,
    #[tabled(rename = "Reg")]
    reg3: String,
    #[tabled(rename = "Val")]
    val3: i32,
    #[tabled(rename = "Reg")]
    reg4: String,
    #[tabled(rename = "Val")]
    val4: i32,
}

fn print_registers(cpu: &Cpu) {
    let mut rows = Vec::new();

    for row in 0..8 {
        rows.push(RegRow {
            reg1: format!("x{}", row),
            val1: cpu.regs[row],
            reg2: format!("x{}", row + 8),
            val2: cpu.regs[row + 8],
            reg3: format!("x{}", row + 16),
            val3: cpu.regs[row + 16],
            reg4: format!("x{}", row + 24),
            val4: cpu.regs[row + 24],
        });
    }

    let mut table = Table::new(rows);
    table.with(Style::rounded());
    println!("{table}");
}

fn main() {
    let mut cpu = Cpu::default();
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() == 2 {
        let program = load_asm(&args[1]).unwrap();
        cpu.load_program(program);
    }

    loop {
        print!("🐚 > ");
        io::stdout().flush().ok();

        let mut input = String::new();
        let n = match io::stdin().read_line(&mut input) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("[ERR] read: {e}");
                continue;
            }
        };
        if n == 0 {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Commands
        if input.starts_with('\\') {
            match input {
                "\\d" => print_registers(&cpu),
                "\\i" => cpu.print_instructions(),
                "\\f" => {
                    match cpu.execute_next() {
                        Ok(true) => println!("[OK]"),
                        Ok(false) => break,
                        Err(e) => eprintln!("[ERR] exec: {e}"),
                    }
                }
                "\\q" => break,
                _ => eprintln!("[ERR] unknown command: {input}"),
            }
            continue;
        }

        let Some(inst) = parse_instruction(input) else {
            eprintln!("[ERR] parse error: {input}");
            continue;
        };

        cpu.add_instruction(inst);

        match cpu.execute_next() {
            Ok(_) => {
                println!("[OK] {input}");
            }
            Err(e) => {
                eprintln!("[ERR] exec: {e}");
            }
        }
    }
}