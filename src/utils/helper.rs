use crate::simulator::instruction::{Instruction, InstructionType};
use crate::simulator::types::Cycle;

// Provides a uniform way for the simulator to query any slot
// in the pipeline for its busy status and instruction index.
pub trait ExecUnitSlot {
    fn is_busy(&self) -> bool;
    fn inst_index(&self) -> Option<usize>;
}

pub fn is_ready_for_writeback<'a>(
    unit_busy: bool,
    maybe_idx: Option<usize>,
    instrs: &'a mut [Instruction],
    cur_cycle: Cycle,
) -> Option<(usize, &'a mut Instruction)> {
    if !unit_busy { return None; }
    let idx = maybe_idx?;
    let inst = instrs.get_mut(idx)?;
    if inst.time.write_back.is_some() { return None; }
    if inst.time.completion.map_or(true, |c| c >= cur_cycle) { return None; }
    Some((idx, inst))
}

pub struct WritebackCandidate {
    pub itype: InstructionType,
    pub unit_idx: usize,
    pub inst_idx: usize,
}

pub fn collect_ready_units<U, F>(
    out: &mut Vec<WritebackCandidate>,
    units: &mut [U],
    instrs: &mut [Instruction],
    current_cycle: Cycle,
    mut extra_ok: F,
) where
    U: ExecUnitSlot,
    F: FnMut(&U) -> bool,
{
    for (idx, u) in units.iter().enumerate() {
        if let Some((inst_idx, inst)) =
            is_ready_for_writeback(u.is_busy(), u.inst_index(), instrs, current_cycle)
        {
            if extra_ok(u) {
                let real_type = inst.meta.inst_type;
                out.push(WritebackCandidate {
                    itype: real_type,
                    unit_idx: idx,
                    inst_idx,
                });
            }
        }
    }
}
