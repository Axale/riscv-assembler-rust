use std::{fmt, fs, path::PathBuf};
data_structures::*
use serde::*;
#[derive(serde::Deserialize, Debug)]

pub struct FileIO {
}

impl FileIO {
    pub fn open_code(&mut self, path: &PathBuf, di : &mut DataInterface){
        let content: String = fs::read_to_string(path)
            .expect("Failed to read code!");
        let strvec = content.split('\n');
        
        for line in strvec.into_iter() {
            di.add_line(line);
        }
        return;
    }

    // Takes each parsed instruction and forms a line of intel hex with it
    fn form_line(node : &ParsedNode) -> Option<String> {
        let mut line: String = "04".to_owned();
        // Formats address to only 4 bytes
        line.push_str(&format!("{:04X}", node.address)[..]);
        line.push_str("00");
        // Formats instruction
        line.push_str(&format!("{:08X}", node.instruction)[..]);
        
        // Checksum calculations 
        let mut checksum : u8 = 0;

        // Converts the string into a vector of charater pairs, as checksum is done by byte.
        let pair_vect : Vec<String> = line.as_bytes()
            .chunks(2)
            .map(|chunk| String::from_utf8_lossy(chunk).to_string())
            .collect();

        // Iterates through pair vector in order to calculate checksum.
        for pair in pair_vect.iter() {
            let fm_hex =u8::from_str_radix(pair, 16)
                .expect("Invalid String");
            checksum = checksum.wrapping_add(fm_hex);
        }
        checksum = !checksum + 1;

        // Putting together the final return line.
        line.push_str(&format!("{:02X}", checksum));
        let mut ret_string: String = ":".to_owned();
        ret_string.push_str(&line[..]);
        ret_string.push('\n');

        return Some(ret_string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Deserialize)]
    struct Test<IN, CHK> {
        test_num : i32,
        input : IN,
        check_value : CHK
    }

    fn load_tests<IN, CHK>(json_name : &str) -> Vec<Test<IN, CHK>> 
    where 
        IN: serde::de::DeserializeOwned, // IN must implement deserialize
        CHK: serde::de::DeserializeOwned, // OUT must implement deserialize
    {
        let path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join(json_name);

        let content = fs::read_to_string(&path)
            .expect(&format!("Failed to read file: {:?}", path));
        return serde_json::from_str(&content).expect("Failed to parse JSON");
    }
    #[test]
    fn test_form_line() {
        let tests: Vec<Test<ParsedNode, String>>= load_tests::<ParsedNode, String>("test_form_line.json");
        
        for curr_test in tests.iter() {
            let test_num = curr_test.test_num;
            let out = FileIO::form_line(&curr_test.input).expect("form_line failed!");
            assert_eq!(out, curr_test.check_value,
                "Test # `{test_num}` failed.");
        }
    }
    
}
