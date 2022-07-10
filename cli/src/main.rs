use clap::Parser;
use hex::FromHex;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use vm::Vm;

#[derive(Parser, Debug)]
#[clap(name = "evm-rs", author, version)]
struct Args {
    #[clap(short, long, value_parser)]
    bytecode: String,
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set default subscriber");

    let args = Args::parse();
    let bytecode = <Vec<u8>>::from_hex(args.bytecode).unwrap();
    let mut vm = Vm::new(&bytecode);
    vm.run().ok();
}
