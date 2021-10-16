mod cmd;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args)
}

fn run(args: Vec<String>) {
    match args.get(1) {
        Some(first_arg) if first_arg == "show" => cmd::show(),
        Some(first_arg) if first_arg == "rename" => match args.get(2) {
            Some(to_name) => cmd::rename(to_name),
            None => {
                println!("USAGE: 'gh default-branch rename <name>' Please specify the name.")
            }
        },
        // when passed no args, or the wrong args.
        _ => cmd::help(),
    }
}
