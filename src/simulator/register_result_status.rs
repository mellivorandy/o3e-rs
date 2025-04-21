#[derive(Default)]
pub struct RegisterResultStatus {
    table: [Option<String>; 16],    // F0, F2, F4, ..., F30
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
