use serde::de::value::Error;

use crate::compiler::VmFile;
use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::io::Write;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum MemoryLocation {
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

pub struct CodeWriter {
    pub output_file: VmFile,
    filename: Option<String>,
    label_number: i16,
    mem_offset_map: Option<HashMap<MemoryLocation, i16>>,
    pub state: i16,
}

impl CodeWriter {
    pub fn new(file: VmFile, is_test: bool) -> Self {
        if is_test {
            let mem_offset_map: Option<HashMap<MemoryLocation, i16>> = Some(HashMap::from([
                (MemoryLocation::Constant, 0),
                (MemoryLocation::Argument, 756),
                (MemoryLocation::Local, 456),
                (MemoryLocation::Static, 3),
                (MemoryLocation::This, 1056),
                (MemoryLocation::That, 1356),
                (MemoryLocation::Pointer, 5),
                (MemoryLocation::Index, 6),
                (MemoryLocation::Stack, 256),
            ]));
            return CodeWriter {
                output_file: file,
                label_number: 0,
                filename: None,
                mem_offset_map,
                state: 0,
            };
        } else {
            let mut code_writer = CodeWriter {
                output_file: file,
                label_number: 0,
                filename: None,
                mem_offset_map: None,
                state: 0,
            };
            code_writer.write_bootstrap().unwrap();

            return code_writer;
        };
    }

    pub fn set_file_name(&mut self, filename: &str) {
        self.filename = Some(filename.to_string())
    }

    fn write_lines(&mut self, lines: Vec<&str>) -> std::io::Result<()> {
        for line in lines {
            writeln!(self.output_file.file, "{}", line)?;
        }
        Ok(())
    }

    fn write_bootstrap(&mut self) -> std::io::Result<()> {
        // set stack pointer value to 256
        self.write_lines(vec!["@256", "D=A", "@0", "M=D"])?;

        // call Sys.init function
        self.write_call(&String::from("Sys.init"), &String::from("0"))?;
        Ok(())
    }

    fn write_address(&mut self, segment: &str) {
        let mem_location = match segment {
            "SP" => Ok(self
                .mem_offset_map
                .as_ref()
                .unwrap()
                .get(&MemoryLocation::Stack)
                .expect("wrong key")),
            "LCL" => Ok(self
                .mem_offset_map
                .as_ref()
                .unwrap()
                .get(&MemoryLocation::Local)
                .expect("wrong key")),
            "ARG" => Ok(self
                .mem_offset_map
                .as_ref()
                .unwrap()
                .get(&MemoryLocation::Argument)
                .expect("wrong key")),
            "THIS" => Ok(self
                .mem_offset_map
                .as_ref()
                .unwrap()
                .get(&MemoryLocation::This)
                .expect("wrong key")),
            "THAT" => Ok(self
                .mem_offset_map
                .as_ref()
                .unwrap()
                .get(&MemoryLocation::That)
                .expect("wrong key")),
            _ => Err(()),
        };

        writeln!(self.output_file.file, "//setting up {} address", segment).unwrap();
        writeln!(self.output_file.file, "@{}", mem_location.unwrap()).unwrap();
        writeln!(self.output_file.file, "D=A").unwrap();
        writeln!(self.output_file.file, "@{}", segment).unwrap();
        writeln!(self.output_file.file, "M=D").unwrap();
    }

    pub fn init_stack(&mut self) {
        let fixed_variables = vec!["SP", "LCL", "ARG", "THIS", "THAT"];
        for var in fixed_variables {
            self.write_address(var);
        }
    }

    pub fn write_push_pop(
        &mut self,
        command: &str,
        segment: &str,
        index: &i16,
    ) -> Result<(), &str> {
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
                        &format!("@{}.{}", &self.filename.clone().unwrap(), index),
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
                        &format!("@{}.{}", &self.filename.clone().unwrap(), index),
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

    pub fn write_arithmetic(&mut self, command: &str) -> Result<(), ErrorKind> {
        match command {
            "add" => {
                self.write_lines(vec![
                    "//add", "@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "A=M-1", "D=D+M", "M=D",
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

    pub fn write_label(&mut self, label: &String) -> Result<(), std::io::Error> {
        self.write_lines(vec!["//label", &format!("({})", label)])
    }

    pub fn write_ifgoto(&mut self, label: &String) -> Result<(), std::io::Error> {
        self.write_lines(vec![
            "//if-goto",
            "@SP",
            "AM=M-1",
            "D=M",
            &format!("@{}", label),
            "D;JNE",
        ])
    }

    pub fn write_goto(&mut self, label: &String) -> Result<(), std::io::Error> {
        self.write_lines(vec!["//goto", &format!("@{}", label), "0; JMP"])
    }

    pub fn write_function(
        &mut self,
        function_name: &String,
        nvars: &String,
    ) -> Result<(), std::io::Error> {
        self.write_lines(vec!["//function"])?;
        self.write_label(function_name).unwrap();
        let mut i = 0;
        let j: i16 = nvars.parse().expect("error parsing");
        while i < j {
            self.write_lines(vec!["//nvars"])?;
            // push 0 for local variables
            self.write_push_pop("C_PUSH", "constant", &0).unwrap();
            i += 1;
        }
        Ok(())
    }

    fn finish_push(&mut self) -> Result<(), std::io::Error> {
        // finishes push to stack
        self.write_lines(vec!["@SP", "A=M", "M=D", "@SP", "M=M+1"])
    }

    pub fn write_call(
        &mut self,
        function_name: &String,
        nargs: &String,
    ) -> Result<(), std::io::Error> {
        let return_address = format!("{}$ret.{}", function_name, &self.label_number);

        // push returnAddr, this should be functionName$ret.i
        self.write_lines(vec![
            "//push returnAddr",
            &format!("@{}", &return_address),
            "D=A",
        ])?;
        self.finish_push().expect("error finish push");

        // push LCL
        self.write_lines(vec!["//push lcl", "@LCL", "D=M"])?;
        self.finish_push().expect("error finish push");

        // push ARG
        self.write_lines(vec!["//push arg", "@ARG", "D=M"])?;
        self.finish_push().expect("error finish push");

        // push THIS
        self.write_lines(vec!["//push this", "@THIS", "D=M"])?;
        self.finish_push().expect("error finish push");

        // push THAT
        self.write_lines(vec!["//push that", "@THAT", "D=M"])?;
        self.finish_push().expect("error finish push");

        // ARG = SP - 5 - nArgs
        self.write_lines(vec![
            "//arg=sp-5-nargs",
            "@SP",
            "D=M",
            "@5",
            "D=D-A",
            &format!("@{}", nargs),
            "D=D-A",
            "@ARG",
            "M=D",
        ])?;

        // LCL = SP
        self.write_lines(vec!["//lcl=sp", "@SP", "D=M", "@LCL", "M=D"])?;

        // goto f
        self.write_goto(function_name).expect("error");

        // (returnAddress)
        self.write_label(&return_address).expect("error");

        // increment label number
        self.label_number += 1;

        Ok(())
    }

    pub fn write_return(&mut self) -> Result<(), std::io::Error> {
        // frame = LCL
        // save LCL address to SP address
        self.write_lines(vec!["//frame=LCL", "@LCL", "D=M", "@SP", "A=M", "M=D"])?;
        // pop SP value(LCL address) to R13
        self.write_lines(vec!["@13", "M=D", "@SP", "M=M-1"])?;

        // retAddr=*(frame-5)
        // set D=5
        self.write_lines(vec!["//retAddr=*(frame-5", "@5", "D=A"])?;
        // D = frame - 5
        self.write_lines(vec!["//frame-5", "@R13", "D=M-D", "A=D", "D=M"])?;
        // write D(retAddr) to R14
        self.write_lines(vec!["@14", "M=D"])?;

        // pop SP value to ARG
        self.write_lines(vec![
            "//arg=pop()",
            "@SP",
            "A=M",
            "D=M",
            "@ARG",
            "A=M",
            "M=D",
        ])?;

        // SP = ARG+1
        self.write_lines(vec!["//SP=ARG+1", "@ARG", "D=M", "@SP", "M=D+1"])?;

        // THAT = *(frame-1)
        self.write_lines(vec![
            "//THAT=*(frame-1)",
            "@R13",
            "A=M-1",
            "D=M",
            "@THAT",
            "M=D",
        ])?;

        // THIS = *(frame-2)
        self.write_lines(vec![
            "//THIS=*(frame-2)",
            "@2",
            "D=A",
            "@R13",
            "A=M-D",
            "D=M",
            "@THIS",
            "M=D",
        ])?;

        // ARG = *(frame-3)
        self.write_lines(vec![
            "//ARG=*(frame-3)",
            "@3",
            "D=A",
            "@R13",
            "A=M-D",
            "D=M",
            "@ARG",
            "M=D",
        ])?;

        // LCL = *(frame-4)
        self.write_lines(vec![
            "//LCL=*(frame-4)",
            "@4",
            "D=A",
            "@R13",
            "A=M-D",
            "D=M",
            "@LCL",
            "M=D",
        ])?;

        // goto retrAddr
        self.write_lines(vec!["//goto", "@R14", "A=M", "0; JMP"])?;
        Ok(())
    }

    fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.output_file.file.sync_all().map_err(|e| e.into())
    }
}
