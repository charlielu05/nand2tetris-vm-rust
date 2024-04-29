use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{ErrorKind, Write};
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

struct VmFile {
    file: File,
    name: String,
}

impl VmFile {
    fn new(filename: &str) -> Result<VmFile, std::io::Error> {
        let file = File::create(format!("{}.asm", filename))?;
        Ok(VmFile {
            file,
            name: filename.to_string(),
        })
    }

    fn name(&self) -> &str {
        &self.name
    }
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
    output_file: VmFile,
    mem_offset_map: &'a HashMap<MemoryLocation, i16>,
    state: i16,
}

impl<'a> CodeWriter<'a> {
    fn write_lines(&mut self, lines: Vec<&str>) -> std::io::Result<()> {
        for line in lines {
            writeln!(self.output_file.file, "{}", line)?;
        }
        Ok(())
    }

    fn write_address(&mut self, segment: &str) {
        writeln!(self.output_file.file, "//setting up {} address", segment).unwrap();

        let mem_location = match segment {
            "SP" => Ok(self
                .mem_offset_map
                .get(&MemoryLocation::Stack)
                .expect("wrong key")),
            "LCL" => Ok(self
                .mem_offset_map
                .get(&MemoryLocation::Local)
                .expect("wrong key")),
            "ARG" => Ok(self
                .mem_offset_map
                .get(&MemoryLocation::Argument)
                .expect("wrong key")),
            "THIS" => Ok(self
                .mem_offset_map
                .get(&MemoryLocation::This)
                .expect("wrong key")),
            "THAT" => Ok(self
                .mem_offset_map
                .get(&MemoryLocation::That)
                .expect("wrong key")),
            _ => Err(()),
        };

        writeln!(self.output_file.file, "@{}", mem_location.unwrap()).unwrap();
        writeln!(self.output_file.file, "D=A").unwrap();
        writeln!(self.output_file.file, "@{}", segment).unwrap();
        writeln!(self.output_file.file, "M=D").unwrap();
    }

    fn init_stack(&mut self) {
        self.write_address("SP");
        self.write_address("LCL");
        self.write_address("ARG");
        self.write_address("THIS");
        self.write_address("THAT")
    }

    fn write_push_pop(&mut self, command: &str, segment: &str, index: &i16) -> Result<(), &str> {
        // segment is a memory location, segment + index = actual memory location
        // stack memory is from 256 - 2047
        // stack memory is shared so we need to allocate sufficient space for each offset
        match command {
            "C_PUSH" => match segment {
                "constant" => {
                    self.write_lines(vec![
                        "// push constant",
                        &format!("@{}", index),
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
                "argument" => {
                    self.write_lines(vec![
                        "//push argument",
                        &format!("@{}", index),
                        "D=A",
                        "@ARG",
                        "A=M+D",
                        "D=M",
                        "@SP",
                        "A=M",
                        "M=D",
                        // increment SP
                        "@SP",
                        "M=M+1",
                    ])
                    .expect("error");
                    Ok(())
                }
                "local" => {
                    self.write_lines(vec![
                        "//push local",
                        &format!("@{}", index),
                        "D=A",
                        "@LCL",
                        "A=M+D",
                        "D=M",
                        "@SP",
                        "A=M",
                        "M=D",
                        // increment SP
                        "@SP",
                        "M=M+1",
                    ])
                    .expect("error");
                    Ok(())
                }
                "static" => {
                    self.write_lines(vec![
                        "//push static",
                        &format!("@{}.{}", &self.output_file.name(), index),
                        "D=M",
                        "@SP",
                        "A=M",
                        "M=D",
                        // increment SP
                        "@SP",
                        "M=M+1",
                    ])
                    .expect("error");
                    Ok(())
                }
                "this" => {
                    self.write_lines(vec![
                        "//push this",
                        &format!("@{}", index),
                        "D=A",
                        "@THIS",
                        "A=M+D",
                        "D=M",
                        "@SP",
                        "A=M",
                        "M=D",
                        // increment SP
                        "@SP",
                        "M=M+1",
                    ])
                    .expect("error");
                    Ok(())
                }
                "that" => {
                    self.write_lines(vec![
                        "//push that",
                        &format!("@{}", index),
                        "D=A",
                        "@THAT",
                        "A=M+D",
                        "D=M",
                        "@SP",
                        "A=M",
                        "M=D",
                        // increment SP
                        "@SP",
                        "M=M+1",
                    ])
                    .expect("error");
                    Ok(())
                }
                "temp" => {
                    self.write_lines(vec![
                        "//push temp",
                        &format!("@{}", index),
                        "D=A",
                        // TEMP starts at RAM[5]
                        "@5",
                        "A=A+D",
                        "D=M",
                        "@SP",
                        "A=M",
                        "M=D",
                        // increment SP
                        "@SP",
                        "M=M+1",
                    ])
                    .expect("error");
                    Ok(())
                }
                "pointer" => {
                    // pointer 0 is THIS, pointer 1 is THAT
                    let address = match index {
                        0 => Ok("THIS"),
                        1 => Ok("THAT"),
                        _ => Err("invalid"),
                    };

                    self.write_lines(vec![
                        "//push pointer",
                        &format!("@{}", address?),
                        "D=M",
                        "@SP",
                        "A=M",
                        "M=D",
                        // increment SP
                        "@SP",
                        "M=M+1",
                    ])
                    .expect("error");
                    Ok(())
                }
                _ => Err("not implemented"),
            },
            "C_POP" => match segment {
                "argument" => {
                    self.write_lines(vec![
                        "// pop argument",
                        "// decrement stack pointer",
                        "@SP",
                        "M=M-1",
                        &format!("@{}", index),
                        "D=A",
                        "@ARG",
                        "// arg address + index",
                        "D=M+D",
                        "// save to temp register",
                        "@R13",
                        "M=D",
                        "// get value of stack pointer",
                        "@SP",
                        "A=M",
                        "D=M",
                        "// save to arg address stored in temp register",
                        "@R13",
                        "A=M",
                        "M=D",
                    ])
                    .expect("error");
                    Ok(())
                }
                "local" => {
                    self.write_lines(vec![
                        "// pop local",
                        "// decrement stack pointer",
                        "@SP",
                        "M=M-1",
                        &format!("@{}", index),
                        "D=A",
                        "@LCL",
                        "// LCL address + index",
                        "D=M+D",
                        "// save to temp register",
                        "@R13",
                        "M=D",
                        "// get value of stack pointer",
                        "@SP",
                        "A=M",
                        "D=M",
                        "// save to LCL address stored in temp register",
                        "@R13",
                        "A=M",
                        "M=D",
                    ])
                    .expect("error");
                    Ok(())
                }
                "this" => {
                    self.write_lines(vec![
                        "// pop this",
                        "// decrement stack pointer",
                        "@SP",
                        "M=M-1",
                        &format!("@{}", index),
                        "D=A",
                        "@THIS",
                        "// THIS address + index",
                        "D=M+D",
                        "// save to temp register",
                        "@R13",
                        "M=D",
                        "// get value of stack pointer",
                        "@SP",
                        "A=M",
                        "D=M",
                        "// save to THIS address stored in temp register",
                        "@R13",
                        "A=M",
                        "M=D",
                    ])
                    .expect("error");
                    Ok(())
                }
                "that" => {
                    self.write_lines(vec![
                        "// pop that",
                        "// decrement stack pointer",
                        "@SP",
                        "M=M-1",
                        &format!("@{}", index),
                        "D=A",
                        "@THAT",
                        "// THAT address + index",
                        "D=M+D",
                        "// save to temp register",
                        "@R13",
                        "M=D",
                        "// get value of stack pointer",
                        "@SP",
                        "A=M",
                        "D=M",
                        "// save to THAT address stored in temp register",
                        "@R13",
                        "A=M",
                        "M=D",
                    ])
                    .expect("error");
                    Ok(())
                }
                "static" => {
                    self.write_lines(vec![
                        "//pop static",
                        "// decrement stack pointer",
                        "@SP",
                        "M=M-1",
                        "// get value of stack pointer",
                        "@SP",
                        "A=M",
                        "D=M",
                        &format!("@{}.{}", &self.output_file.name(), index),
                        "M=D",
                    ])
                    .expect("error");
                    Ok(())
                }
                "temp" => {
                    self.write_lines(vec![
                        "//pop temp",
                        "// decrement stack pointer",
                        "@SP",
                        "M=M-1",
                        &format!("@{}", index),
                        "D=A",
                        // TEMP starts at RAM[5]
                        "@5",
                        "// temp address + index",
                        "D=A+D",
                        "// save to temp register",
                        "@R13",
                        "M=D",
                        "// get value of stack pointer",
                        "@SP",
                        "A=M",
                        "D=M",
                        "// save to arg address stored in temp register",
                        "@R13",
                        "A=M",
                        "M=D",
                    ])
                    .expect("error");
                    Ok(())
                }
                "pointer" => {
                    // pointer 0 is THIS, pointer 1 is THAT
                    let address = match index {
                        0 => Ok("THIS"),
                        1 => Ok("THAT"),
                        _ => Err("invalid"),
                    };

                    self.write_lines(vec![
                        "//pop pointer",
                        "// decrement stack pointer",
                        "@SP",
                        "M=M-1",
                        "// get value of stack pointer",
                        "@SP",
                        "A=M",
                        "D=M",
                        &format!("@{}", address?),
                        "M=D",
                    ])
                    .expect("error");
                    Ok(())
                }
                _ => Err("not implemented"),
            },
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
                    "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M-1", "D=M-D", "M=D",
                ])
                .expect("error");
                Ok(())
            }
            "neg" => {
                self.write_lines(vec![
                    "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M", "D=-D", "M=D",
                    // SP + 1
                    "@SP", "M=M+1",
                ])
                .expect("error");
                Ok(())
            }
            "eq" => {
                self.write_lines(vec![
                    "@SP",
                    "M=M-1",
                    "@SP",
                    "A=M",
                    "D=M",
                    "@SP",
                    "A=M-1",
                    "D=M-D",
                    &format!("@TRUE_{}", &self.state),
                    "D;JEQ",
                    // false
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "M=0",
                    &format!("@CONTINUE_{}", &self.state),
                    "0;JMP",
                    &format!("(TRUE_{})", &self.state),
                    // true
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "M=-1",
                    &format!("(CONTINUE_{})", &self.state),
                    // SP + 1
                    "@SP",
                    "M=M+1",
                ])
                .expect("error");
                // increment the state counter to keep the labels unique
                self.state += 1;
                Ok(())
            }
            "gt" => {
                self.write_lines(vec![
                    "@SP",
                    "M=M-1",
                    "@SP",
                    "A=M",
                    "D=M",
                    "@SP",
                    "A=M-1",
                    "D=M-D",
                    &format!("@TRUE_{}", &self.state),
                    "D;JGT",
                    // false
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "M=0",
                    &format!("@CONTINUE_{}", &self.state),
                    "0;JMP",
                    &format!("(TRUE_{})", &self.state),
                    // true
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "M=-1",
                    &format!("(CONTINUE_{})", &self.state),
                    // SP + 1
                    "@SP",
                    "M=M+1",
                ])
                .expect("error");
                // increment the state counter to keep the labels unique
                self.state += 1;
                Ok(())
            }
            "lt" => {
                self.write_lines(vec![
                    "@SP",
                    "M=M-1",
                    "@SP",
                    "A=M",
                    "D=M",
                    "@SP",
                    "A=M-1",
                    "D=D-M",
                    &format!("@TRUE_{}", &self.state),
                    "D;JGT",
                    // false
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "M=0",
                    &format!("@CONTINUE_{}", &self.state),
                    "0;JMP",
                    &format!("(TRUE_{})", &self.state),
                    // true
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "M=-1",
                    &format!("(CONTINUE_{})", &self.state),
                    // SP + 1
                    "@SP",
                    "M=M+1",
                ])
                .expect("error");
                // increment the state counter to keep the labels unique
                self.state += 1;
                Ok(())
            }
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
                self.write_lines(vec!["@SP", "A=M-1", "M=!M"])
                    .expect("error");
                Ok(())
            }
            _ => Err(ErrorKind::InvalidInput),
        }
    }
    fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.output_file.file.sync_all().map_err(|e| e.into())
    }
}
#[derive(Hash, Eq, PartialEq, Debug)]
enum MemoryLocation {
    Constant,
    Argument,
    Local,
    Static,
    This,
    That,
    Pointer,
    Index,
    Stack,
}

fn compile_vm_code(mut parser: Parser, mut code_writer: CodeWriter, test: bool) {
    // initialize the memory base address if we are testing/debugging
    if test {
        code_writer.init_stack();
    }

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
                            cmd,
                            &parser.arg1().expect("error"),
                            &parser.arg2().expect("error").parse().unwrap(),
                        )
                        .expect("error");
                }
                "C_POP" => {
                    code_writer
                        .write_push_pop(
                            cmd,
                            &parser.arg1().expect("error"),
                            &parser.arg2().expect("error").parse().unwrap(),
                        )
                        .expect("error");
                }
                "C_ARITHMETIC" => code_writer
                    .write_arithmetic(&parser.arg1().expect("error"))
                    .expect("error"),
                _ => {}
            }
        }
    }
}
fn main() {
    let mem_offset_map: HashMap<MemoryLocation, i16> = HashMap::from([
        (MemoryLocation::Constant, 0),
        (MemoryLocation::Argument, 756),
        (MemoryLocation::Local, 456),
        (MemoryLocation::Static, 3),
        (MemoryLocation::This, 1056),
        (MemoryLocation::That, 1356),
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

    let parser = Parser {
        contents: lines,
        currentLine: 0,
        currentInstruction: "".to_string(),
    };

    // write to file
    let file = VmFile::new(filename).unwrap();

    let code_writer = CodeWriter {
        output_file: file,
        mem_offset_map: &mem_offset_map,
        state: 0,
    };

    let test = {
        if args[2] == "true".to_string() {
            true
        } else {
            false
        }
    };

    compile_vm_code(parser, code_writer, test)
}
