#[macro_use]
pub mod error;
pub mod memory;
pub mod opcode;
pub mod stack;

use error::VmError;
use opcode::{Control, Opcode};
use std::fmt;

pub struct Vm {
    pub stack: stack::Stack,
    pub memory: memory::Memory,
    pub pc: usize, // program counter
    pub code: Vec<u8>,
}

#[derive(Debug)]
pub enum ExitReason {
    Nil,
    Stop,
    Revert,
    Error(VmError),
}

impl Vm {
    pub fn new(code: &[u8]) -> Self {
        Vm {
            stack: stack::Stack::default(),
            memory: memory::Memory::new(),
            pc: 0,
            code: code.to_vec(),
        }
    }

    pub fn run(&mut self) {
        loop {
            info!("\n{:?}", self);
            match self.step() {
                Ok(reason) => match reason {
                    ExitReason::Stop => {
                        info!("STOP");
                        return;
                    }
                    _ => (),
                },
                Err(e) => {
                    println!("Err: {:?}", e);
                    break;
                }
            }
        }
    }

    pub fn step(&mut self) -> Result<ExitReason, ExitReason> {
        let opcode = if let Some(op) = self.code.get(self.pc).and_then(|&code| Opcode::get(code)) {
            op
        } else {
            return Err(ExitReason::Error(VmError::UnknownOpcode));
        };

        let func = opcode.exec;

        match func(self) {
            Control::Continue(n) => {
                self.pc += n;
                Ok(ExitReason::Nil)
            }
            Control::Stop => Ok(ExitReason::Stop),
            Control::Revert => Err(ExitReason::Revert),
            Control::Error(e) => Err(ExitReason::Error(e)),
        }
    }
}

impl fmt::Debug for Vm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_name = if let Some(op) = self.code.get(self.pc).and_then(|&code| Opcode::get(code)) {
            op.mnemonic
        } else {
            Opcode::INVALID.mnemonic
        };
        write!(
            f,
            "pc: {:?} -> opcode: {}\nstack:\n{}\nmemory: {:?}",
            self.pc, op_name, self.stack, self.memory
        )
    }
}

#[cfg(test)]
mod test {
    use super::Vm;

    #[test]
    fn run() {
        let code = vec![0x60, 0x05, 0x00];
        let mut vm = Vm::new(&code);
        assert_eq!(vm.code.len(), 3);
        assert_eq!(vm.pc, 0);
        vm.step().ok();
        // assert_eq!(vm.pc, 1);
        // vm.run();
    }
}
