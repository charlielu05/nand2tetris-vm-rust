use hack_vm::code_writer::CodeWriter;
use hack_vm::compiler::{compile_vm_code, parse_filename, read_lines, VmFile};
use hack_vm::parser::Parser;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args.len() == 2 {
        panic!("two arguments required, filepath string and test boolean(true/false)")
    }

    let filepath = parse_filename(&args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    // match whether filepath is a single file or a folder
    let is_dir = std::path::PathBuf::from(filepath).is_dir();
    let is_file = std::path::PathBuf::from(filepath).is_file();

    dbg!(format!("is_dir: {}", is_dir));
    dbg!(format!("is_file: {}", is_file));

    let file_parent = Path::new(filepath).parent().unwrap().to_str().unwrap();
    dbg!(file_parent);
    let filename = Path::new(filepath).file_stem().unwrap().to_str().unwrap();
    dbg!(filename);

    if is_file {
        let file = VmFile::new(format!("{}/{}", file_parent, filename).as_str()).unwrap();
        dbg!(format!("{}/{}", file_parent, filename));
        println!(
            "Creating Virtual Machine bytecode file: {:?}",
            format!("{}/{}", file_parent, filename)
        );

        let is_test = matches!(args[2].as_str(), "true");

        // read the code file
        let lines = read_lines(filepath);

        // // initialise the file object
        // let file = VmFile::new(format!("{}/{}", file_parent, filename).as_str()).unwrap();

        let parser = Parser::new(lines);

        let code_writer = CodeWriter::new(file, is_test);
        _ = compile_vm_code(parser, code_writer, &is_test)
    } else if is_dir {
        let file =
            VmFile::new(format!("{}/{}/{}", file_parent, filename, filename).as_str()).unwrap();
        let is_test = matches!(args[2].as_str(), "true");
        dbg!(format!("is_dir: {}", is_dir));

        // filter for all the .vm files in the directory
        let vm = &OsStr::new("vm");

        // want all files with a .vm extension
        let entries = Vec::from_iter(
            fs::read_dir(format!("{}/{}", file_parent, filename))
                .unwrap()
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension() == Some(vm)),
        );

        dbg!(&entries);

        let mut code_writer = CodeWriter::new(file, is_test);
        // create new parser for each vm file
        // pass the parser sequentially to the code_writer, also invoking setFilename on the code_writer
        for vm_file in entries {
            let filename = vm_file.as_path().file_stem();
            let lines = read_lines(vm_file.as_os_str().to_str().unwrap());
            let parser = Parser::new(lines);
            code_writer.set_file_name(filename.unwrap().to_str().unwrap());
            code_writer = compile_vm_code(parser, code_writer, &is_test);
        }
    }
}
