use clap::Parser;
use vm::Vm;

#[derive(Parser, Debug)]
#[clap(name = "evm-rs", author, version)]
struct Args {
    #[clap(short, long, value_parser)]
    bytecode: String,
}

fn main() {
    let args = Args::parse();

    let bytecode = args.bytecode;
    let mut vm = Vm::new(bytecode.as_bytes());
    vm.run().ok();
}
