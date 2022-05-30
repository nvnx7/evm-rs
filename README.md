# rust-evm

EVM (Ethereum Virtual Machine) implementation in Rust language acc. to the specs.

### Example
```rust
// Iterate and sum up to 5

let code =  vec![
            0x60, 0x00, // PUSH1 (initial value 0)
            0x5b, // JUMPDEST    (loop start)
            0x60, 0x01, // PUSH1
            0x01, // ADD         (increment)
            0x80, // DUP1
            0x60, 0x05, // PUSH1
            0x11, // GT          (check sum < 5)
            0x60, 0x02, // PUSH1
            0x57, // JUMPI       (iterate if sum < 5)
            0x60, 0x00, // PUSH1
            0x52, // MSTORE      (store sum in memory)
            0x60, 0x20, // PUSH1
            0x60, 0x00, // PUSH1
            0xf3, // RETURN      (return sum)
        ];

let mut vm = Vm::new(&code);
vm.run().ok();

let sum = vm.get_return_data(); // 0x000..005
```