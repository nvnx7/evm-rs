#[macro_use]
mod macros;
mod arithmetic;
mod i256;
mod logic;
mod misc;
mod store;

use crate::vm::{error::VmError, Vm};
use arithmetic::*;
use logic::*;
use misc::*;
use store::*;

type OpcodeFunction = fn(&mut Vm) -> Control;

#[derive(Debug, PartialEq)]
pub enum Control {
    Continue(usize),
    Jump(usize),
    Stop,
    Return,
    Revert,
    Error(VmError),
}

macro_rules! make_opcode {
    ($code: expr, $name: ident, $fn: ident) => {
        #[allow(dead_code)]
        pub const $name: Opcode = Opcode {
            code: $code,
            mnemonic: stringify!($name),
            exec: $fn,
        };
    };
}

pub struct Opcode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub exec: OpcodeFunction,
}

impl Opcode {
    make_opcode!(0x00, STOP, stop);
    make_opcode!(0x01, ADD, add);
    make_opcode!(0x02, MUL, mul);
    make_opcode!(0x03, SUB, sub);
    make_opcode!(0x04, DIV, div);
    make_opcode!(0x05, SDIV, sdiv);
    make_opcode!(0x06, MOD, rem);
    make_opcode!(0x07, SMOD, srem);
    make_opcode!(0x08, ADDMOD, addmod);
    make_opcode!(0x09, MULMOD, mulmod);
    make_opcode!(0x0a, EXP, exp);
    // make_opcode!(0x0b, SIGNEXTEND, signextend);

    make_opcode!(0x10, LT, lt);
    make_opcode!(0x11, GT, gt);
    make_opcode!(0x12, SLT, slt);
    make_opcode!(0x13, SGT, sgt);
    make_opcode!(0x14, EQ, eq);
    make_opcode!(0x15, ISZERO, iszero);
    make_opcode!(0x16, AND, and);
    make_opcode!(0x17, OR, or);
    make_opcode!(0x18, XOR, xor);
    make_opcode!(0x19, NOT, not);
    // make_opcode!(0x1a, BYTE, byte);

    make_opcode!(0x50, POP, pop);
    make_opcode!(0x51, MLOAD, mload);
    make_opcode!(0x52, MSTORE, mstore);
    make_opcode!(0x53, MSTORE8, mstore8);
    make_opcode!(0x56, JUMP, jump);
    make_opcode!(0x57, JUMPI, jumpi);
    make_opcode!(0x58, PC, pc);
    make_opcode!(0x59, MSIZE, msize);
    make_opcode!(0x5b, JUMPDEST, jumpdest);

    // PUSH1 - PUSH32
    make_opcode!(0x60, PUSH1, push1);
    make_opcode!(0x61, PUSH2, push2);
    make_opcode!(0x62, PUSH3, push3);
    make_opcode!(0x63, PUSH4, push4);
    make_opcode!(0x64, PUSH5, push5);
    make_opcode!(0x65, PUSH6, push6);
    make_opcode!(0x66, PUSH7, push7);
    make_opcode!(0x67, PUSH8, push8);
    make_opcode!(0x68, PUSH9, push9);
    make_opcode!(0x69, PUSH10, push10);
    make_opcode!(0x6a, PUSH11, push11);
    make_opcode!(0x6b, PUSH12, push12);
    make_opcode!(0x6c, PUSH13, push13);
    make_opcode!(0x6d, PUSH14, push14);
    make_opcode!(0x6e, PUSH15, push15);
    make_opcode!(0x6f, PUSH16, push16);
    make_opcode!(0x70, PUSH17, push17);
    make_opcode!(0x71, PUSH18, push18);
    make_opcode!(0x72, PUSH19, push19);
    make_opcode!(0x73, PUSH20, push20);
    make_opcode!(0x74, PUSH21, push21);
    make_opcode!(0x75, PUSH22, push22);
    make_opcode!(0x76, PUSH23, push23);
    make_opcode!(0x77, PUSH24, push24);
    make_opcode!(0x78, PUSH25, push25);
    make_opcode!(0x79, PUSH26, push26);
    make_opcode!(0x7a, PUSH27, push27);
    make_opcode!(0x7b, PUSH28, push28);
    make_opcode!(0x7c, PUSH29, push29);
    make_opcode!(0x7d, PUSH30, push30);
    make_opcode!(0x7e, PUSH31, push31);
    make_opcode!(0x7f, PUSH32, push32);

    // DUP1 - DUP16
    make_opcode!(0x80, DUP1, dup1);
    make_opcode!(0x81, DUP2, dup2);
    make_opcode!(0x82, DUP3, dup3);
    make_opcode!(0x83, DUP4, dup4);
    make_opcode!(0x84, DUP5, dup5);
    make_opcode!(0x85, DUP6, dup6);
    make_opcode!(0x86, DUP7, dup7);
    make_opcode!(0x87, DUP8, dup8);
    make_opcode!(0x88, DUP9, dup9);
    make_opcode!(0x89, DUP10, dup10);
    make_opcode!(0x8a, DUP11, dup11);
    make_opcode!(0x8b, DUP12, dup12);
    make_opcode!(0x8c, DUP13, dup13);
    make_opcode!(0x8d, DUP14, dup14);
    make_opcode!(0x8e, DUP15, dup15);
    make_opcode!(0x8f, DUP16, dup16);

    // SWAP1 - SWAP16
    make_opcode!(0x90, SWAP1, swap1);
    make_opcode!(0x91, SWAP2, swap2);
    make_opcode!(0x92, SWAP3, swap3);
    make_opcode!(0x93, SWAP4, swap4);
    make_opcode!(0x94, SWAP5, swap5);
    make_opcode!(0x95, SWAP6, swap6);
    make_opcode!(0x96, SWAP7, swap7);
    make_opcode!(0x97, SWAP8, swap8);
    make_opcode!(0x98, SWAP9, swap9);
    make_opcode!(0x99, SWAP10, swap10);
    make_opcode!(0x9a, SWAP11, swap11);
    make_opcode!(0x9b, SWAP12, swap12);
    make_opcode!(0x9c, SWAP13, swap13);
    make_opcode!(0x9d, SWAP14, swap14);
    make_opcode!(0x9e, SWAP15, swap15);
    make_opcode!(0x9f, SWAP16, swap16);

    make_opcode!(0xf3, RETURN, return_);
    make_opcode!(0xfd, REVERT, revert);
    make_opcode!(0xfe, INVALID, invalid);

    pub fn get(code: u8) -> Option<&'static Self> {
        OPCODE_LIST.get(code as usize)
    }
}

#[allow(dead_code)]
pub const OPCODE_LIST: [Opcode; 256] = {
    let mut opcodes: [Opcode; 256] = [Opcode::INVALID; 256];

    opcodes[Opcode::STOP.code as usize] = Opcode::STOP;
    opcodes[Opcode::ADD.code as usize] = Opcode::ADD;
    opcodes[Opcode::MUL.code as usize] = Opcode::MUL;
    opcodes[Opcode::SUB.code as usize] = Opcode::SUB;
    opcodes[Opcode::DIV.code as usize] = Opcode::DIV;
    opcodes[Opcode::SDIV.code as usize] = Opcode::SDIV;
    opcodes[Opcode::MOD.code as usize] = Opcode::MOD;
    opcodes[Opcode::SMOD.code as usize] = Opcode::SMOD;
    opcodes[Opcode::ADDMOD.code as usize] = Opcode::ADDMOD;
    opcodes[Opcode::MULMOD.code as usize] = Opcode::MULMOD;
    opcodes[Opcode::EXP.code as usize] = Opcode::EXP;
    // opcodes[Opcode::SIGNEXTEND.code as usize] = Opcode::SIGNEXTEND;

    opcodes[Opcode::LT.code as usize] = Opcode::LT;
    opcodes[Opcode::GT.code as usize] = Opcode::GT;
    opcodes[Opcode::SLT.code as usize] = Opcode::SLT;
    opcodes[Opcode::SGT.code as usize] = Opcode::SGT;
    opcodes[Opcode::EQ.code as usize] = Opcode::EQ;
    opcodes[Opcode::ISZERO.code as usize] = Opcode::ISZERO;
    opcodes[Opcode::AND.code as usize] = Opcode::AND;
    opcodes[Opcode::OR.code as usize] = Opcode::OR;
    opcodes[Opcode::XOR.code as usize] = Opcode::XOR;
    opcodes[Opcode::NOT.code as usize] = Opcode::NOT;
    // opcodes[Opcode::SHL.code as usize] = Opcode::SHL;
    // opcodes[Opcode::SHR.code as usize] = Opcode::SHR;
    // opcodes[Opcode::SAR.code as usize] = Opcode::SAR;
    // opcodes[Opcode::BYTE.code as usize] = Opcode::BYTE;
    opcodes[Opcode::POP.code as usize] = Opcode::POP;
    opcodes[Opcode::MLOAD.code as usize] = Opcode::MLOAD;
    opcodes[Opcode::MSTORE.code as usize] = Opcode::MSTORE;
    opcodes[Opcode::MSTORE8.code as usize] = Opcode::MSTORE8;
    opcodes[Opcode::JUMP.code as usize] = Opcode::JUMP;
    opcodes[Opcode::JUMPI.code as usize] = Opcode::JUMPI;
    opcodes[Opcode::PC.code as usize] = Opcode::PC;
    opcodes[Opcode::MSIZE.code as usize] = Opcode::MSIZE;
    opcodes[Opcode::JUMPDEST.code as usize] = Opcode::JUMPDEST;

    opcodes[Opcode::PUSH1.code as usize] = Opcode::PUSH1;
    opcodes[Opcode::PUSH2.code as usize] = Opcode::PUSH2;
    opcodes[Opcode::PUSH3.code as usize] = Opcode::PUSH3;
    opcodes[Opcode::PUSH4.code as usize] = Opcode::PUSH4;
    opcodes[Opcode::PUSH5.code as usize] = Opcode::PUSH5;
    opcodes[Opcode::PUSH6.code as usize] = Opcode::PUSH6;
    opcodes[Opcode::PUSH7.code as usize] = Opcode::PUSH7;
    opcodes[Opcode::PUSH8.code as usize] = Opcode::PUSH8;
    opcodes[Opcode::PUSH9.code as usize] = Opcode::PUSH9;
    opcodes[Opcode::PUSH10.code as usize] = Opcode::PUSH10;
    opcodes[Opcode::PUSH11.code as usize] = Opcode::PUSH11;
    opcodes[Opcode::PUSH12.code as usize] = Opcode::PUSH12;
    opcodes[Opcode::PUSH13.code as usize] = Opcode::PUSH13;
    opcodes[Opcode::PUSH14.code as usize] = Opcode::PUSH14;
    opcodes[Opcode::PUSH15.code as usize] = Opcode::PUSH15;
    opcodes[Opcode::PUSH16.code as usize] = Opcode::PUSH16;
    opcodes[Opcode::PUSH17.code as usize] = Opcode::PUSH17;
    opcodes[Opcode::PUSH18.code as usize] = Opcode::PUSH18;
    opcodes[Opcode::PUSH19.code as usize] = Opcode::PUSH19;
    opcodes[Opcode::PUSH20.code as usize] = Opcode::PUSH20;
    opcodes[Opcode::PUSH21.code as usize] = Opcode::PUSH21;
    opcodes[Opcode::PUSH22.code as usize] = Opcode::PUSH22;
    opcodes[Opcode::PUSH23.code as usize] = Opcode::PUSH23;
    opcodes[Opcode::PUSH24.code as usize] = Opcode::PUSH24;
    opcodes[Opcode::PUSH25.code as usize] = Opcode::PUSH25;
    opcodes[Opcode::PUSH26.code as usize] = Opcode::PUSH26;
    opcodes[Opcode::PUSH27.code as usize] = Opcode::PUSH27;
    opcodes[Opcode::PUSH28.code as usize] = Opcode::PUSH28;
    opcodes[Opcode::PUSH29.code as usize] = Opcode::PUSH29;
    opcodes[Opcode::PUSH30.code as usize] = Opcode::PUSH30;
    opcodes[Opcode::PUSH31.code as usize] = Opcode::PUSH31;
    opcodes[Opcode::PUSH32.code as usize] = Opcode::PUSH32;

    opcodes[Opcode::DUP1.code as usize] = Opcode::DUP1;
    opcodes[Opcode::DUP2.code as usize] = Opcode::DUP2;
    opcodes[Opcode::DUP3.code as usize] = Opcode::DUP3;
    opcodes[Opcode::DUP4.code as usize] = Opcode::DUP4;
    opcodes[Opcode::DUP5.code as usize] = Opcode::DUP5;
    opcodes[Opcode::DUP6.code as usize] = Opcode::DUP6;
    opcodes[Opcode::DUP7.code as usize] = Opcode::DUP7;
    opcodes[Opcode::DUP8.code as usize] = Opcode::DUP8;
    opcodes[Opcode::DUP9.code as usize] = Opcode::DUP9;
    opcodes[Opcode::DUP10.code as usize] = Opcode::DUP10;
    opcodes[Opcode::DUP11.code as usize] = Opcode::DUP11;
    opcodes[Opcode::DUP12.code as usize] = Opcode::DUP12;
    opcodes[Opcode::DUP13.code as usize] = Opcode::DUP13;
    opcodes[Opcode::DUP14.code as usize] = Opcode::DUP14;
    opcodes[Opcode::DUP15.code as usize] = Opcode::DUP15;

    opcodes[Opcode::SWAP1.code as usize] = Opcode::SWAP1;
    opcodes[Opcode::SWAP2.code as usize] = Opcode::SWAP2;
    opcodes[Opcode::SWAP3.code as usize] = Opcode::SWAP3;
    opcodes[Opcode::SWAP4.code as usize] = Opcode::SWAP4;
    opcodes[Opcode::SWAP5.code as usize] = Opcode::SWAP5;
    opcodes[Opcode::SWAP6.code as usize] = Opcode::SWAP6;
    opcodes[Opcode::SWAP7.code as usize] = Opcode::SWAP7;
    opcodes[Opcode::SWAP8.code as usize] = Opcode::SWAP8;
    opcodes[Opcode::SWAP9.code as usize] = Opcode::SWAP9;
    opcodes[Opcode::SWAP10.code as usize] = Opcode::SWAP10;
    opcodes[Opcode::SWAP11.code as usize] = Opcode::SWAP11;
    opcodes[Opcode::SWAP12.code as usize] = Opcode::SWAP12;
    opcodes[Opcode::SWAP13.code as usize] = Opcode::SWAP13;
    opcodes[Opcode::SWAP14.code as usize] = Opcode::SWAP14;
    opcodes[Opcode::SWAP15.code as usize] = Opcode::SWAP15;
    opcodes[Opcode::SWAP16.code as usize] = Opcode::SWAP16;

    opcodes[Opcode::RETURN.code as usize] = Opcode::RETURN;
    opcodes[Opcode::REVERT.code as usize] = Opcode::REVERT;
    opcodes[Opcode::INVALID.code as usize] = Opcode::INVALID;

    opcodes
};
