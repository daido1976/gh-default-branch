mod cmd;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args)
}

fn run(args: Vec<String>) {
    if let Some(first_arg) = args.get(1) {
        if first_arg == "show" {
            cmd::show();
        } else if first_arg == "rename" {
            cmd::rename();
        } else {
            cmd::help();
        }
    } else {
        // when passed no args
        cmd::help();
    }
}
