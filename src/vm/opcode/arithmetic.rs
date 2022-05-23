use super::Control;
use crate::vm::Vm;
use primitive_types::{H256, U256};

// 0x01
pub fn add(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let (res, _) = a.overflowing_add(b);
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x02
pub fn mul(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let (res, _) = a.overflowing_mul(b);
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x03
pub fn sub(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let (res, _) = a.overflowing_sub(b);
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x04
pub fn div(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let res = a.checked_div(b).unwrap_or(U256::zero());
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x05
pub fn sdiv(vm: &mut Vm) -> Control {
    div(vm)
}

// 0x06 (`mod` is reserved keyword)
pub fn rem(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let res = a.checked_rem(b).unwrap_or(U256::zero());
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x07
pub fn srem(vm: &mut Vm) -> Control {
    rem(vm)
}

// 0x08
pub fn addmod(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b, c);
    let res = if c == U256::zero() {
        U256::zero()
    } else {
        let (x, _) = a.overflowing_add(b);
        x.checked_rem(c).unwrap()
    };

    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x09
pub fn mulmod(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b, c);
    let res = if c == U256::zero() {
        U256::zero()
    } else {
        let (x, _) = a.overflowing_mul(b);
        x.checked_rem(c).unwrap()
    };

    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x0a
pub fn exp(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let (res, _) = a.overflowing_pow(b);
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x0b
// pub fn signextend(vm: &mut Vm) -> Control {
//     pop_u256!(vm, a, b);
//     Control::Continue(1)
// }
