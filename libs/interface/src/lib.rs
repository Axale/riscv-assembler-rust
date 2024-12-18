use std::collections::LinkedList;
use std::{clone, cmp};

// Fully Parsed Instruction, containing the machine code and the instruction address
#[derive(Debug ,serde::Deserialize)]
pub struct ParsedNode {
    pub instruction: u32,
    pub address:u16,
}

impl cmp::PartialEq for ParsedNode {
    fn eq(&self, other: &Self) -> bool {
        return (self.address == other.address) && (self.instruction == other.instruction);
    }
}

impl std::clone::Clone for ParsedNode{
    fn clone(&self) -> Self {
        ParsedNode { instruction: (self.instruction), address: (self.address) }
    }
}

#[derive(serde::Deserialize)]
pub struct DataInterface {
    lines : LinkedList<String>,
    parsed : LinkedList<ParsedNode>,
}

impl DataInterface {
    pub fn new() -> Self{
        Self{
            lines: LinkedList::new(),
            parsed: LinkedList::new(),
        }
    }

    pub fn add_line(&mut self, line : &str) {
        self.lines.push_back(line.to_string());
    }

    pub fn pop_line(&mut self) -> Option<String> {
        return self.lines.pop_front();
    }

    pub fn add_parsed(&mut self, parsed_node : &ParsedNode) {
        self.parsed.push_back(parsed_node.clone());
    }

    pub fn pop_parsed(&mut self) -> Option <ParsedNode> {
        return self.parsed.pop_front();
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    
    #[derive(serde::Deserialize)]
    struct Test<IN, CHK> {
        test_num : i32,
        input : IN,
        check_value : CHK
    }
    #[test]
    fn test_add_line() {
        let test_vec: Vec<Test<Vec<String>, LinkedList<String>>>;
        let path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_add_line.json");

        let content = fs::read_to_string(&path)
            .expect(&format!("Failed to read file: {:?}", path));

        test_vec = serde_json::from_str(&content).expect("Failed to parse JSON");

        for unit in test_vec.iter() {
            let mut di: Box<DataInterface> = Box::new(DataInterface::new());
            let test_num = unit.test_num;
            for st in unit.input.iter() {
                di.add_line(&st);
            }
            assert_eq!(di.lines, unit.check_value,
                "Test # `{test_num}` failed.");
            drop(di);
        }
    }

    #[test]
    fn test_add_parsed() {
        let test_vec: Vec<Test<Vec<ParsedNode>, LinkedList<ParsedNode>>>;
        let path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test_add_parsed.json");

        let content = fs::read_to_string(&path)
            .expect(&format!("Failed to read file: {:?}", path));

        test_vec = serde_json::from_str(&content).expect("Failed to parse JSON");

        for unit in test_vec.iter() {
            let mut di: Box<DataInterface> = Box::new(DataInterface::new());
            let test_num = unit.test_num;
            for st in unit.input.iter() {
                di.add_parsed(st);
            }
            assert_eq!(di.parsed, unit.check_value,
                "Test # `{test_num}` failed.");
            drop(di);
        }
    }
}