#[derive(Default)]
pub struct RegisterResultStatus {
    pub table: [Option<String>; 16],    // F0, F2, F4, ..., F30
}

impl RegisterResultStatus {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn fp_num_to_index(fp_num: usize) -> usize { fp_num / 2 }

    pub fn get(&self, fp_num: usize) -> Option<&String> {
        self.table[Self::fp_num_to_index(fp_num)].as_ref()
    }

    pub fn set(&mut self, fp_num: usize, station: String) {
        self.table[Self::fp_num_to_index(fp_num)] = Some(station);
    }

    pub fn clear(&mut self, fp_num: usize) {
        self.table[Self::fp_num_to_index(fp_num)] = None;
    }
}

impl std::fmt::Display for RegisterResultStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, entry) in self.table.iter().enumerate() {
            let name = format!("F{}", i * 2);
            
            match entry {
                Some(station) => write!(f, "{:<3}: {:<7}", name, station)?,
                None => write!(f, "{:<3}: {:<7}", name, "-")?,
            }
            
            if (i + 1) % 4 == 0 {
                writeln!(f)?;
            }
        }
        
        Ok(())
    }
}
