use super::instruction::InstructionType;

#[derive(Debug, Clone)]
pub struct ReservationStation {
    pub name: String,

    pub op: Option<InstructionType>,

    pub vj: Option<f64>,
    pub vk: Option<f64>,

    pub qj: Option<String>,
    pub qk: Option<String>,

    pub busy: bool,

    pub remaining_cycles: u32,
    pub inst_idx: Option<usize>,
}
