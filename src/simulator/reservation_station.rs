use super::{instruction::InstructionType, types::Cycle};

#[derive(Debug, Clone)]
pub struct ReservationStation {
    pub name: String,                          // Name of the station, e.g., Add1 

    pub op: Option<InstructionType>,    // Type of the instruction, e.g., MUL.D

    pub vj: Option<f64>,
    pub vk: Option<f64>,

    pub qj: Option<String>,
    pub qk: Option<String>,

    pub busy: bool,

    pub remaining_cycles: Option<Cycle>,
    pub inst_idx: Option<usize>,               // Index of the instruction
}

impl ReservationStation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        *self = ReservationStation::new(&self.name);
    }
}

impl Default for ReservationStation {
    fn default() -> Self {
        Self {
            name: String::new(),
            op: None,
            vj: None,
            vk: None,
            qj: None,
            qk: None,
            busy: false,
            remaining_cycles: None,
            inst_idx: None,
        }
    }
}

impl std::fmt::Display for ReservationStation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let remain = match &self.remaining_cycles {
            Some(c) => format!("{}", c.value()),
            None => "-".to_string(),
        };
        
        write!(
            f,
            "{:<5} | busy: {:<3} | op: {:<6} | vj: {:<6} | vk: {:<6} | qj: {:<6} | qk: {:<6} | remain: {:<2} | inst_idx: {}",
            
            self.name,
            if self.busy { "Yes" } else { "No" },

            match &self.op {
                Some(inst_type) => format!("{:?}", inst_type),
                None => "-".to_string(),
            },

            match self.vj {
                Some(v) => format!("{:.2}", v),
                None => "-".to_string(),
            },

            match self.vk {
                Some(v) => format!("{:.2}", v),
                None => "-".to_string(),
            },

            self.qj.as_deref().unwrap_or("-"),
            self.qk.as_deref().unwrap_or("-"),

            remain,

            match self.inst_idx {
                Some(idx) => idx.to_string(),
                None => "-".to_string(),
            }
        )
    }
}
