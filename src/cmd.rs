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
    push_new_branch(&from_branch, to_branch);
    rename_default_branch(to_branch);
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

/// * `from` - e.g. master
/// * `to` - e.g. main
fn push_new_branch(from: &str, to: &str) {
    // masterブランチからmainブランチを強制的に分岐させる
    println!("$ git checkout -B {} origin/{} --no-track", to, from);
    let output = Command::new("git")
        .args([
            "checkout",
            "-B",
            to,
            &format!("origin/{}", from),
            "--no-track",
        ])
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);

    // mainブランチをリモートにプッシュ
    println!("$ git push -u origin {}", to);
    let output = Command::new("git")
        .args(["push", "-u", "origin", to])
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);

    // リポジトリのHEADをmainブランチに切り替え
    println!("$ git remote set-head origin {}", to);
    let output = Command::new("git")
        .args(["remote", "set-head", "origin", to])
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);
}

fn rename_default_branch(to: &str) {
    // TODO: fetch dynamically
    let repo = "daido1976/gh-default-branch";
    let repo = &format!("repos/{}", repo);
    let default_branch = &format!("default_branch={}", to);

    // GitHub上のデフォルトブランチをmainブランチに切り替え
    // gh api -X PATCH "repos/${REPO}" -f default_branch=main
    println!("$ gh api -X PATCH {} -f {}", repo, default_branch);
    Command::new("gh")
        .args(["api", "-X", "PATCH", repo, "-f", default_branch])
        .output()
        .expect("Failed to execute command");
    println!("default branch is updated!");

    // GitHub上の全てのPRのbase branchもmainブランチに切り替え
}
