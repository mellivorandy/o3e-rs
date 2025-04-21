#[derive(Debug, Clone)]
pub enum StoreData {
    Ready(f64),         // V
    Waiting(String),    // Q
}

#[derive(Debug, Clone)]
pub struct LoadBuffer {
    pub name: String,
    pub busy: bool,

    pub base: usize,
    pub offset: i32,

    pub dest: usize,

    pub remaining_cycles: u32,
    pub inst_idx: Option<usize>,
}

impl LoadBuffer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            busy: false,
            base: 0,
            offset: 0,
            dest: 0,
            remaining_cycles: 0,
            inst_idx: None,
        }
    }

    pub fn clear(&mut self) {
        *self = LoadBuffer::new(&self.name);
    }
}

#[derive(Debug, Clone)]
pub struct StoreBuffer {
    pub name: String,
    pub busy: bool,

    pub base: usize,
    pub offset: i32,

    pub data: Option<StoreData>,

    pub remaining_cycles: u32,
    pub inst_idx: Option<usize>,
}

impl StoreBuffer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            busy: false,
            base: 0,
            offset: 0,
            data: None,
            remaining_cycles: 0,
            inst_idx: None,
        }
    }

    pub fn clear(&mut self) {
        *self = StoreBuffer::new(&self.name);
    }
}
