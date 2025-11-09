use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;

enum AddrOperand {
    Imm(u16),
    Label(String),
}

enum ParsedInstr {
    Nop,
    Halt(u8),
    Mov(u8, u8),
    Add(u8, u8),
    Sub(u8, u8),
    Ldi(u8, u8),
    Jmp(AddrOperand),
    Jz(AddrOperand),
    Jnz(AddrOperand),
    Call(AddrOperand),
    Ret,
    Push(u8),
    Pop(u8),
    Ld(AddrOperand),
    St(AddrOperand),
}

struct InstrNode {
    line: usize,
    instr: ParsedInstr,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Input file path is required".to_string());
    }
    let input_path = &args[1];
    let source =
        fs::read_to_string(input_path).map_err(|e| format!("Failed to read input file: {e}"))?;
    let bytes = assemble(&source)?;
    let mut file =
        fs::File::create("out.bin").map_err(|e| format!("Failed to create output file: {e}"))?;
    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write output file: {e}"))?;
    Ok(())
}

fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut instrs: Vec<InstrNode> = Vec::new();
    let mut pc: u16 = 0;
    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        let mut part = line;
        if let Some(pos) = part.find(';') {
            part = &part[..pos];
        }
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut rest = trimmed;
        if let Some(colon_pos) = trimmed.find(':') {
            let (left, right_with_colon) = trimmed.split_at(colon_pos);
            let label = left.trim();
            if label.is_empty() {
                return Err(format!("Empty label at line {line_no}"));
            }
            if labels.contains_key(label) {
                return Err(format!("Duplicate label '{label}' at line {line_no}"));
            }
            labels.insert(label.to_string(), pc);
            rest = right_with_colon[1..].trim();
            if rest.is_empty() {
                continue;
            }
        }
        let instr = parse_instruction(rest, line_no)?;
        instrs.push(InstrNode {
            line: line_no,
            instr,
        });
        pc = pc
            .checked_add(2)
            .ok_or_else(|| "Program is too large".to_string())?;
    }
    let mut output = Vec::new();
    for node in &instrs {
        let word = encode_instruction(node, &labels)?;
        output.push((word >> 8) as u8);
        output.push((word & 0xFF) as u8);
    }
    Ok(output)
}

#[allow(clippy::too_many_lines)]
fn parse_instruction(text: &str, line: usize) -> Result<ParsedInstr, String> {
    let mut parts: Vec<String> = Vec::new();
    for part in text.split_whitespace() {
        let p = part.trim_matches(',');
        if !p.is_empty() {
            parts.push(p.to_string());
        }
    }
    if parts.is_empty() {
        return Err(format!("Empty instruction at line {line}"));
    }
    let mnemonic = parts[0].to_uppercase();
    let args = &parts[1..];
    match mnemonic.as_str() {
        "NOP" => {
            if !args.is_empty() {
                return Err(format!("NOP takes no arguments at line {line}"));
            }
            Ok(ParsedInstr::Nop)
        }
        "HALT" => {
            let code = if args.is_empty() {
                0u8
            } else if args.len() == 1 {
                parse_u8(&args[0], line)?
            } else {
                return Err(format!("HALT takes zero or one argument at line {line}"));
            };
            Ok(ParsedInstr::Halt(code))
        }
        "MOV" => {
            if args.len() != 2 {
                return Err(format!("MOV expects 2 arguments at line {line}"));
            }
            let dst = parse_reg(&args[0], line)?;
            let src = parse_reg(&args[1], line)?;
            Ok(ParsedInstr::Mov(dst, src))
        }
        "ADD" => {
            if args.len() != 2 {
                return Err(format!("ADD expects 2 arguments at line {line}"));
            }
            let dst = parse_reg(&args[0], line)?;
            let src = parse_reg(&args[1], line)?;
            Ok(ParsedInstr::Add(dst, src))
        }
        "SUB" => {
            if args.len() != 2 {
                return Err(format!("SUB expects 2 arguments at line {line}"));
            }
            let dst = parse_reg(&args[0], line)?;
            let src = parse_reg(&args[1], line)?;
            Ok(ParsedInstr::Sub(dst, src))
        }
        "LDI" => {
            if args.len() != 2 {
                return Err(format!("LDI expects 2 arguments at line {line}"));
            }
            let dst = parse_reg(&args[0], line)?;
            let imm = parse_u8(&args[1], line)?;
            Ok(ParsedInstr::Ldi(dst, imm))
        }
        "JMP" => {
            if args.len() != 1 {
                return Err(format!("JMP expects 1 argument at line {line}"));
            }
            Ok(ParsedInstr::Jmp(parse_addr_operand(&args[0])))
        }
        "JZ" => {
            if args.len() != 1 {
                return Err(format!("JZ expects 1 argument at line {line}"));
            }
            Ok(ParsedInstr::Jz(parse_addr_operand(&args[0])))
        }
        "JNZ" => {
            if args.len() != 1 {
                return Err(format!("JNZ expects 1 argument at line {line}"));
            }
            Ok(ParsedInstr::Jnz(parse_addr_operand(&args[0])))
        }
        "CALL" => {
            if args.len() != 1 {
                return Err(format!("CALL expects 1 argument at line {line}"));
            }
            Ok(ParsedInstr::Call(parse_addr_operand(&args[0])))
        }
        "RET" => {
            if !args.is_empty() {
                return Err(format!("RET takes no arguments at line {line}"));
            }
            Ok(ParsedInstr::Ret)
        }
        "PUSH" => {
            if args.len() != 1 {
                return Err(format!("PUSH expects 1 argument at line {line}"));
            }
            let r = parse_reg(&args[0], line)?;
            Ok(ParsedInstr::Push(r))
        }
        "POP" => {
            if args.len() != 1 {
                return Err(format!("POP expects 1 argument at line {line}"));
            }
            let r = parse_reg(&args[0], line)?;
            Ok(ParsedInstr::Pop(r))
        }
        "LD" => {
            if args.len() != 1 {
                return Err(format!("LD expects 1 argument at line {line}"));
            }
            Ok(ParsedInstr::Ld(parse_addr_operand(&args[0])))
        }
        "ST" => {
            if args.len() != 1 {
                return Err(format!("ST expects 1 argument at line {line}"));
            }
            Ok(ParsedInstr::St(parse_addr_operand(&args[0])))
        }
        _ => Err(format!("Unknown instruction '{mnemonic}' at line {line}")),
    }
}

fn parse_reg(s: &str, line: usize) -> Result<u8, String> {
    let t = s.to_uppercase();
    if let Some(num_str) = t.strip_prefix('R') {
        if num_str.is_empty() {
            return Err(format!("Invalid register '{s}' at line {line}"));
        }
        let n: u8 = num_str
            .parse()
            .map_err(|_| format!("Invalid register '{s}' at line {line}"))?;
        if n <= 7 {
            Ok(n)
        } else {
            Err(format!("Register index out of range '{s}' at line {line}"))
        }
    } else {
        Err(format!("Unknown register '{s}' at line {line}"))
    }
}

fn parse_u8(s: &str, line: usize) -> Result<u8, String> {
    let v = parse_u16_literal(s).ok_or_else(|| format!("Invalid number '{s}' at line {line}"))?;
    if v > 0xFF {
        return Err(format!(
            "Number '{s}' does not fit in 8 bits at line {line}"
        ));
    }
    Ok(v as u8)
}

fn parse_u16_literal(s: &str) -> Option<u16> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u16::from_str_radix(&s[2..], 16).ok()
    } else if s.chars().all(|c| c.is_ascii_digit()) {
        s.parse::<u16>().ok()
    } else {
        None
    }
}

fn parse_addr_operand(s: &str) -> AddrOperand {
    if let Some(v) = parse_u16_literal(s) {
        AddrOperand::Imm(v)
    } else {
        AddrOperand::Label(s.to_string())
    }
}

fn resolve_addr(
    op: &AddrOperand,
    labels: &HashMap<String, u16>,
    line: usize,
) -> Result<u16, String> {
    match op {
        AddrOperand::Imm(v) => Ok(*v),
        AddrOperand::Label(name) => {
            if let Some(addr) = labels.get(name) {
                Ok(*addr + 256)
            } else {
                Err(format!("Unknown label '{name}' at line {line}"))
            }
        }
    }
}

fn encode_instruction(node: &InstrNode, labels: &HashMap<String, u16>) -> Result<u16, String> {
    use ParsedInstr::{Add, Call, Halt, Jmp, Jnz, Jz, Ld, Ldi, Mov, Nop, Pop, Push, Ret, St, Sub};
    let line = node.line;
    match &node.instr {
        Nop => Ok(0x0000),
        Halt(code) => {
            let v = *code as u16;
            Ok(0x0100 | v)
        }
        Mov(dst, src) => Ok((0x1 << 12) | ((*dst as u16 & 0xF) << 4) | (*src as u16 & 0xF)),
        Add(dst, src) => {
            Ok((0x1 << 12) | (0x1 << 8) | ((*dst as u16 & 0xF) << 4) | (*src as u16 & 0xF))
        }
        Sub(dst, src) => {
            Ok((0x1 << 12) | (0x2 << 8) | ((*dst as u16 & 0xF) << 4) | (*src as u16 & 0xF))
        }
        Ldi(dst, imm) => {
            let imm16 = *imm as u16;
            Ok((0x2 << 12) | ((*dst as u16 & 0xF) << 8) | (imm16 & 0xFF))
        }
        Jmp(op) => {
            let addr = resolve_addr(op, labels, line)?;
            if addr > 0x0FFF {
                return Err(format!("Address out of range at line {line}"));
            }
            Ok((0x3 << 12) | (addr & 0x0FFF))
        }
        Jz(op) => {
            let addr = resolve_addr(op, labels, line)?;
            if addr > 0x0FFF {
                return Err(format!("Address out of range at line {line}"));
            }
            Ok((0x4 << 12) | (addr & 0x0FFF))
        }
        Jnz(op) => {
            let addr = resolve_addr(op, labels, line)?;
            if addr > 0x0FFF {
                return Err(format!("Address out of range at line {line}"));
            }
            Ok((0x5 << 12) | (addr & 0x0FFF))
        }
        Call(op) => {
            let addr = resolve_addr(op, labels, line)?;
            if addr > 0x0FFF {
                return Err(format!("Address out of range at line {line}"));
            }
            Ok((0x6 << 12) | (addr & 0x0FFF))
        }
        Ret => Ok(0x7000),
        Push(r) => Ok((0x7 << 12) | (0x1 << 8) | ((*r as u16 & 0xF) << 4)),
        Pop(r) => Ok((0x7 << 12) | (0x2 << 8) | ((*r as u16 & 0xF) << 4)),
        Ld(op) => {
            let addr = resolve_addr(op, labels, line)?;
            if addr > 0x0FFF {
                return Err(format!("Address out of range at line {line}"));
            }
            Ok((0x8 << 12) | (addr & 0x0FFF))
        }
        St(op) => {
            let addr = resolve_addr(op, labels, line)?;
            if addr > 0x0FFF {
                return Err(format!("Address out of range at line {line}"));
            }
            Ok((0x9 << 12) | (addr & 0x0FFF))
        }
    }
}
