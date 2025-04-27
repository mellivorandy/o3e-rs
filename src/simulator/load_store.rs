use super::types::Cycle;

#[derive(Debug, Clone)]
pub enum StoreData {
    Ready(f64),         // V
    Waiting(String),    // Q
}

#[derive(Debug, Clone)]
pub struct LoadBuffer {
    pub name: String,
    pub busy: bool,
    pub dest: Option<usize>,
    pub offset: Option<i32>,
    pub base: Option<usize>,
    pub remaining_cycles: Option<Cycle>,
    pub inst_idx: Option<usize>,
}

impl LoadBuffer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            busy: false,
            dest: None,
            offset: None,
            base: None,
            remaining_cycles: None,
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
    pub data: Option<StoreData>,
    pub offset: Option<i32>,
    pub base: Option<usize>,
    pub remaining_cycles: Option<Cycle>,
    pub inst_idx: Option<usize>,
}

impl StoreBuffer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            busy: false,
            data: None,
            offset: None,
            base: None,
            remaining_cycles: None,
            inst_idx: None,
        }
    }

    pub fn clear(&mut self) {
        *self = StoreBuffer::new(&self.name);
    }
}

impl std::fmt::Display for LoadBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dest = match self.dest {
            Some(d) => format!("F{}", d),
            None => "-".to_string(),
        };
        let offset = match self.offset {
            Some(off) => off.to_string(),
            None => "-".to_string(),
        };
        let base = match self.base {
            Some(b) => format!("R{}", b),
            None => "-".to_string(),
        };
        let remain = match &self.remaining_cycles {
            Some(c) => format!("{}", c.value()),
            None => "-".to_string(),
        };
        
        write!(
            f,
            "{} | busy: {:<3} | dest: {:<2} | offset: {:<3} | base: {:<2} | remain: {:<2} | inst_idx: {:<2}",
            self.name,
            if self.busy { "Yes" } else { "No" },
            dest,
            offset,
            base,
            remain,
            self.inst_idx.map(|i| (i + 1).to_string()).unwrap_or_else(|| "-".to_string())
        )
    }
}

impl std::fmt::Display for StoreBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match &self.data {
            Some(StoreData::Ready(v)) => format!("V={}", v),
            Some(StoreData::Waiting(q)) => format!("Q={}", q),
            None => "-".to_string(),
        };
        let offset = match self.offset {
            Some(off) => off.to_string(),
            None => "-".to_string(),
        };
        let base = match self.base {
            Some(b) => format!("R{}", b),
            None => "-".to_string(),
        };
        let remain = match &self.remaining_cycles {
            Some(c) => format!("{}", c.value()),
            None => "-".to_string(),
        };

        write!(
            f,
            "{} | busy: {:<3} | data: {:<8} | offset: {:<3} | base: {:<2} | remain: {:<2} | inst_idx: {}",
            self.name,
            if self.busy { "Yes" } else { "No" },
            data,
            offset,
            base,
            remain,
            self.inst_idx.map(|i| (i + 1).to_string()).unwrap_or_else(|| "-".to_string())
        )
    }
}
