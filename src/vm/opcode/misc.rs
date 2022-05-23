use super::Control;
use crate::vm::{Vm, VmError};

pub fn invalid(_vm: &mut Vm) -> Control {
    Control::Error(VmError::UnknownOpcode)
}

pub fn stop(_vm: &mut Vm) -> Control {
    Control::Stop
}
