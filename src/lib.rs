pub mod compiler;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::compiler::{VmFile, Parser, CodeWriter};
    #[test]
    fn create_vm_file() {
        let test_vm_file = VmFile::new("test").unwrap();
        assert!(test_vm_file.name() == "test");
    }

    #[test]
    fn test_parser() {
        let test_data = vec!["push constant 7".to_string(), "push constant 8".to_string(), "add".to_string()];
        let mut parser = Parser::new(test_data);
        
        parser.advance();
        dbg!(parser.currentInstruction.clone());
        assert!(parser.currentInstruction == "push constant 8");
    }
}
