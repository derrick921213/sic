mod line;
mod optables;
use line::*;
use optables::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
struct FileWriter {
    writer: BufWriter<File>,
}
impl FileWriter {
    fn new(path: &str) -> io::Result<Self> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        Ok(Self { writer })
    }
    fn write(&mut self, content: &str) -> io::Result<()> {
        self.writer.write_all(content.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let program = args.next().unwrap();
    let mut base_register: u32 = 0;
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
                let mut tokens = line.split(['\t', ',']).collect::<Vec<&str>>();
                if tokens.len() > 3 {
                    tokens.truncate(3);
                }
                if let Some(symbol) = symbol_table.get(tokens[1]) {
                    base_register = *symbol;
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
                                // formatter = obj.fmt.clone();
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!(
                                        "Line {}: {} Can't be used with extended format",
                                        index + 1,
                                        &opcode
                                    ),
                                ));
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
            let total = end - start;
            // Pass1
            // =========
            // for line in line_struct.iter() {
            //     println!(
            //         "{:06}\t{:8}\t{:8}\t{:8}",
            //         line.get_memory(),
            //         line.get_symbol().unwrap_or(&"".to_string()),
            //         line.get_op(),
            //         line.get_operand1().unwrap_or(&"".to_string())
            //     )
            // }
            // println!("\nProgram length = {:06X}\n", total);
            // for (k, v) in symbol_table.iter() {
            //     println!("{:10}: {:06X}", k, v);
            // }
            // println!("{:#?}", line_struct);
            // =========
            pass2(&line_struct, &symbol_table, total, base_register)?;
        }
        None => println!("Usage: {} <filename.asm>", program),
    }

    Ok(())
}

#[allow(unused_assignments)]
fn pass2(
    line_struct: &[Line],
    symbol_table: &HashMap<String, u32>,
    total_len: u32,
    base_register: u32,
) -> io::Result<()> {
    let mut file_writer = FileWriter::new("output.txt")?;
    let mut end_address = 0x0;
    if let Some(start_line) = line_struct.iter().find(|l| l.get_op() == "START") {
        let empty = "".to_string();
        let program_name = start_line.get_symbol().unwrap_or(&empty);
        let start_address = start_line.get_memory();
        end_address = start_address;
        let header_record = format!(
            "H{:<6}{:06X}{:06X}\n",
            program_name, start_address, total_len
        );
        file_writer.write(&header_record)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No START directive found",
        ));
    }
    let mut text_record = String::new();
    let mut text_record_length = 0;
    let mut start_address = 0;
    for line in line_struct.iter() {
        if let FormatDirective::Format(Format::FMT0) = line.get_fmt() {
            continue;
        }
        let mut object_code = String::new();
        let ni = match line.get_address_mode() {
            AddrMode::Simple => 0x3,
            AddrMode::Immediate => 0x1,
            AddrMode::Indirect => 0x2,
            AddrMode::Index => 0x3,
        };
        let x_bit = if matches!(line.get_address_mode(), AddrMode::Index) {
            0x8
        } else {
            0x0
        };
        let extended = matches!(line.get_fmt(), FormatDirective::Format(Format::FMT4));
        let pc = line.get_memory() + if extended { 4 } else { 3 };
        let mut displacement = 0;
        let mut xbpe = if extended { 0x1 } else { 0x2 };
        if let Some(symbol) = line.get_operand1() {
            if let Some(&address) = symbol_table.get(symbol) {
                let pc_relative = address.wrapping_sub(pc) & 0xFFF;
                let base_relative_disp = address.wrapping_sub(base_register) & 0xFFF;
                if pc_relative > 2048 && base_relative_disp <= 2048 {
                    displacement = base_relative_disp as i32;
                    xbpe |= 0x4;
                } else {
                    displacement = pc_relative as i32;
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Undefined symbol {}", symbol),
                ));
            }
        }
        xbpe |= x_bit;
        let opcode = line.get_code() as u32;
        object_code = format!("{:02X}{:01X}{:03X}", opcode, ni << 4 | xbpe, displacement);

        if text_record_length + object_code.len() / 2 > 30 {
            file_writer.write(&format!(
                "T{:06X}{:02X}{}\n",
                start_address, text_record_length, text_record
            ))?;
            text_record.clear();
            text_record_length = 0;
            start_address = line.get_memory();
        }

        if text_record.is_empty() {
            start_address = line.get_memory();
        }

        text_record_length += object_code.len() / 2;
        text_record.push_str(&object_code);
    }

    if !text_record.is_empty() {
        file_writer.write(&format!(
            "T{:06X}{:02X}{}\n",
            start_address, text_record_length, text_record
        ))?;
    }
    file_writer.write(&format!("E{:06X}\n", end_address))?;
    Ok(())
}
