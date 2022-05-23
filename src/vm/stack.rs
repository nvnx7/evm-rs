use crate::vm::error::VmError;
use primitive_types::H256;
use std::{default, fmt};

pub const EVM_STACK_MAX_SIZE: usize = 1024;

#[derive(Clone)]
pub struct Stack {
    data: Vec<H256>,
    pub max_size: usize,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Self {
            data: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, value: H256) -> Result<(), VmError> {
        match self.data.len() {
            x if x < self.max_size => Ok(self.data.push(value)),
            _ => Err(VmError::StackOverflow),
        }
    }

    pub fn pop(&mut self) -> Result<H256, VmError> {
        match self.data.pop() {
            Some(x) => Ok(x),
            None => Err(VmError::StackUnderflow),
        }
    }

    pub fn peek(&mut self, n: usize) -> Result<H256, VmError> {
        match self.data.get(self.data.len() - n - 1) {
            Some(x) => Ok(*x),
            None => Err(VmError::StackOverflow),
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) -> Result<(), VmError> {
        let len = self.data.len();

        if len == 0 || a >= len || b >= len {
            return Err(VmError::StackOverflow);
        }

        self.data.swap(len - a - 1, len - b - 1);
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl default::Default for Stack {
    fn default() -> Self {
        Self::new(EVM_STACK_MAX_SIZE)
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .rev()
                .fold(String::new(), |acc, &x| format!("{}{}\n", acc, x))
        )
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self.data
                .iter()
                .rev()
                .fold(String::new(), |acc, &x| format!("{}{}\n", acc, x))
        )
    }
}

#[cfg(test)]
mod test {
    use super::Stack;
    use super::VmError;
    use super::H256;

    macro_rules! h256_from_slice {
        ($x:expr) => {{
            let mut v: Vec<u8> = vec![0; 32 - $x.len()];
            v.extend_from_slice($x);
            H256::from_slice(&v[..])
        }};
    }

    #[test]
    fn test_stack() {
        let max_sz: usize = 1024;
        let mut data = Stack::new(max_sz);
        assert_eq!(data.max_size, max_sz);

        data.push(h256_from_slice!(&[1])).ok();
        data.push(h256_from_slice!(&[2])).ok();
        data.push(h256_from_slice!(&[3])).ok();
        assert_eq!(data.size(), 3);

        assert_eq!(data.pop(), Ok(h256_from_slice!(&[3])));
        assert_eq!(data.pop(), Ok(h256_from_slice!(&[2])));
        assert_eq!(data.pop(), Ok(h256_from_slice!(&[1])));
        assert_eq!(data.size(), 0);
        assert_eq!(data.pop(), Err(VmError::StackUnderflow));

        // fill the data
        for _ in 0..max_sz {
            data.push(h256_from_slice!(&[5])).ok();
        }

        assert_eq!(
            data.push(h256_from_slice!(&[5])),
            Err(VmError::StackOverflow)
        )
    }
}
