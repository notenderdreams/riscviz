use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use crate::instruction::Instruction;

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<String, usize>,
}

fn parse_reg(reg: &str) -> Option<usize> {
    let reg = reg.trim();
    if reg.starts_with('x') {
        reg[1..].parse::<usize>().ok()
    } else {
        None
    }
}

fn parse_imm(imm: &str) -> Option<i32> {
    let imm = imm.trim();
    if imm.starts_with("0x") || imm.starts_with("0X") {
        i32::from_str_radix(&imm[2..], 16).ok()
    } else if imm.starts_with("-0x") || imm.starts_with("-0X") {
        i32::from_str_radix(&imm[3..], 16).ok().map(|v| -v)
    } else {
        i32::from_str(imm).ok()
    }
}

fn parse_mem_operand(mem: &str) -> Option<(i32, usize)> {
    let mem = mem.trim();
    let start = mem.find('(')?;
    let end = mem.find(')')?;
    let imm = if start == 0 { 0 } else { parse_imm(&mem[..start])? };
    let rs = parse_reg(&mem[start + 1..end])?;
    Some((imm, rs))
}

macro_rules! parse_r_type {
    ($tokens:ident, $variant:ident) => {
        Some(Instruction::$variant {
            rd: parse_reg($tokens.next()?)?,
            rs1: parse_reg($tokens.next()?)?,
            rs2: parse_reg($tokens.next()?)?,
        })
    };
}

macro_rules! parse_i_type {
    ($tokens:ident, $variant:ident) => {
        Some(Instruction::$variant {
            rd: parse_reg($tokens.next()?)?,
            rs1: parse_reg($tokens.next()?)?,
            imm: parse_imm($tokens.next()?)?,
        })
    };
}

macro_rules! parse_load {
    ($tokens:ident, $variant:ident) => {{
        let rd = parse_reg($tokens.next()?)?;
        let (imm, rs1) = parse_mem_operand($tokens.next()?)?;
        Some(Instruction::$variant { rd, rs1, imm })
    }};
}

macro_rules! parse_store {
    ($tokens:ident, $variant:ident) => {{
        let rs2 = parse_reg($tokens.next()?)?;
        let (imm, rs1) = parse_mem_operand($tokens.next()?)?;
        Some(Instruction::$variant { rs1, rs2, imm })
    }};
}

macro_rules! parse_b_type {
    ($tokens:ident, $variant:ident) => {
        Some(Instruction::$variant {
            rs1: parse_reg($tokens.next()?)?,
            rs2: parse_reg($tokens.next()?)?,
            offset:0,
        })
    };
}

macro_rules! parse_u_type {
    ($tokens:ident, $variant:ident) => {
        Some(Instruction::$variant {
            rd: parse_reg($tokens.next()?)?,
            imm: parse_imm($tokens.next()?)?,
        })
    };
}

macro_rules! parse_j_type {
    ($tokens:ident, $variant:ident) => {
        Some(Instruction::$variant {
            rd: parse_reg($tokens.next()?)?,
            offset: 0,
        })
    };
}

pub fn parse_instruction(line: &str) -> Option<Instruction> {
    let line = line.split('#').next()?.trim();
    if line.is_empty() {
        return None;
    }

    let mut tokens = line
        .split(|c| c == ',' || c == ' ' || c == '\t')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    let mnemonic = tokens.next()?.to_lowercase();

    match mnemonic.as_str() {
        // R-Format
        "add" => parse_r_type!(tokens, Add),
        "sub" => parse_r_type!(tokens, Sub),
        "mul" => parse_r_type!(tokens, Mul),
        "mulh" => parse_r_type!(tokens, Mulh),
        "mulhsu" => parse_r_type!(tokens, Mulhsu),
        "mulhu" => parse_r_type!(tokens, Mulhu),
        "div" => parse_r_type!(tokens, Div),
        "divu" => parse_r_type!(tokens, Divu),
        "rem" => parse_r_type!(tokens, Rem),
        "remu" => parse_r_type!(tokens, Remu),
        "and" => parse_r_type!(tokens, And),
        "or" => parse_r_type!(tokens, Or),
        "xor" => parse_r_type!(tokens, Xor),
        "sll" => parse_r_type!(tokens, Sll),
        "srl" => parse_r_type!(tokens, Srl),
        "sra" => parse_r_type!(tokens, Sra),
        "slt" => parse_r_type!(tokens, Slt),
        "sltu" => parse_r_type!(tokens, Sltu),

        // I-Format (arithmetic/logic)
        "addi" => parse_i_type!(tokens, Addi),
        "andi" => parse_i_type!(tokens, Andi),
        "ori" => parse_i_type!(tokens, Ori),
        "xori" => parse_i_type!(tokens, Xori),
        "slli" => parse_i_type!(tokens, Slli),
        "srli" => parse_i_type!(tokens, Srli),
        "srai" => parse_i_type!(tokens, Srai),
        "slti" => parse_i_type!(tokens, Slti),
        "sltiu" => parse_i_type!(tokens, Sltiu),
        "jalr" => parse_i_type!(tokens, Jalr),

        // I-Format (loads)
        "lb" => parse_load!(tokens, Lb),
        "lh" => parse_load!(tokens, Lh),
        "lw" => parse_load!(tokens, Lw),
        "lbu" => parse_load!(tokens, Lbu),
        "lhu" => parse_load!(tokens, Lhu),

        // S-Format (stores)
        "sb" => parse_store!(tokens, Sb),
        "sh" => parse_store!(tokens, Sh),
        "sw" => parse_store!(tokens, Sw),

        // B-Format
        "beq" => parse_b_type!(tokens, Beq),
        "bne" => parse_b_type!(tokens, Bne),
        "blt" => parse_b_type!(tokens, Blt),
        "bltu" => parse_b_type!(tokens, Bltu),
        "bge" => parse_b_type!(tokens, Bge),
        "bgeu" => parse_b_type!(tokens, Bgeu),

        // J-Format
        "jal" => parse_j_type!(tokens, Jal),

        // U-Format
        "lui" => parse_u_type!(tokens, Lui),
        "auipc" => parse_u_type!(tokens, Auipc),

        // Debug
        "print" => Some(Instruction::Print {
            rs: parse_reg(tokens.next()?)?,
        }),

        _ => None,
    }
}

pub fn load_asm(path: &str)->io::Result<Program> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut instructions:Vec<Instruction> = Vec::new();
    let mut labels: HashMap<String,usize>= HashMap::new();
    let mut patch_list: HashMap<String,Vec<usize>> = HashMap::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let trimmed = line.split('#').next().unwrap_or("").trim();
        if trimmed.is_empty() {
            continue
        }

        let pos = instructions.len();
        if trimmed.ends_with(':') {
            let label = trimmed.trim_end_matches(':').to_string();
            labels.insert(label.clone(),pos);

            if let Some(waiting) = patch_list.remove(&label){
                for idx in waiting {
                    let offset = pos as i32 - idx as i32;
                    instructions[idx].patch_label(offset);
                }
            }
            continue;
        }

        let mut inst = match parse_instruction(trimmed) {
            Some(inst) => inst,
            None => continue,
        };

        let last_token = trimmed
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty())
            .last();

        if let Some(label_ref) = last_token {
            if labels.contains_key(label_ref) {
                let offset = labels[label_ref] as i32 - pos as i32;
                inst.patch_label(offset);
            } else if matches!(inst,
                Instruction::Beq {..} | Instruction::Bne {..} |
                Instruction::Blt {..} | Instruction::Bltu {..} |
                Instruction::Bge {..} | Instruction::Bgeu {..} |
                Instruction::Jal {..}){

                patch_list.entry(label_ref.to_string()).or_default().push(pos);
            }
        }
        instructions.push(inst);
    }

    Ok(Program{instructions, labels})
}
