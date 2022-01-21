use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage:
        \t{} [u|p] iso
        
        p - Apply ESR patch to ISO file.
        u - Remove ESR patch to ISO file.
        iso - A path to an ISO file.", args[0]);
        exit(1);
    }
    esrtool::patch(&args[2]);
}
