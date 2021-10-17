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
# Rename default branch
$ gh default-branch rename main
```

## Release

```sh
$ ./release.sh <version(e.g.`v0.1`)>
# And please update the tag version in `gh-default-branch` file to <version>.
```
