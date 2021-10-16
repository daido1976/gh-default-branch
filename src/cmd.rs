use std::process::Command;

pub fn show() {
    let default_branch = get_default_branch();
    println!("{}", default_branch);
}

pub fn rename(to_branch: &str) {
    let from_branch = get_default_branch();
    println!(
        "=== START: Rename from {} to {} ===",
        from_branch, to_branch
    );
    fetch_origin(&from_branch);
    println!(
        "=== FINISH: Rename from {} to {} ===",
        from_branch, to_branch
    );
}

pub fn help() {
    println!("TODO: show help message");
}

fn get_default_branch() -> String {
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
    output.to_string()
}

fn fetch_origin(branch: &str) {
    println!("$ git fetch origin {}", branch);
    let output = Command::new("git")
        .args(["fetch", "origin", branch])
        .output()
        .expect("Failed to execute command");
    // git outputs the fetch and push logs to stderr...
    // See. https://stackoverflow.com/questions/57016157/stop-git-from-writing-non-errors-to-stderr
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);
}
