use std::array;

use data_structures::*;
pub struct  Translator <'a> {
    inst_hm : HashMap<Vec<Inst>>, // HashMap with instructions
    reg_hm: HashMap<Reg>, // Hashmap with registers
    curr_address : u16,
    di: &'a mut DataInterface
}

impl <'a> Translator <'a> {
    pub fn new(inst_size : u32, reg_size: u32, __di : &'a mut DataInterface) -> Self{
        Self {
            inst_hm : HashMap::new(inst_size),
            reg_hm : HashMap::new(reg_size),
            di : __di,
            curr_address: 0
        }
    }

    // Initializes both hashmaps
    pub fn initialize(&mut self, extracted_instuctions : &Vec::<ExtractedData<Vec<Inst>>>, extracted_regs : &Vec::<ExtractedData<Reg>>) {
        self.inst_hm.build(extracted_instuctions);
        self.reg_hm.build(extracted_regs);
        return;
    }

    // Pops a new line, splits the line, and parses the instruction and registers/immediates.
    // At the moment, ignores everything past the 3rd argument, will be fixed later.
    fn rtype (&mut self, new_parsed : &mut ParsedNode, broken_line : &Vec<&str>) -> bool {
        
        // Adds each of the operators, shofting them over.
        let shift_arr = [0, 7, 15, 20];
        for i in [1, 2, 3] {
            let reg_opt = self.reg_hm.get(broken_line[i]);
            if reg_opt.is_none() {
                return false;
            }

            let reg = reg_opt.unwrap();           
            new_parsed.instruction |= (0b11111 & reg.reg_num) << shift_arr[i];
        }
        
        return true;
    }

    // i-type
    // Will need to add compatability for hex
    fn itype (&mut self, new_parsed : &mut ParsedNode, broken_line : &Vec<&str>) -> bool {
        
        // Adds each of the operators, shofting them over.
        let shift_arr = [0, 7, 15];
        for i in [1, 2] {
            let reg_opt = self.reg_hm.get(broken_line[i]);
            if reg_opt.is_none() {
                return false;
            }

            let reg = reg_opt.unwrap();           
            new_parsed.instruction |= (0b11111 & reg.reg_num) << shift_arr[i];
        }
        new_parsed.instruction |= u32::from_str_radix(broken_line[3], 10).expect("Bad immediate")
            << 20;
        
        return true;
    }
    
    // No big difference for this one, just the immediate is broken up into two
    fn stype (&mut self, new_parsed : &mut ParsedNode, broken_line : &Vec<&str>) -> bool {
        
        // Adds each of the operators, shofting them over.
        let shift_arr = [0, 15, 20];
        for i in [1, 2] {
            let reg_opt = self.reg_hm.get(broken_line[i]);
            if reg_opt.is_none() {
                return false;
            }

            let reg = reg_opt.unwrap();           
            new_parsed.instruction |= (0b11111 & reg.reg_num) << shift_arr[i];
        }
        let imm = u32::from_str_radix(broken_line[3], 10).expect("Bad immediate");
        new_parsed.instruction |= (imm & 0b11111) << 7;
        new_parsed.instruction |= (imm & 0b111111100000) << 20;
        
        return true;
    }

    // Also similar to S-Type
    fn btype (&mut self, new_parsed : &mut ParsedNode, broken_line : &Vec<&str>) -> bool {
        
        // Adds each of the operators, shofting them over.
        let shift_arr = [0, 15, 20];
        for i in [1, 2] {
            let reg_opt = self.reg_hm.get(broken_line[i]);
            if reg_opt.is_none() {
                return false;
            }

            let reg = reg_opt.unwrap();           
            new_parsed.instruction |= (0b11111 & reg.reg_num) << shift_arr[i];
        }
        let imm = u32::from_str_radix(broken_line[3], 10).expect("Bad immediate");
        new_parsed.instruction |= (imm & 0b11110 | ((imm & 0x800) >> 11)) << 7;
        new_parsed.instruction |= ((imm & 0x400 >> 1) |  (0b1111100000 & imm)) << 20;
        
        return true;
    }
    

    fn utype (&mut self, new_parsed : &mut ParsedNode, broken_line : &Vec<&str>) -> bool {
        
        // Adds each of the operators, shofting them over.
        let shift_arr = [7];
        for i in [1] {
            let reg_opt = self.reg_hm.get(broken_line[i]);
            if reg_opt.is_none() {
                return false;
            }

            let reg = reg_opt.unwrap();           
            new_parsed.instruction |= (0b11111 & reg.reg_num) << shift_arr[i];
        }
        let imm = u32::from_str_radix(broken_line[3], 10).expect("Bad immediate");
        new_parsed.instruction |= (imm & 0xFFFFF000);
        
        return true;
    }
    
    fn jtype (&mut self, new_parsed : &mut ParsedNode, broken_line : &Vec<&str>) -> bool {
        
        // Adds each of the operators, shofting them over.
        let shift_arr = [7];
        for i in [1] {
            let reg_opt = self.reg_hm.get(broken_line[i]);
            if reg_opt.is_none() {
                return false;
            }

            let reg = reg_opt.unwrap();           
            new_parsed.instruction |= (0b11111 & reg.reg_num) << shift_arr[i];
        }
        let imm = u32::from_str_radix(broken_line[3], 10).expect("Bad immediate");
        // J types have a weird bit placement
        new_parsed.instruction |= imm & 0x000FF000;
        new_parsed.instruction |= (imm & 0x000800) << 9;
        new_parsed.instruction |= (imm & 0x000007FF) << 20;
        new_parsed.instruction |= (imm & 0x00100000) << 11;
        
        return true;
    }
    // Parses a line, breaks the line up into a vector of strings (commas and whitespace used to split)
    // Determines the instruction type and calls the appropriate command
    // Will need some work to add compatibility to meta-instructions (i.e. placing data at specific addresses while assembling)
    // Also needs some work to make it work with address labels.
    // Returns a bool to indicate success
    fn parse_line(&mut self) -> bool {
        let line_opt = self.di.pop_line();
        if line_opt.is_none() {
            return false;
        }        

        let curr_line = line_opt.unwrap();

        let broken_line: Vec<&str> = curr_line.split(|c| c == ',' || c == ' ').collect();

        let inst_opt = self.inst_hm.get(broken_line[0]);
        if inst_opt.is_none() {
            return false;
        } 
        
        // Vector of instructions that make up an instruction.
        // This so that this can work for both regular and pseudo-instructions
        let inst_vector = inst_opt.unwrap().clone();

        for inst in inst_vector.iter() {
            // Instruction Types, (based on RISC-V Standard)
            let mut new_parsed = ParsedNode{
                instruction: inst.funct3 | inst.opcode | inst.funct7, 
                address: self.curr_address
            };
            let mut success = false;
            match inst.inst_type {
                InstType::R => {
                    success = self.rtype(&mut new_parsed, &broken_line);
                },
                InstType::I=> {
                    success = self.itype(&mut new_parsed, &broken_line);
                },
                InstType::S=> {
                    success = self.stype(&mut new_parsed, &broken_line);
                },
                InstType::B=> {
                    success = self.btype(&mut new_parsed, &broken_line);
                },
                InstType::U=> {
                    success = self.utype(&mut new_parsed, &broken_line);
                },
                InstType::J=>{
                    success = self.jtype(&mut new_parsed, &broken_line);
                },
                _=> return false
            }
            if success == false {
                return false;
            }

            self.di.add_parsed(&new_parsed);
        }

        return true;
    }

    pub fn parse_file(&mut self) {
        let mut line_num: u32 = 1;

        while self.di.lines_len() > 0 {
            let success = self.parse_line();
            if !success {
                panic!("Failed at line #`{line_num}`");
            }
            line_num += 1;
            self.curr_address += 1;
        }
    }
}