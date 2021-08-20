use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    binary: PathBuf,
    args: Vec<String>,
}

fn main() {
    env_logger::init();
    let args = Args::from_args();
    let binary_blob = fs::read(&args.binary).expect("Failed to read the binary");
    let binary = libdamit::Binary::new(binary_blob);
    eprintln!("Run {:?} with args {:?}", args.binary, args.args);
}
