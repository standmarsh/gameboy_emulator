use crate::{
    cpu::{
        InstructionReturn,
        ReturnType::{Jumped, NotJumped},
        CPU,
    },
    utils::{le_bytes_to_word, word_to_bytes},
};

use super::opcodes::JumpCondition;

macro_rules! to_jump {
    ($cpu: ident, $jump_condition: ident) => {
        match $jump_condition {
            JumpCondition::NIL => true, // always take this action
            JumpCondition::Z => $cpu.registers.f.zero,
            JumpCondition::C => $cpu.registers.f.carry,
            JumpCondition::NZ => !$cpu.registers.f.zero,
            JumpCondition::NC => !$cpu.registers.f.carry,
        }
    };
}

pub fn call(cpu: &mut CPU, flag: JumpCondition) -> InstructionReturn {
    let to_jump = to_jump!(cpu, flag);

    let lo = cpu.bus.read(cpu.registers.pc + 1);
    let hi = cpu.bus.read(cpu.registers.pc + 2);
    let addr = le_bytes_to_word(lo, hi);

    let (return_type, n_cycles) = match to_jump {
        true => {
            cpu.registers.sp -= 1;
            let (pc_high, pc_low) = word_to_bytes(cpu.registers.pc + 3);
            cpu.bus.write(cpu.registers.sp, pc_high);
            cpu.registers.sp -= 1;
            cpu.bus.write(cpu.registers.sp, pc_low);
            cpu.registers.pc = addr;

            (Jumped, 24)
        }
        false => (NotJumped, 12),
    };

    InstructionReturn {
        n_cycles,
        n_bytes: 3,
        return_type,
    }
}

pub fn jp(cpu: &mut CPU, flag: JumpCondition) -> InstructionReturn {
    let to_jump = to_jump!(cpu, flag);

    let (return_type, n_cycles) = match to_jump {
        true => {
            let lo = cpu.bus.read(cpu.registers.pc + 1);
            let hi = cpu.bus.read(cpu.registers.pc + 2);
            let addr = le_bytes_to_word(lo, hi);
            cpu.registers.pc = addr;

            (Jumped, 16)
        }
        false => (NotJumped, 12),
    };

    InstructionReturn {
        n_cycles,
        n_bytes: 3,
        return_type,
    }
}
