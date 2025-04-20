use crate::utils::parser::Parser;
use crate::utils::parser::fp_index_to_name;

pub struct RegisterFile {
    pub fp: [f64; 16],     // F0, F2, F4, ..., F30
    pub int: [i32; 32],    // R0, R1, R2, ..., R31
}

impl RegisterFile {
    pub fn new() -> Self {
        let mut int = [0; 32];
        int[1] = 16;

        Self {
            fp: [1.0; 16],
            int,
        }
    }

    pub fn read_fp(&self, name: &str) -> Option<f64> {
        Parser::parse_fp_register(name).map(|idx| self.fp[idx as usize])
    }

    pub fn write_fp(&mut self, name: &str, value: f64) -> bool {
        match Parser::parse_fp_register(name) {
            Some(idx) => {
                self.fp[idx as usize] = value;
                true
            }
            None => false,
        }
    }

    pub fn read_int(&self, name: &str) -> Option<i32> {
        Parser::parse_int_register(name).map(|idx| self.int[idx as usize])
    }

    pub fn write_int(&mut self, name: &str, value: i32) -> bool {
        match Parser::parse_int_register(name) {
            Some(idx) => {
                self.int[idx as usize] = value;
                true
            }
            None => false,
        }
    }
}

impl std::fmt::Display for RegisterFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.fp.iter().enumerate() {
            let name = fp_index_to_name(i as u8);
            writeln!(f, "{} = {}", name, v)?;
        }
        for (i, v) in self.int.iter().enumerate() {
            writeln!(f, "R{} = {}", i, v)?;
        }
        Ok(())
    }
}
