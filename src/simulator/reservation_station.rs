use super::instruction::InstructionType;

#[derive(Debug, Clone)]
pub struct ReservationStation {
    pub name: String,                          // Name of the station, e.g., Add1 

    pub inst_type: Option<InstructionType>,    // Type of the instruction, e.g., MUL.D

    pub vj: Option<f64>,
    pub vk: Option<f64>,

    pub qj: Option<String>,
    pub qk: Option<String>,

    pub busy: bool,

    pub remaining_cycles: u32,
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
            inst_type: None,
            vj: None,
            vk: None,
            qj: None,
            qk: None,
            busy: false,
            remaining_cycles: 0,
            inst_idx: None,
        }
    }
}
