use crate::code_writer::CodeWriter;
use crate::parser::Parser;
use std::fs::{read_to_string, File};

pub struct VmFile {
    pub file: File,
    name: String,
}

impl VmFile {
    pub fn new(filename: &str) -> Result<VmFile, std::io::Error> {
        let file = File::create(format!("{}.asm", filename))?;
        Ok(VmFile {
            file,
            name: filename.to_string(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn parse_filename(configs: &[String]) -> Result<&String, &'static str> {
    if configs.len() == 0 {
        return Err("missing filename argument");
    }
    Ok(&configs[1])
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|line| line.trim().to_string())
        .collect()
}

pub fn compile_vm_code(mut parser: Parser, mut code_writer: CodeWriter, test: bool) {
    // initialize the memory base address if we are testing/debugging
    if test {
        code_writer.init_stack();
    }
    dbg!(test);
    while parser.hasMoreLines() {
        parser.advance();

        dbg!(&parser.current_instruction());
        dbg!(&parser.commandType());
        // match on command type
        if let Ok(cmd) = parser.commandType() {
            match cmd {
                "C_PUSH" => {
                    dbg!(cmd);
                    code_writer
                        .write_push_pop(
                            cmd,
                            &parser.arg1().expect("error"),
                            &parser.arg2().expect("error").parse().unwrap(),
                        )
                        .expect("error");
                }
                "C_POP" => {
                    dbg!(cmd);
                    code_writer
                        .write_push_pop(
                            cmd,
                            &parser.arg1().expect("error"),
                            &parser.arg2().expect("error").parse().unwrap(),
                        )
                        .expect("error");
                }
                "C_ARITHMETIC" => {
                    dbg!(cmd);
                    code_writer
                        .write_arithmetic(&parser.arg1().expect("error"))
                        .expect("error");
                }
                "C_LABEL" => {
                    dbg!(cmd);
                    code_writer
                        .write_label(&parser.arg1().expect("error"))
                        .expect("error");
                }
                "C_IFGOTO" => {
                    dbg!(cmd);
                    code_writer
                        .write_ifgoto(&parser.arg1().expect("error"))
                        .expect("error");
                }
                "C_GOTO" => {
                    dbg!(cmd);
                    code_writer
                        .write_goto(&parser.arg1().expect("error"))
                        .expect("error")
                }
                _ => {}
            }
        } else {
            panic!(
                "{}: {}",
                "command not implemented",
                parser.current_instruction()
            );
        }
    }
}
