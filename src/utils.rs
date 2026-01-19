use crate::cpu::Cpu;
use crate::instruction::Instruction;

// Test Utils
pub fn run_program(program: Vec<Instruction>, entry: usize) -> Cpu {
    let mut cpu = Cpu::default();
    cpu.load_program(program);
    cpu.pc = entry;
    while cpu.execute_next().unwrap() {}
    cpu
}
#[macro_export]
macro_rules! run_program {
    ($program:expr) => {
        $crate::utils::run_program($program, 0)
    };
    ($program:expr, $entry:expr) => {
        $crate::utils::run_program($program, $entry)
    };
}
