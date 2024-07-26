pub struct Parser {
    pub contents: Vec<String>,
    pub currentLine: usize,
    pub currentInstruction: String,
}

impl Parser {
    pub fn new(code_lines: Vec<String>) -> Self {
        let mut new_parser = Parser {
            contents: code_lines,
            currentLine: 0,
            currentInstruction: "".to_string(),
        };
        // set the initial instruction
        // new_parser.advance();
        new_parser
    }

    pub fn current_instruction(&self) -> String {
        self.currentInstruction.to_owned()
    }

    pub fn hasMoreLines(&self) -> bool {
        self.currentLine <= self.contents.len() - 1
    }

    fn increment_line(&mut self) {
        if self.hasMoreLines() {
            self.currentLine += 1;
        }
    }
    fn set_instruction(&mut self) {
        self.currentInstruction = self.contents[self.currentLine].to_string();
    }

    pub fn advance(&mut self) {
        if self.contents[self.currentLine].starts_with("//")
            | self.contents[self.currentLine].is_empty()
        {
            self.increment_line();
            self.advance();
        } else {
            self.set_instruction();
            self.increment_line();
        }
    }

    pub fn commandType(&self) -> Result<&str, &str> {
        if self.currentInstruction.starts_with("push") {
            return Ok("C_PUSH");
        } else if self.currentInstruction.starts_with("pop") {
            return Ok("C_POP");
        } else if self.currentInstruction.starts_with("label") {
            return Ok("C_LABEL");
        } else if self.currentInstruction.starts_with("if-goto") {
            return Ok("C_IFGOTO");
        } else if self.currentInstruction.starts_with("goto") {
            return Ok("C_GOTO");
        } else if ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"]
            .contains(&self.currentInstruction.as_str())
        {
            return Ok("C_ARITHMETIC");
        } else {
            return Err("could not match command");
        };
    }

    pub fn arg1(&self) -> Option<String> {
        match self.commandType() {
            Ok(cmd) => match cmd {
                "C_PUSH" | "C_POP" | "C_LABEL" | "C_IFGOTO" | "C_GOTO" => {
                    let parts: Vec<&str> = self.currentInstruction.split_whitespace().collect();
                    Some(parts[1].to_string())
                }
                "C_ARITHMETIC" => Some(self.currentInstruction.to_string()),
                _ => None,
            },
            Err(_) => None,
        }
    }

    pub fn arg2(&self) -> Option<String> {
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
