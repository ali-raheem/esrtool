use std::{env, panic};
use std::process::exit;

fn print_usage(name: &str) {
    println!(
        "Usage:
    ./{} [u|p] FILE
    
    p - Apply ESR patch to ISO file.
    u - Remove ESR patch from ISO file.
    FILE - A path to an ISO or BIN file.",
        name
    );
    exit(1);
}

fn main() {
    panic::set_hook(Box::new(|e| {
        println!("Error: {}", e);
    }));
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage(&args[0]);
    }
    let mut game = esrtool::Iso::new(&args[2]).unwrap();
    println!("{}", game);
    match &args[1][..] {
        "p" => {
            println!("Attempting to apply patch...");
            game.patch().expect("Couldn't apply patch");
            }
        "u" => {
            println!("Attempting to remove patch...");
            game.unpatch().expect("Couldn't unpatch");
        }
        _ => print_usage(&args[0]),
    }
    game.write().expect("Couldn't write out file");
    println!("{}", game);
}

