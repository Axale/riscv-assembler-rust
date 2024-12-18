use std::collections::LinkedList;
use std::cmp;


#[derive(Clone)]

pub enum InstType {R, I, S, B, U, J}


#[derive(Clone)]
pub struct Reg {
    pub reg_num : u32  // register number
}

#[derive(Clone)]
pub struct Inst {
    pub opcode :  u32, // Opcode field
    pub funct3  : u32, // Funct3 field
    pub funct7  : u32, // Funct7 field
    pub inst_type : InstType // Instruction type
}
pub struct ExtractedData<T>
where 
    T : std::clone::Clone
{
    data: T, // Data
    key: String //
}

#[derive(Clone)]
pub struct HashNode<T> 
where 
    T : std::clone::Clone
{
    data : T, // Data
    key : u32, 
    next_node : Option<Box<HashNode<T>>>
}

pub struct HashMap<T> 
where 
    T : std::clone::Clone
{
    size: u32, // size, will use as modulo
    hash_vect : Vec<Option<Box<HashNode<T>>>>
}

impl<T> HashMap<T> 
where 
    T : std::clone::Clone
{
    pub fn new(size : u32) -> Self {
        Self { 
            hash_vect: vec![None; size as usize],
            size: size, 
        }
    }

    // Hashes a string
    pub fn hash_str(&self, key: &str) -> u32 {
        let n: u32 = 1;
        let mut hash_key: u32 = 0;
        
        for chr in key.chars() {
            if !chr.is_alphanumeric() {
                panic!("Invalid Character");
            }
            // 
            hash_key +=  (chr.to_ascii_lowercase() as u32) * n;
        }
        return hash_key;
    }


    fn insert(&mut self, dat : &T, key : &str) {
        if key.len() > 10 {
            panic!("Invalid Instruction!");
        }
        let hash_key = HashMap::hash_str(&self,key);
        let index = (hash_key % self.size) as usize;


        let mut new_node = Box::new(
            HashNode {
                data: dat.clone(),
                key: hash_key,
                next_node: None
            }
        );

        new_node.next_node = self.hash_vect[index].take();

        self.hash_vect[index] = Some(new_node);
        return;
    }

    pub fn build(&mut self, extracted_data : &Vec::<ExtractedData<T>>) {
        for node in extracted_data{
            self.insert(&node.data, &(node.key[..]));
        }
    }

    pub fn get(&self, key : &str) -> Option<&T>{
        
        if key.len() > 10 {
            return None
        }

        let hash = HashMap::hash_str(&self, key);
        let index = (hash % self.size) as usize;

        let mut curr = &self.hash_vect[index];
        if curr.is_none() {
            panic!("Invalid Key!"); // Panics to be removed in the future
        }

        while let Some(node) = curr {
            if node.key == hash {
                return Some(&node.data);
            }
            curr = &node.next_node;
        }

        None
    }
}


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

    pub fn lines_len(&self) -> usize {
        self.lines.len()
    }
    
    pub fn parsed_len(&self) -> usize {
        self.parsed.len()
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