use crate::utils::parser::Parser;

pub struct RegisterFile {
    pub fp: [f64; 32],     // F0, F2, F4, ..., F30
    pub int: [i32; 32],    // R0, R1, R2, ..., R31
}

impl RegisterFile {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn read_fp(&self, name: &str) -> Option<f64> {
        Parser::parse_fp_register(name).map(|idx| self.fp[idx as usize])
    }

    #[allow(dead_code)]
    pub fn write_fp(&mut self, name: &str, value: f64) -> bool {
        match Parser::parse_fp_register(name) {
            Some(idx) => {
                self.fp[idx as usize] = value;
                true
            }
            None => false,
        }
    }

    #[allow(dead_code)]
    pub fn read_int(&self, name: &str) -> Option<i32> {
        Parser::parse_int_register(name).map(|idx| self.int[idx as usize])
    }

    #[allow(dead_code)]
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

impl Default for RegisterFile {
    fn default() -> Self {
        let mut int = [0; 32];
        int[1] = 16;
        
        let mut fp = [0.0; 32];
        for i in (0..32).step_by(2) {
            fp[i] = 1.0;
        }
        
        Self {
            fp: [1.0; 32],
            int,
        }
    }
}

impl std::fmt::Display for RegisterFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "------------- Floating Point Registers -------------")?;
        for (i, val) in self.fp.iter().enumerate() {
            if i % 2 == 0 {
                write!(f, "F{:<2} = {:<7.2} ", i, val)?;
                
                if (i / 2 + 1) % 4 == 0 {
                    writeln!(f)?;
                }
            }
        }
        writeln!(f)?;

        writeln!(f, "-------------------------------- Integer Registers ---------------------------------")?;
        for (i, v) in self.int.iter().enumerate() {
            write!(f, "R{:<2} = {:<4} ", i, v)?;
            
            if (i + 1) % 8 == 0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
