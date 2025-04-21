use super::types::Cycle;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    LD,      // L.D
    SD,      // S.D
    ADDD,    // ADD.D
    SUBD,    // SUB.D
    MULD,    // MUL.D
    DIVD,    // DIV.D
}

impl InstructionType {
    pub fn exec_cycles(&self) -> i32 {
        match self {
            InstructionType::LD => 2,
            InstructionType::SD => 1,
            InstructionType::ADDD => 2,
            InstructionType::SUBD => 2,
            InstructionType::MULD => 10,
            InstructionType::DIVD => 40,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstructionMeta {
    pub inst_type: InstructionType,

    pub rd: Option<u8>,
    pub rs: Option<u8>,
    pub rt: Option<u8>,
    
    pub base: Option<u8>,
    pub offset: i32,
}

#[derive(Debug, Clone, Default)]
pub struct InstructionTime {
    pub issue: Option<Cycle>,
    pub exec_start: Option<Cycle>,
    pub completion: Option<Cycle>,
    pub write_back: Option<Cycle>,
}

impl std::fmt::Display for InstructionTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Issue = {:?}, Exec start = {:?}, completion = {:?}, Write_back = {:?}",
            self.issue, self.exec_start, self.completion, self.write_back
        )
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub meta: InstructionMeta,
    pub time: InstructionTime,
}

impl Instruction {
    #[inline]
    pub fn new(meta: InstructionMeta) -> Self {
        Self { meta, time: InstructionTime::default(), }
    }
}
