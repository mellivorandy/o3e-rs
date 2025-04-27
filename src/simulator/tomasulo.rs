use std::fs::OpenOptions;

use crate::simulator::instruction::*;
use crate::simulator::reservation_station::*;
use crate::simulator::load_store::*;
use crate::simulator::register_result_status::*;
use crate::simulator::register::RegisterFile;
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
    pub fn new(instructions: Vec<Instruction>) -> Self {
        let add_stations: Vec<_> = (1..=3)
            .map(|i| ReservationStation::new(&format!("Add{}", i)))
            .collect();
        
        let mul_stations = vec![
            ReservationStation::new("Mult1"),
            ReservationStation::new("Mult2"),
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
            instructions,
            add_stations,
            mul_stations,
            load_buffers,
            store_buffers,
            registers: RegisterFile::new(),
            f_register_status: RegisterResultStatus::new(),
            memory: [1.0; 8],
            current_cycle: Cycle::new(0),
        }
    }

    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let metas = Parser::parse_file(&content);
        let instructions = metas.into_iter()
            .map(Instruction::new)
            .collect();
        Ok(Self::new(instructions))
    }

    pub fn issue_stage(&mut self) {
        for (i, inst) in self.instructions.iter_mut().enumerate() {
            if inst.time.issue.is_some() {
                continue;
            }

            let meta = &inst.meta;

            match meta.inst_type {
                InstructionType::LD => {
                    if let Some(lb) = self.load_buffers.iter_mut().find(|b| !b.busy) {
                        let rd = meta.rd.expect("LD instruction missing destination register");
                        let offset = meta.offset.expect("LD instruction missing offset");
                        let base = meta.base.expect("LD instruction missing base register");
                         
                        lb.busy = true;
                        lb.dest = Some(rd as usize);
                        lb.offset = Some(offset);
                        lb.base = Some(base as usize);
                        
                        lb.remaining_cycles = Some(Cycle::new(meta.inst_type.exec_cycles() as u32));
                        lb.inst_idx = Some(i);
                        self.f_register_status.set(rd as usize, lb.name.clone());
                        inst.time.issue = Some(self.current_cycle);
                        break;
                    }
                }
    
                InstructionType::SD => {
                    if let Some(sb) = self.store_buffers.iter_mut().find(|b| !b.busy) {
                        let rs = meta.rs.expect("SD instruction missing source register");
                        let base = meta.base.expect("SD instruction missing base register");
                        let offset = meta.offset.expect("SD instruction missing offset");
                        
                        sb.busy = true; 
                        sb.data = match self.f_register_status.get(rs as usize) {
                            Some(station) => Some(StoreData::Waiting(station.clone())),
                            None => Some(StoreData::Ready(self.registers.fp[rs as usize])),
                        };
                        sb.offset = Some(offset);
                        sb.base = Some(base as usize);
                        
                        sb.remaining_cycles = Some(Cycle::new(meta.inst_type.exec_cycles() as u32));
                        sb.inst_idx = Some(i);
                        inst.time.issue = Some(self.current_cycle);
                        break;
                    }
                }
    
                InstructionType::ADDD | InstructionType::SUBD => {
                    if let Some(st) = self.add_stations.iter_mut().find(|s| !s.busy) {
                        st.busy = true;
                        st.op = Some(meta.inst_type.clone());
                        st.remaining_cycles = Some(Cycle::new(meta.inst_type.exec_cycles() as u32));
                        st.inst_idx = Some(i);
    
                        if let Some(fd) = meta.rd {
                            self.f_register_status.set(fd.into(), st.name.clone());
                        }
    
                        if let Some(rs) = meta.rs {
                            if let Some(qj) = self.f_register_status.get(rs.into()) {
                                // not ready yet
                                st.vj = None;
                                st.qj = Some(qj.clone());
                            } else {
                                // ready
                                st.vj = Some(self.registers.fp[rs as usize]);
                                st.qj = None;
                            }
                        }
    
                        if let Some(rt) = meta.rt {
                            if let Some(qk) = self.f_register_status.get(rt.into()) {
                                st.vk = None;
                                st.qk = Some(qk.clone());
                            } else {
                                st.vk = Some(self.registers.fp[rt as usize]);
                                st.qk = None;
                            }
                        }
    
                        inst.time.issue = Some(self.current_cycle);
                        break;
                    }
                }
    
                InstructionType::MULTD | InstructionType::DIVD => {
                    if let Some(st) = self.mul_stations.iter_mut().find(|s| !s.busy) {
                        st.busy = true;
                        st.op = Some(meta.inst_type.clone());
                        st.remaining_cycles = Some(Cycle::new(meta.inst_type.exec_cycles() as u32));
                        st.inst_idx = Some(i);
    
                        if let Some(fd) = meta.rd {
                            self.f_register_status.set(fd.into(), st.name.clone());
                        }
    
                        if let Some(rs) = meta.rs {
                            if let Some(qj) = self.f_register_status.get(rs.into()) {
                                st.vj = None;
                                st.qj = Some(qj.clone());
                            } else {
                                st.vj = Some(self.registers.fp[rs as usize]);
                                st.qj = None;
                            }
                        }
    
                        if let Some(rt) = meta.rt {
                            if let Some(qk) = self.f_register_status.get(rt.into()) {
                                st.vk = None;
                                st.qk = Some(qk.clone());
                            } else {
                                st.vk = Some(self.registers.fp[rt as usize]);
                                st.qk = None;
                            }
                        }
    
                        inst.time.issue = Some(self.current_cycle);
                        break;
                    }
                }
            }
        }
    }

    pub fn execute_stage(&mut self) {
        for st in self.add_stations.iter_mut() {
            if st.busy && st.op.is_some() {
                if let Some(cycle) = st.remaining_cycles.as_mut() {
                    if cycle.value() > 0 {
                        if let Some(idx) = st.inst_idx {
                            let inst = &mut self.instructions[idx];
                            
                            if inst.time.exec_start.is_none() {
                                // issued in this cycle
                                inst.time.exec_start = Some(self.current_cycle);
                            } else {
                                // the cycles afterwards
                                cycle.tick_down();
                            }
                
                            if cycle.value() == 0 {
                                inst.time.completion = Some(self.current_cycle);
                            }
                        }
                    }
                }
            }
        }

        for st in self.mul_stations.iter_mut() {
            if st.busy && st.op.is_some() {
                if let Some(cycle) = st.remaining_cycles.as_mut() {
                    if cycle.value() > 0 {
                        if let Some(idx) = st.inst_idx {
                            let inst = &mut self.instructions[idx];
                            
                            if inst.time.exec_start.is_none() {
                                inst.time.exec_start = Some(self.current_cycle);
                            } else {
                                cycle.tick_down();
                            }
                
                            if cycle.value() == 0 {
                                inst.time.completion = Some(self.current_cycle);
                            }
                        }
                    }
                }
            }
        }

        for lb in self.load_buffers.iter_mut() {
            if let Some(cycle) = lb.remaining_cycles.as_mut() {
                if cycle.value() > 0 {
                    if let Some(idx) = lb.inst_idx {
                        let inst = &mut self.instructions[idx];
                        
                        if inst.time.exec_start.is_none() {
                            inst.time.exec_start = Some(self.current_cycle);
                        } else {
                            cycle.tick_down();
                        }
            
                        if cycle.value() == 0 {
                            inst.time.completion = Some(self.current_cycle);
                        }
                    }
                }
            }
        }
        
        for sb in self.store_buffers.iter_mut() {
            if sb.busy {
                let data_ready = matches!(sb.data, Some(StoreData::Ready(_)));
                
                if !data_ready {
                    continue;
                }
            
                if let Some(cycle) = sb.remaining_cycles.as_mut() {
                    if cycle.value() > 0 {
                        if let Some(idx) = sb.inst_idx {
                            let inst = &mut self.instructions[idx];
                            
                            if inst.time.exec_start.is_none() {
                                inst.time.exec_start = Some(self.current_cycle);
                            } else {
                                cycle.tick_down();
                            }
                
                            if cycle.value() == 0 {
                                inst.time.completion = Some(self.current_cycle);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn writeback_stage(&mut self) {
        let mut to_release: Vec<String> = Vec::new();
        let mut broadcast_list = Vec::new();
        
        for st in self.add_stations.iter_mut() {
            if st.busy && st.inst_idx
                .and_then(|idx| self.instructions.get(idx))
                .and_then(|inst| inst.time.completion)
                .map_or(false, |comp| comp < self.current_cycle)
            {
                if let Some(idx) = st.inst_idx {
                    let inst = &mut self.instructions[idx];
                    
                    if inst.time.write_back.is_none() {
                        let val = match st.op.as_ref().unwrap() {
                            InstructionType::ADDD => st.vj.unwrap() + st.vk.unwrap(),
                            InstructionType::SUBD => st.vj.unwrap() - st.vk.unwrap(),
                            _ => unreachable!(),
                        };
                        
                        let dest = inst.meta.rd.expect("Missing rd in ADD/SUB instruction") as usize;
                        
                        inst.time.write_back = Some(self.current_cycle);
                        broadcast_list.push((st.name.clone(), val, dest));

                        st.remaining_cycles = None;
                        to_release.push(st.name.clone());
                    }
                }
            }
        }
        
        for st in self.mul_stations.iter_mut() {
            if st.busy && st.inst_idx
                .and_then(|idx| self.instructions.get(idx))
                .and_then(|inst| inst.time.completion)
                .map_or(false, |comp| comp < self.current_cycle)
            {
                if let Some(idx) = st.inst_idx {
                    let inst = &mut self.instructions[idx];
                    
                    if inst.time.write_back.is_none() {
                        let val = match st.op.as_ref().unwrap() {
                            InstructionType::MULTD => st.vj.unwrap() * st.vk.unwrap(),
                            InstructionType::DIVD => st.vj.unwrap() / st.vk.unwrap(),
                            _ => unreachable!(),
                        };

                        let dest = inst.meta.rd.expect("Missing rd in MUL/DIV instruction") as usize;
                        
                        inst.time.write_back = Some(self.current_cycle);
                        broadcast_list.push((st.name.clone(), val, dest));

                        st.remaining_cycles = None;
                        to_release.push(st.name.clone());
                    }
                }
            }
        }

        for lb in self.load_buffers.iter_mut() {
            if lb.busy && lb.inst_idx
                .and_then(|idx| self.instructions.get(idx))
                .and_then(|inst| inst.time.completion)
                .map_or(false, |comp| comp < self.current_cycle)
            {
                if let Some(idx) = lb.inst_idx {
                    let inst = &mut self.instructions[idx];
                    
                    if inst.time.write_back.is_none() {
                        let base = lb.base.expect("LoadBuffer missing base register");
                        let offset = lb.offset.expect("LoadBuffer missing offset");
                        
                        let addr = (self.registers.int[base] + offset) / 8;
                        let val = self.memory[addr as usize];
                        
                        inst.time.write_back = Some(self.current_cycle);
                        
                        if let Some(dest) = lb.dest {
                            broadcast_list.push((lb.name.clone(), val, dest));
                        }

                        lb.remaining_cycles = None;
                        to_release.push(lb.name.clone());
                    }
                }
            }
        }

        for sb in self.store_buffers.iter_mut() {
            if sb.busy && sb.inst_idx
                .and_then(|idx| self.instructions.get(idx))
                .and_then(|inst| inst.time.completion)
                .map_or(false, |comp| comp < self.current_cycle)
            {
                if let Some(idx) = sb.inst_idx {
                    let inst = &mut self.instructions[idx];
                    
                    if inst.time.write_back.is_none() {
                        let base = sb.base.expect("StoreBuffer missing base register");
                        let offset = sb.offset.expect("StoreBuffer missing offset");
                        
                        let addr = (self.registers.int[base] + offset) / 8;
                        
                        let store_val = match sb.data.as_ref().expect("Missing store data") {
                            StoreData::Ready(v) => *v,
                            StoreData::Waiting(_) => continue,
                        };
                        
                        self.memory[addr as usize] = store_val;
                        inst.time.write_back = Some(self.current_cycle);

                        sb.remaining_cycles = None;
                        to_release.push(sb.name.clone());
                    }
                }
            }
        }

        // broadcast to CDB
        for (station_name, val, fp_idx) in broadcast_list {
            if self.f_register_status.get(fp_idx) == Some(&station_name) {
                self.registers.fp[fp_idx] = val;
                self.f_register_status.clear(fp_idx);
            }
    
            for st in self.add_stations.iter_mut().chain(self.mul_stations.iter_mut()) {
                if st.qj.as_deref() == Some(&station_name) {
                    st.qj = None;
                    st.vj = Some(val);
                }
                if st.qk.as_deref() == Some(&station_name) {
                    st.qk = None;
                    st.vk = Some(val);
                }
            }
    
            for sb in self.store_buffers.iter_mut() {
                if let Some(StoreData::Waiting(q)) = &sb.data {
                    if q == &station_name {
                        sb.data = Some(StoreData::Ready(val));
                    }
                }
            }
        }

        for name in to_release {
            self.release_station(&name);
        }
    }

    pub fn all_instructions_done(&self) -> bool {
        self.instructions
            .iter()
            .all(|inst| inst.time.write_back.is_some())
    }

    pub fn release_station(&mut self, station_name: &str) {
        for st in self.add_stations.iter_mut() {
            if st.name == station_name {
                st.clear();
                return;
            }
        }
        for st in self.mul_stations.iter_mut() {
            if st.name == station_name {
                st.clear();
                return;
            }
        }
        for lb in self.load_buffers.iter_mut() {
            if lb.name == station_name {
                lb.clear();
                return;
            }
        }
        for sb in self.store_buffers.iter_mut() {
            if sb.name == station_name {
                sb.clear();
                return;
            }
        }
    }

    pub fn dump_state(&self, path: &str) {
        use std::io::Write;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Failed to open output file");

        writeln!(file, "==================================================== Cycle {} ====================================================", self.current_cycle.value()).unwrap();

        writeln!(file, "\n-------------------------------------------- Reservation Stations --------------------------------------------").unwrap();
        for rs in &self.add_stations {
            writeln!(file, "{}", rs).unwrap();
        }
        for rs in &self.mul_stations {
            writeln!(file, "{}", rs).unwrap();
        }

        writeln!(file, "\n------------------------------------- Load Buffers -------------------------------------").unwrap();
        for lb in &self.load_buffers {
            writeln!(file, "{}", lb).unwrap();
        }

        writeln!(file, "\n-------------------------------------- Store Buffers --------------------------------------").unwrap();
        for sb in &self.store_buffers {
            writeln!(file, "{}", sb).unwrap();
        }

        writeln!(file, "\n--------- Register Result Status ---------").unwrap();
        writeln!(file, "{}", self.f_register_status).unwrap();

        writeln!(file, "------------------------------------ Registers -------------------------------------\n").unwrap();
        writeln!(file, "{}", self.registers).unwrap();

        writeln!(file, "\n\n\n\n\n\n\n\n").unwrap();
    }

    pub fn run(&mut self, output_path: &str) {
        let max_cycles = 2000;
    
        while !self.all_instructions_done() {
            self.current_cycle = self.current_cycle.next();
        
            // Safety check to avoid infinite loops
            if self.current_cycle.value() > max_cycles {
                println!("Reached maximum cycle count of {}. Possible deadlock.", max_cycles);
                // Debug info: find blocked instructions
                for (i, inst) in self.instructions.iter().enumerate() {
                    if inst.time.write_back.is_none() {
                        println!("Instruction {} is blocked: {:?}", i, inst);
                    }
                }
                break;
            }

            self.issue_stage();
            self.execute_stage();
            self.writeback_stage();

            self.dump_state(output_path);
        }
    }
}
