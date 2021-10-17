# gh-default-branch

GitHub CLI extension to show & rename the default branch.

`rename` subcommand was inspired by [this gist](https://gist.github.com/mislav/5ac69530acbe1b4ca909e272caabfdba).

### ⚠️ Caution

The `rename` subcommand is missing some features.

If you want any of the following missing features, try to change the default branch name from the GUI.

See. https://github.com/github/renaming#renaming-existing-branches

> - [x] Re-target any open pull requests
> - [ ] Update any draft releases based on the branch
> - [ ] Move any branch protection rules that explicitly reference the old name
> - [ ] Update the branch used to build GitHub Pages, if applicable
> - [ ] Show a notice to repository contributors, maintainers, and admins on the repository homepage with instructions to update local copies of the repository
> - [ ] Show a notice to contributors who git push to the old branch
> - [ ] Redirect web requests for the old branch name to the new branch name
> - [ ] Return a "Moved Permanently" response in API requests for the old branch name

Also see. https://github.com/cli/cli/issues/1215

## Installation

```
$ gh extension install daido1976/gh-default-branch
```

## Usage

```sh
# Show default branch
$ gh default-branch show
# Current default branch is "master"

# Rename default branch
$ gh default-branch rename main
# The following execution log will be output.
#
# === START: Rename from master to main ===
# $ git fetch origin master
# From https://github.com/daido1976/gh-default-branch
#  * branch            master     -> FETCH_HEAD
# $ git checkout -B main origin/master --no-track
# Switched to and reset branch 'main'
# $ git push -u origin main
# remote:
# remote: Create a pull request for 'main' on GitHub by visiting:
# remote:      https://github.com/daido1976/gh-default-branch/pull/new/main
# remote:
# remote: Heads up! The branch 'main' that you pushed to was renamed to 'master'.
# remote:
# To https://github.com/daido1976/gh-default-branch.git
#  * [new branch]      main -> main
# $ git remote set-head origin main
# Set HEAD to main
# $ gh api -X PATCH repos/daido1976/gh-default-branch -f default_branch=main
# Default branch is renamed to main!
# $ gh api -X PATCH repos/daido1976/gh-default-branch/pulls/2 -f base=main
# $ gh api -X PATCH repos/daido1976/gh-default-branch/pulls/1 -f base=main
# All PR's base branch are updated!
# $ git push --delete origin master
# To https://github.com/daido1976/gh-default-branch.git
#  - [deleted]         master
# === FINISH: Rename from master to main ===
```

## Release

```sh
$ ./release.sh <version(e.g.`v0.1`)>
# And please update the tag version in `gh-default-branch` file to <version>.
```
