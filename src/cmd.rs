use std::process::{Command, Output};

pub fn show() {
    let repo_name_with_owner = get_repo_name_with_owner();
    let default_branch = get_default_branch(&repo_name_with_owner);
    println!("Current default branch is \"{}\"", default_branch,);
}

// TODO: Make it interactive because this is a command with a lot of side effects.
pub fn rename(to_branch: &str) {
    let repo_name_with_owner = get_repo_name_with_owner();
    let from_branch = get_default_branch(&repo_name_with_owner);
    println!(
        "=== START: Rename from {} to {} ===",
        from_branch, to_branch
    );

    fetch_origin(&from_branch);
    push_new_branch(&from_branch, to_branch);
    rename_default_branch(&from_branch, to_branch, &repo_name_with_owner);
    delete_old_branch(&from_branch);

    println!(
        "=== FINISH: Rename from {} to {} ===",
        from_branch, to_branch
    );
}

/// e.g. daido1976/gh-default-branch
fn get_repo_name_with_owner() -> String {
    // gh repo view --json nameWithOwner --jq .nameWithOwner
    let output = gh(&[
        "repo",
        "view",
        "--json",
        "nameWithOwner",
        "--jq",
        ".nameWithOwner",
    ]);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn get_default_branch(repo_name_with_owner: &str) -> String {
    // gh api "repos/daido1976/gh-default-branch" --jq '.default_branch'
    let output = gh(&[
        "api",
        &format!("repos/{}", repo_name_with_owner),
        "--jq",
        ".default_branch",
    ]);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn fetch_origin(branch: &str) {
    println!("$ git fetch origin {}", branch);
    let output = git(&["fetch", "origin", branch]);
    // git outputs the fetch and push logs to stderr...
    // See. https://stackoverflow.com/questions/57016157/stop-git-from-writing-non-errors-to-stderr
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);
}

/// * `from` - e.g. master
/// * `to` - e.g. main
fn push_new_branch(from: &str, to: &str) {
    // Force a branch from old branch to new branch.
    println!("$ git checkout -B {} origin/{} --no-track", to, from);
    let output = git(&[
        "checkout",
        "-B",
        to,
        &format!("origin/{}", from),
        "--no-track",
    ]);
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);
    // handle error
    // TODO: Use Result instead of panic!
    output
        .starts_with("error:")
        .then(|| panic!("Exit the program because the git command failed!"));

    // Push the new branch to the remote.
    println!("$ git push -u origin {}", to);
    let output = git(&["push", "-u", "origin", to]);
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);

    // Set the repository HEAD to the new branch.
    println!("$ git remote set-head origin {}", to);
    git(&["remote", "set-head", "origin", to]);
    println!("Set HEAD to {}", to);
}

fn rename_default_branch(from: &str, to: &str, repo: &str) {
    let repo = &format!("repos/{}", repo);
    let default_branch = &format!("default_branch={}", to);

    // Rename the default branch on GitHub to the new branch.
    println!("$ gh api -X PATCH {} -f {}", repo, default_branch);
    gh(&["api", "-X", "PATCH", repo, "-f", default_branch]);
    println!("Default branch is renamed to {}!", to);

    // Switch the base branch of all PRs on GitHub to the new branch.
    // gh pr list -B master -L999 --json number --jq '.[].number'
    let output = gh(&[
        "pr",
        "list",
        "-B",
        from,
        "-L999",
        "--json",
        "number",
        "--jq",
        ".[].number",
    ]);
    let output = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let pr_numbers: Vec<String> = output.split('\n').map(String::from).collect();
    for num in pr_numbers {
        let target_pr = &format!("{}/pulls/{}", repo, num);
        let base_branch = &format!("base={}", to);
        println!("$ gh api -X PATCH {} -f {}", target_pr, base_branch);
        gh(&["api", "-X", "PATCH", target_pr, "-f", base_branch]);
    }
    println!("All PR's base branch are updated!");
}

fn delete_old_branch(branch: &str) {
    // Delete the old branch from the remote Git repository.
    println!("$ git push --delete origin {}", branch);
    let output = git(&["push", "--delete", "origin", branch]);
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);

    // Delete the local old branch.
    println!("$ git branch -D {}", branch);
    git(&["branch", "-D", branch]);
    println!("Deleted the local old branch!");
}

fn git(args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .output()
        .expect("Failed to execute command")
}

fn gh(args: &[&str]) -> Output {
    Command::new("gh")
        .args(args)
        .output()
        .expect("Failed to execute command")
}
