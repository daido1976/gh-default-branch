use std::process::{Command, Output};

pub fn show() {
    let repo_name_with_owner = get_repo_name_with_owner();
    let default_branch = get_default_branch(&repo_name_with_owner);
    println!("Current default branch is \"{}\"", default_branch,);
}

// TODO: 副作用の大きいコマンドなので対話式にする
pub fn rename(to_branch: &str) {
    let repo_name_with_owner = get_repo_name_with_owner();
    let from_branch = get_default_branch(&repo_name_with_owner);
    println!(
        "=== START: Rename from {} to {} ===",
        from_branch, to_branch
    );
    fetch_origin(&from_branch);
    push_new_branch(&from_branch, to_branch);
    rename_default_branch(to_branch, &repo_name_with_owner);
    delete_old_branch(&from_branch);
    println!(
        "=== FINISH: Rename from {} to {} ===",
        from_branch, to_branch
    );
}

fn delete_old_branch(branch: &str) {
    // Gitのリモートリポジトリからmasterブランチを削除
    println!("$ git push --delete origin {}", branch);
    let output = git(&["push", "--delete", "origin", branch]);
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);

    // ローカルのmasterブランチは必要があれば消してください
    // println!("$ git branch -D {}", branch);
    // let output = git(&["branch", "-D", branch]);
    // let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    // println!("{}", output);
}

pub fn help() {
    println!("TODO: show help message");
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
    // masterブランチからmainブランチを強制的に分岐させる
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

    // mainブランチをリモートにプッシュ
    println!("$ git push -u origin {}", to);
    let output = git(&["push", "-u", "origin", to]);
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);

    // リポジトリのHEADをmainブランチに切り替え
    println!("$ git remote set-head origin {}", to);
    let output = git(&["remote", "set-head", "origin", to]);
    let output = String::from_utf8_lossy(&output.stderr).trim().to_string();
    println!("{}", output);
}

fn rename_default_branch(to: &str, repo: &str) {
    let repo = &format!("repos/{}", repo);
    let default_branch = &format!("default_branch={}", to);

    // GitHub上のデフォルトブランチをmainブランチに切り替え
    // gh api -X PATCH "repos/${REPO}" -f default_branch=main
    println!("$ gh api -X PATCH {} -f {}", repo, default_branch);
    gh(&["api", "-X", "PATCH", repo, "-f", default_branch]);
    println!("default branch is updated!");

    // TODO: GitHub上の全てのPRのbase branchもmainブランチに切り替え
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
