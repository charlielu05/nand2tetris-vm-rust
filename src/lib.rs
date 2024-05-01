pub mod compiler;

#[cfg(test)]
mod tests {
    use crate::compiler::{Parser, VmFile};
    use std::vec;

    #[test]
    fn create_vm_file() {
        let test_vm_file = VmFile::new("test").unwrap();
        assert!(test_vm_file.name() == "test");
    }

    #[test]
    fn test_parser() {
        let test_data = vec![
            "// comment".to_string(),
            "push constant 7".to_string(),
            "// comment 2".to_string(),
            "pop temp 8".to_string(),
            "add".to_string(),
        ];
        let mut parser = Parser::new(test_data);
        assert!(parser.currentInstruction == "push constant 7");
        assert!(parser.commandType().unwrap() == "C_PUSH");
        assert!(parser.arg1().unwrap() == "constant");
        assert!(parser.arg2().unwrap() == "7");

        parser.advance();
        assert!(parser.currentInstruction == "pop temp 8");
        assert!(parser.commandType().unwrap() == "C_POP");
        assert!(parser.arg1().unwrap() == "temp");
        assert!(parser.arg2().unwrap() == "8");

        parser.advance();
        assert!(parser.currentInstruction == "add");
        assert!(parser.commandType().unwrap() == "C_ARITHMETIC");
        assert!(parser.arg1().unwrap() == "add");
        assert!(parser.arg2() == None);
    }

    #[test]
    fn test_code_writer() {}
}
