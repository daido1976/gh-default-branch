use std::process::Command;

pub fn show() {
    let output = Command::new("git")
        .args(["remote", "show", "origin"])
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let output = output
        .lines()
        .find(|line| line.contains("HEAD branch"))
        .expect("Failed to find HEAD branch");
    let output = output
        .split_whitespace()
        .last()
        .expect("Failed to fetch last element");

    println!("{}", output);
}

pub fn rename(_name: &str) {
    println!("unimplemented rename command!");
}

pub fn help() {
    println!("TODO: show help message");
}