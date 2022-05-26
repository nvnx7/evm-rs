#[derive(Debug, PartialEq)]
pub enum VmError {
    // stack errors
    StackOverflow,
    StackUnderflow,
    // runtime error
    UnsuccessfulRun,
    InvalidOpcode,
    UnsupportedOperation,
    InvalidJump,
}
