use std::env;
use std::process::exit;

fn print_usage(name: &str) {
    println!("Usage:
    \t{} [u|p] iso
    
    p - Apply ESR patch to ISO file.
    u - Remove ESR patch to ISO file.
    iso - A path to an ISO file.", name);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage(&args[0]);
    }
    match &args[1][..] {
        "p" => esrtool::patch(&args[2]),
        "u" => esrtool::unpatch(&args[2]),
        _ => print_usage(&args[0])
    }
    println!("OK!");
}
