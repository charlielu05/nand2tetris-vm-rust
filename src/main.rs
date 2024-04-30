use std::collections::HashMap;
use std::env;
use std::path::Path;
use hack_vm::{VmFile, MemoryLocation, Parser, CodeWriter};
use hack_vm::{parse_filename, read_lines, compile_vm_code};

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

    let is_test = matches!(args[2].as_str(), "test");

    // read the code file 
    let lines = read_lines(filepath);

    // initialise the file object
    let file = VmFile::new(filename).unwrap();

    let parser = Parser::new(lines, 0);

    let code_writer = CodeWriter::new(file, is_test);

    compile_vm_code(parser, code_writer, is_test)
}
