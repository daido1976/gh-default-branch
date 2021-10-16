mod cmd;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args)
}

fn run(args: Vec<String>) {
    match args.get(1) {
        Some(first) if first == "show" => cmd::show(),
        Some(first) if first == "rename" => {
            if let Some(to_name) = args.get(2) {
                cmd::rename(to_name)
            } else {
                println!("USAGE: 'gh default-branch rename <name>' Please specify the name.")
            }
        }
        // when passed no args, or the wrong args.
        _ => cmd::help(),
    }
}
