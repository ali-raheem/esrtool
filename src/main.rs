use std::env;
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
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage(&args[0]);
    }
    let mut game = esrtool::Iso::new(&args[2]);
    println!("{}", game);
    match &args[1][..] {
        "p" => {
            println!("Attempting to apply patch...");
            game.patch()
        }
        "u" => {
            println!("Attempting to remove patch...");
            game.unpatch()
        }
        _ => print_usage(&args[0]),
    }
    println!("{}", game);
    game.write();
    println!("OK!");
}
