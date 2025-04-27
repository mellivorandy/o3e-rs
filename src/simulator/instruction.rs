use super::types::Cycle;

pub type FPRegister = u8;
pub type IntRegister = u8;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    LD,      // L.D
    SD,      // S.D
    ADDD,    // ADD.D
    SUBD,    // SUB.D
    MULTD,   // MUL.D
    DIVD,    // DIV.D
}

impl InstructionType {
    pub fn exec_cycles(&self) -> u32 {
        match self {
            InstructionType::LD => 2,
            InstructionType::SD => 1,
            InstructionType::ADDD => 2,
            InstructionType::SUBD => 2,
            InstructionType::MULTD => 10,
            InstructionType::DIVD => 40,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstructionMeta {
    pub inst_type: InstructionType,

    pub rd: Option<FPRegister>,
    pub rs: Option<FPRegister>,
    pub rt: Option<FPRegister>,
    
    pub base: Option<IntRegister>,
    pub offset: Option<i32>,           
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
        fn format_cycle(opt: Option<Cycle>) -> String {
            match opt {
                Some(c) => c.value().to_string(),
                None => "-".to_string(),
            }
        }

        write!(
            f,
            "{:<3}         {:<3}            {:<3}            {:<3}",
            format_cycle(self.issue),
            format_cycle(self.exec_start),
            format_cycle(self.completion),
            format_cycle(self.write_back),
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
