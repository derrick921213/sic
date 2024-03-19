mod line;
mod optables;
use line::*;
use optables::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let program = args.next().unwrap();
    match args.next() {
        Some(filename) => {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            let mut symbol_table: HashMap<String, u32> = HashMap::new();
            let optab = Optab::new();
            let mut line_struct: Vec<Line> = Vec::new();
            let mut locctr: u32 = 0;
            let mut start = 0;
            let mut end = 0;
            for (index, line) in reader.lines().enumerate() {
                let mut line = line?;
                if line.trim().is_empty()
                    || line.trim_start().starts_with('.')
                    || line.starts_with('.')
                {
                    continue;
                }
                if let Some(index) = line.find('.') {
                    line = line[..index].trim_end().to_string();
                }
                let mut tokens = line.split('\t').collect::<Vec<&str>>();
                if tokens.len() > 3 {
                    tokens.truncate(3);
                }
                if tokens[0].is_empty() && tokens[1] == "RSUB" {
                    tokens.append(&mut vec![""]);
                }
                if tokens.len() != 3 {
                    let msg = format!("錯誤: {} 行的格式不正确。", index + 1);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
                }
                let mut opcode = tokens[1].to_string();
                let use_extended_format = opcode.starts_with('+');
                if use_extended_format {
                    opcode = opcode.trim_start_matches('+').to_string();
                }
                match optab.is_opcode(&opcode) {
                    Some(obj) => {
                        let mut formatter = obj.fmt.clone();
                        if use_extended_format {
                            if let FormatDirective::Format(Format::FMT3_4) = obj.fmt {
                                formatter = FormatDirective::Format(Format::FMT4);
                            } else {
                                formatter = obj.fmt.clone();
                            }
                        }
                        if obj.code == 0x107 {
                            locctr = tokens[2].parse::<u32>().unwrap();
                            start = locctr;
                        }
                        if obj.code == 0x108 {
                            end = locctr;
                        }
                        let line = Line::new(
                            locctr,
                            tokens
                                .first()
                                .map(|s| s.to_string())
                                .filter(|s| !s.is_empty()),
                            tokens[1].to_string(),
                            tokens
                                .get(2)
                                .map(|s| s.to_string())
                                .filter(|s| !s.is_empty()),
                            tokens
                                .get(3)
                                .map(|s| s.to_string())
                                .filter(|s| !s.is_empty()),
                            obj.code,
                            formatter.clone(),
                            AddrMode::Simple,
                        );
                        if let Some(symbol) = line.get_symbol() {
                            if symbol_table.contains_key(symbol) {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("Duplicate symbol {}", symbol),
                                ));
                            } else {
                                symbol_table.insert(symbol.clone(), locctr);
                            }
                        }
                        locctr += match tokens[1] {
                            "WORD" => 3,
                            "RESW" => 3 * tokens[2].parse::<u32>().unwrap(),
                            "RESB" => tokens[2].parse::<u32>().unwrap(),
                            "BYTE" => {
                                if tokens[2].starts_with('C') {
                                    tokens[2].len() as u32 - 3
                                } else if tokens[2].starts_with('X') {
                                    ((tokens[2].len() as u32 - 3) + 1) / 2
                                } else {
                                    0
                                }
                            }
                            _ => 0,
                        };
                        locctr += match formatter {
                            FormatDirective::Format(Format::FMT1) => 1,
                            FormatDirective::Format(Format::FMT2) => 2,
                            FormatDirective::Format(Format::FMT3_4) => 3,
                            FormatDirective::Format(Format::FMT4) => 4,
                            _ => 0,
                        };

                        line_struct.push(line);
                    }
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Line {}: {} is not a valid opcode", index + 1, tokens[1]),
                        ));
                    }
                }
            }
            for line in line_struct.iter() {
                println!(
                    "{:06}\t{:8}\t{:8}\t{:8}",
                    line.get_memory(),
                    line.get_symbol().unwrap_or(&"".to_string()),
                    line.get_op(),
                    line.get_operand1().unwrap_or(&"".to_string())
                )
            }
            let total = end - start;
            println!("\nProgram length = {}\n", total);
            for (k, v) in symbol_table.iter() {
                println!("{:10}: {:06}", k, v);
            }
        }
        None => println!("Usage: {} <filename.asm>", program),
    }
    Ok(())
}
