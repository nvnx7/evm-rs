# rust-evm

EVM (Ethereum Virtual Machine) implementation in Rust language acc. to the specs.

### Example
```rust
// Iterate and sum up to 5

let code = vec![
    0x60, 0x00, // PUSH1
    0x5b, // JUMPDEST
    0x60, 0x01, // PUSH1
    0x01, // ADD
    0x80, // DUP1
    0x60, 0x05, // PUSH1
    0x11, // GT
    0x60, 0x02, // PUSH1
    0x57, // JUMPI
    0x00, // STOP
];

let mut vm = vm::Vm::new(&code4);
vm.run();
```