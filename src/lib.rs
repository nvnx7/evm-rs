#[macro_use]
extern crate log;

mod vm;

#[allow(unused_imports)]
#[allow(dead_code)]
fn add_one(num: i32) -> i32 {
    info!("add_one called with {}", num);
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_one() {
        // env_logger::init();

        info!("can log from the test too");
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn it_works() {
        env_logger::init();

        let code = vec![0x60, 0x02, 0x60, 0x03, 0x03, 0x00];
        let mut vm = vm::Vm::new(&code);
        vm.run();
    }
}
