use super::Control;
use crate::vm::{Vm, VmError};
use primitive_types::{H256, U256};

macro_rules! make_push_fn {
    ($name: ident, $n: expr) => {
        #[allow(dead_code)]
        pub fn $name(vm: &mut Vm) -> Control {
            let end = std::cmp::min(vm.pc + $n + 1, vm.code.len());
            let slice = &vm.code[(vm.pc + 1)..end];
            let mut value = [0u8; 32];
            value[(32 - slice.len())..32].copy_from_slice(slice);
            push!(vm, H256(value));
            Control::Continue($n + 1)
        }
    };
}

macro_rules! make_dup_fn {
    ($name: ident, $n: expr) => {
        #[allow(dead_code)]
        pub fn $name(vm: &mut Vm) -> Control {
            peek!(vm, v, $n);
            push!(vm, v);
            Control::Continue(1)
        }
    };
}

macro_rules! make_swap_fn {
    ($name: ident, $n: expr) => {
        #[allow(dead_code)]
        pub fn $name(vm: &mut Vm) -> Control {
            match vm.stack.swap(0, $n) {
                Ok(_) => Control::Continue(1),
                Err(e) => Control::Error(e),
            }
        }
    };
}

// 0x50
pub fn pop(vm: &mut Vm) -> Control {
    match vm.stack.pop() {
        Ok(_) => Control::Continue(1),
        Err(e) => Control::Error(e),
    }
}

// 0x51
pub fn mload(vm: &mut Vm) -> Control {
    pop_usize!(vm, a);
    vm.memory.load(a);
    Control::Continue(1)
}

// 0x52
pub fn mstore(vm: &mut Vm) -> Control {
    pop_usize!(vm, offset);
    pop!(vm, value);
    vm.memory.store(offset, value.as_fixed_bytes());
    Control::Continue(1)
}

// 0x53
pub fn mstore8(vm: &mut Vm) -> Control {
    pop_usize!(vm, offset);
    pop!(vm, value);
    let byte = value.as_fixed_bytes()[31];
    vm.memory.store8(offset, byte);
    Control::Continue(1)
}

// 0x58
pub fn pc(vm: &mut Vm) -> Control {
    push_u256!(vm, U256::from(vm.pc));
    Control::Continue(1)
}

// 0x59
pub fn msize(vm: &mut Vm) -> Control {
    push_u256!(vm, U256::from(vm.memory.size()));
    Control::Continue(1)
}

// 0x60 - 0x7f
make_push_fn!(push1, 1); // PUSH1
make_push_fn!(push2, 2); // PUSH2
make_push_fn!(push3, 3); // PUSH3
make_push_fn!(push4, 4); // PUSH4
make_push_fn!(push5, 5); // PUSH5
make_push_fn!(push6, 6); // PUSH6
make_push_fn!(push7, 7); // PUSH7
make_push_fn!(push8, 8); // PUSH8
make_push_fn!(push9, 9); // PUSH9
make_push_fn!(push10, 10); // PUSH10
make_push_fn!(push11, 11); // PUSH11
make_push_fn!(push12, 12); // PUSH12
make_push_fn!(push13, 13); // PUSH13
make_push_fn!(push14, 14); // PUSH14
make_push_fn!(push15, 15); // PUSH15
make_push_fn!(push16, 16); // PUSH16
make_push_fn!(push17, 17); // PUSH17
make_push_fn!(push18, 18); // PUSH18
make_push_fn!(push19, 19); // PUSH19
make_push_fn!(push20, 20); // PUSH20
make_push_fn!(push21, 21); // PUSH21
make_push_fn!(push22, 22); // PUSH22
make_push_fn!(push23, 23); // PUSH23
make_push_fn!(push24, 24); // PUSH24
make_push_fn!(push25, 25); // PUSH25
make_push_fn!(push26, 26); // PUSH26
make_push_fn!(push27, 27); // PUSH27
make_push_fn!(push28, 28); // PUSH28
make_push_fn!(push29, 29); // PUSH29
make_push_fn!(push30, 30); // PUSH30
make_push_fn!(push31, 31); // PUSH31
make_push_fn!(push32, 32); // PUSH32

// 0x80 - 0x8f
make_dup_fn!(dup1, 1); // DUP1
make_dup_fn!(dup2, 2); // DUP2
make_dup_fn!(dup3, 3); // DUP3
make_dup_fn!(dup4, 4); // DUP4
make_dup_fn!(dup5, 5); // DUP5
make_dup_fn!(dup6, 6); // DUP6
make_dup_fn!(dup7, 7); // DUP7
make_dup_fn!(dup8, 8); // DUP8
make_dup_fn!(dup9, 9); // DUP9
make_dup_fn!(dup10, 10); // DUP10
make_dup_fn!(dup11, 11); // DUP11
make_dup_fn!(dup12, 12); // DUP12
make_dup_fn!(dup13, 13); // DUP13
make_dup_fn!(dup14, 14); // DUP14
make_dup_fn!(dup15, 15); // DUP15
make_dup_fn!(dup16, 16); // DUP16

// 0x90 - 0x9f
make_swap_fn!(swap1, 1); // SWAP1
make_swap_fn!(swap2, 2); // SWAP2
make_swap_fn!(swap3, 3); // SWAP3
make_swap_fn!(swap4, 4); // SWAP4
make_swap_fn!(swap5, 5); // SWAP5
make_swap_fn!(swap6, 6); // SWAP6
make_swap_fn!(swap7, 7); // SWAP7
make_swap_fn!(swap8, 8); // SWAP8
make_swap_fn!(swap9, 9); // SWAP9
make_swap_fn!(swap10, 10); // SWAP10
make_swap_fn!(swap11, 11); // SWAP11
make_swap_fn!(swap12, 12); // SWAP12
make_swap_fn!(swap13, 13); // SWAP13
make_swap_fn!(swap14, 14); // SWAP14
make_swap_fn!(swap15, 15); // SWAP15
make_swap_fn!(swap16, 16); // SWAP16
