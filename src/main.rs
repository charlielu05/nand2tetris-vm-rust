use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs::{read_to_string, File};
use std::io::{self, ErrorKind, Write};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_filename(configs: &[String]) -> Result<&String, &'static str> {
    if configs.len() == 0 {
        return Err("missing filename argument");
    }
    Ok(&configs[1])
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|line| line.trim().to_string())
        .collect()
}

struct Parser {
    contents: Vec<String>,
    currentLine: usize,
    currentInstruction: String,
}

impl Parser {
    fn hasMoreLines(&self) -> bool {
        if self.currentLine < self.contents.len() - 1 {
            return true;
        } else {
            return false;
        }
    }

    fn advance(&mut self) {
        if self.hasMoreLines() {
            self.currentLine += 1;
            if self.contents[self.currentLine].starts_with("//")
                | self.contents[self.currentLine].is_empty()
            {
                self.advance();
            } else {
                self.currentInstruction = self.contents[self.currentLine].to_string();
            }
        }
    }

    fn commandType(&self) -> Result<&str, &str> {
        match self.contents[self.currentLine].as_str() {
            line if line.starts_with("push") => Ok("C_PUSH"),
            line if line.starts_with("pop") => Ok("C_POP"),
            line if ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"].contains(&line) => {
                Ok("C_ARITHMETIC")
            }
            _ => Err("could not match command"),
        }
    }

    fn arg1(&self) -> Option<String> {
        match self.commandType() {
            Ok(cmd) => match cmd {
                "C_PUSH" | "C_POP" => {
                    let parts: Vec<&str> = self.currentInstruction.split_whitespace().collect();
                    Some(parts[1].to_string())
                }
                "C_ARITHMETIC" => Some(self.currentInstruction.to_string()),
                _ => None,
            },
            Err(_) => None,
        }
    }

    fn arg2(&self) -> Option<String> {
        match self.commandType() {
            Ok(cmd) => match cmd {
                "C_PUSH" | "C_POP" | "C_FUNCTION" | "C_CALL" => {
                    let parts: Vec<&str> = self.currentInstruction.split_whitespace().collect();
                    Some(parts[2].parse().unwrap())
                }
                _ => None,
            },
            Err(_) => None,
        }
    }
}

struct CodeWriter<'a> {
    output_file: File,
    mem_offset_map: &'a HashMap<MemoryLocation, i16>,
}

impl<'a> CodeWriter<'a> {
    fn write_lines(&mut self, lines: Vec<&str>) -> std::io::Result<()> {
        for line in lines {
            writeln!(self.output_file, "{}", line)?;
        }
        Ok(())
    }

    fn init_stack(&mut self) {
        writeln!(
            self.output_file,
            "@{}",
            self.mem_offset_map
                .get(&MemoryLocation::Stack)
                .expect("wrong key")
        )
        .unwrap();
        writeln!(self.output_file, "D=A").unwrap();
        writeln!(self.output_file, "@SP").unwrap();
        writeln!(self.output_file, "M=D").unwrap();
    }

    fn write_push_pop(&mut self, command: &str, segment: &str, index: &i16) -> Result<(), &str> {
        // segment is a memory location, segment + index = actual memory location
        let offset = match segment {
            "constant" => 0,
            "argument" => 1,
            "local" => 2,
            "static" => 3,
            "this" => 4,
            "pointer" => 256,
            "index" => 6,
            _ => -1,
        };

        match command {
            "C_PUSH" => match segment {
                "constant" => {
                    let address = index;
                    self.write_lines(vec![
                        &format!("@{}", address),
                        "D=A",
                        "@SP",
                        "A=M",
                        "M=D",
                        "@SP",
                        "M=M+1",
                    ])
                    .expect("error");
                    Ok(())
                }
                _ => Err("not implemented"),
            },
            // "C_POP" => {}
            _ => Ok(()),
        }
    }

    fn write_arithmetic(&mut self, command: &str) -> Result<(), ErrorKind> {
        match command {
            "add" => {
                self.write_lines(vec![
                    "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M-1", "D=D+M", "M=D",
                ])
                .expect("error");
                Ok(())
            }
            "sub" => {
                self.write_lines(vec![
                    "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M-1", "D=D-M", "M=D",
                ])
                .expect("error");
                Ok(())
            }
            "neg" => {
                self.write_lines(vec![
                    "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M", "D=-D", "M=D",
                ])
                .expect("error");
                Ok(())
            }
            "eq" => Ok(()),
            "gt" => Ok(()),
            "lt" => Ok(()),
            "and" => {
                self.write_lines(vec![
                    "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M-1", "D=D&M", "M=D",
                ])
                .expect("error");
                Ok(())
            }
            "or" => {
                self.write_lines(vec![
                    "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M-1", "D=D|M", "M=D",
                ])
                .expect("error");
                Ok(())
            }
            "not" => {
                self.write_lines(vec![
                    "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M", "D=!D", "M=D",
                ])
                .expect("error");
                Ok(())
            }
            _ => Err(ErrorKind::InvalidInput),
        }
    }
    fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.output_file.sync_all().map_err(|e| e.into())
    }
}
#[derive(Hash, Eq, PartialEq, Debug)]
enum MemoryLocation {
    Constant,
    Argument,
    Local,
    Static,
    This,
    Pointer,
    Index,
    Stack,
}

fn main() {
    let mem_offset_map: HashMap<MemoryLocation, i16> = HashMap::from([
        (MemoryLocation::Constant, 0),
        (MemoryLocation::Argument, 1),
        (MemoryLocation::Local, 2),
        (MemoryLocation::Static, 3),
        (MemoryLocation::This, 4),
        (MemoryLocation::Pointer, 5),
        (MemoryLocation::Index, 6),
        (MemoryLocation::Stack, 256),
    ]);

    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let filepath = parse_filename(&args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    let filename = Path::new(filepath).file_stem().unwrap().to_str().unwrap();
    println!("Creating Virtual Machine bytecode file: {:?}", filename);

    let lines = read_lines(filepath);

    //
    let mut parser = Parser {
        contents: lines,
        currentLine: 0,
        currentInstruction: "".to_string(),
    };

    // write to file
    let f = File::create(format!("{}.asm", filename)).expect("failed...");

    let mut code_writer = CodeWriter {
        output_file: f,
        mem_offset_map: &mem_offset_map,
    };

    code_writer.init_stack();

    while parser.hasMoreLines() {
        parser.advance();

        dbg!(&parser.currentLine);
        // match on command type
        if let Ok(cmd) = parser.commandType() {
            dbg!(&parser.arg1());
            match cmd {
                "C_PUSH" => {
                    code_writer
                        .write_push_pop(
                            "C_PUSH",
                            &parser.arg1().expect("error"),
                            &parser.arg2().expect("error").parse().unwrap(),
                        )
                        .expect("error");
                }
                "C_POP" => {}
                "C_ARITHMETIC" => code_writer
                    .write_arithmetic(&parser.arg1().expect("error"))
                    .expect("error"),
                _ => {}
            }
        }
    }
}
