use super::Control;
use crate::vm::opcode::i256::I256;
use crate::vm::{Vm, VmError};
use core::ops::{BitAnd, BitOr, BitXor};
use primitive_types::{H256, U256};

// 0x10
pub fn lt(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let res = if a.lt(&b) { U256::one() } else { U256::zero() };
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x11
pub fn gt(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let res = if a.gt(&b) { U256::one() } else { U256::zero() };
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x12
pub fn slt(vm: &mut Vm) -> Control {
    pop_i256!(vm, a, b);
    let res = if a.lt(&b) { U256::one() } else { U256::zero() };
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x13
pub fn sgt(vm: &mut Vm) -> Control {
    pop_i256!(vm, a, b);
    let res = if a.gt(&b) { U256::one() } else { U256::zero() };
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x14
pub fn eq(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let res = if a.eq(&b) { U256::one() } else { U256::zero() };
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x15
pub fn iszero(vm: &mut Vm) -> Control {
    pop_u256!(vm, a);
    let res = if a.is_zero() {
        U256::one()
    } else {
        U256::zero()
    };
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x16
pub fn and(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let res = a.bitand(b);
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x17
pub fn or(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let res = a.bitor(b);
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x18
pub fn xor(vm: &mut Vm) -> Control {
    pop_u256!(vm, a, b);
    let res = a.bitxor(b);
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x19
pub fn not(vm: &mut Vm) -> Control {
    pop_u256!(vm, a);
    let res = !a;
    push_u256!(vm, res);
    Control::Continue(1)
}

// 0x1a
pub fn byte(vm: &mut Vm) -> Control {
    pop_usize!(vm, offset);
    pop_u256!(vm, value);

    if offset > 32 {
        push_u256!(vm, U256::zero());
    } else {
        // U256 is little endian
        let byte = value.byte(32 - offset - 1);
        push_u256!(vm, U256::from(byte));
    }
    Control::Continue(1)
}

// 0x1b
pub fn shl(vm: &mut Vm) -> Control {
    pop_u256!(vm, value, shift);
    if shift >= U256::from(256) || value == U256::zero() {
        push_u256!(vm, U256::zero());
    } else {
        push_u256!(vm, value << shift);
    }
    Control::Continue(1)
}

// 0x1c
pub fn shr(vm: &mut Vm) -> Control {
    pop_u256!(vm, value, shift);
    if shift >= U256::from(256) || value == U256::zero() {
        push_u256!(vm, U256::zero());
    } else {
        push_u256!(vm, value >> shift);
    }
    Control::Continue(1)
}
