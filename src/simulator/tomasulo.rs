use crate::simulator::instruction::*;
use crate::simulator::reservation_station::*;
use crate::simulator::load_store::*;
use crate::simulator::register_result_status::*;
use crate::simulator::register::*;
use crate::simulator::types::Cycle;
use crate::utils::parser::Parser;

pub struct Tomasulo {
    // Instruction stream
    instructions: Vec<Instruction>,
    
    add_stations: Vec<ReservationStation>,
    mul_stations: Vec<ReservationStation>,

    load_buffers: Vec<LoadBuffer>,
    store_buffers: Vec<StoreBuffer>,

    registers: RegisterFile,

    f_register_status: RegisterResultStatus,

    // 8 f64 slots (64 Bytes)
    memory: [f64; 8],

    // Global cycle counter
    current_cycle: Cycle,
}

impl Tomasulo {
    pub fn new() -> Self {
        let add_stations: Vec<_> = (1..=3)
            .map(|i| ReservationStation::new(&format!("Add{}", i)))
            .collect();
        
        let mul_stations = vec![
            ReservationStation::new("Mul1"),
            ReservationStation::new("Mul2"),
        ];

        let load_buffers = vec![
            LoadBuffer::new("Load1"),
            LoadBuffer::new("Load2"),
        ];

        let store_buffers = vec![
            StoreBuffer::new("Store1"),
            StoreBuffer::new("Store2"),
        ];

        Self {
            instructions: Vec::new(),
            add_stations,
            mul_stations,
            load_buffers,
            store_buffers,
            registers: RegisterFile::default(),
            f_register_status: RegisterResultStatus::new(),
            memory: [1.0; 8],
            current_cycle: Cycle::new(0),
        }
    }
}
