use super::Control;
use crate::vm::{Vm, VmError};
use primitive_types::{H256, U256};

// 0x00
pub fn stop(_vm: &mut Vm) -> Control {
    Control::Stop
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
    pop_usize!(vm, dest, a);
    if a == 0 {
        Control::Continue(1)
    } else {
        Control::Jump(dest)
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

// 0xfe
pub fn invalid(_vm: &mut Vm) -> Control {
    Control::Error(VmError::UnknownOpcode)
}
