use hack_vm::compiler::{compile_vm_code, parse_filename, read_lines};
use hack_vm::compiler::{CodeWriter, Parser, VmFile};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args.len() == 2 {
        panic!("two arguments required, filepath string and test boolean")
    }

    let filepath = parse_filename(&args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    let filename = Path::new(filepath).file_stem().unwrap().to_str().unwrap();
    println!("Creating Virtual Machine bytecode file: {:?}", filename);

    let is_test = matches!(args[2].as_str(), "test");

    // read the code file
    let lines = read_lines(filepath);

    // initialise the file object
    let file = VmFile::new(filename).unwrap();

    let parser = Parser::new(lines);

    let code_writer = CodeWriter::new(file, is_test);

    compile_vm_code(parser, code_writer, is_test)
}
