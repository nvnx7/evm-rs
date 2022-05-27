use super::Control;
use crate::vm::{Vm, VmError};
use primitive_types::{H256, U256};

// 0x00
pub fn stop(_vm: &mut Vm) -> Control {
    Control::Stop
}

// 0x38
pub fn codesize(vm: &mut Vm) -> Control {
    let size = vm.code.len();
    push_u256!(vm, U256::from(size));
    Control::Continue(1)
}

// 0x39
pub fn codecopy(vm: &mut Vm) -> Control {
    pop_usize!(vm, mem_offset, code_offset, code_size);
    let code_slice = vm.code[code_offset..(code_offset + code_size)].to_vec();
    vm.memory.write(mem_offset, &code_slice);
    Control::Continue(1)
}

// 0x56
pub fn jump(vm: &mut Vm) -> Control {
    pop_usize!(vm, dest);
    if vm.is_valid_jump(dest) {
        Control::Jump(dest)
    } else {
        Control::Error(VmError::InvalidJump)
    }
}

// 0x57
pub fn jumpi(vm: &mut Vm) -> Control {
    pop_usize!(vm, dest);
    pop_u256!(vm, a);
    if a != U256::zero() {
        if vm.is_valid_jump(dest) {
            Control::Jump(dest)
        } else {
            Control::Error(VmError::InvalidJump)
        }
    } else {
        Control::Continue(1)
    }
}

// 0x58
pub fn pc(vm: &mut Vm) -> Control {
    push_u256!(vm, U256::from(vm.pc));
    Control::Continue(1)
}

// 0x5b
pub fn jumpdest(_vm: &mut Vm) -> Control {
    Control::Continue(1)
}

// 0xf3
pub fn return_(vm: &mut Vm) -> Control {
    Control::Stop
}

// 0xfd
pub fn revert(vm: &mut Vm) -> Control {
    Control::Revert
}

// 0xfe
pub fn invalid(_vm: &mut Vm) -> Control {
    Control::Error(VmError::InvalidOpcode)
}
