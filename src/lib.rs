#[macro_use]
extern crate log;

mod vm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        env_logger::init();

        let code = vec![0x60, 0x02, 0x60, 0x03, 0x03, 0x00];
        let mut vm = vm::Vm::new(&code);
        vm.run();
    }
}
