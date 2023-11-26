---
title: Ratatui
---

Check out the [CONTRIBUTING GUIDE](https://github.com/ratatui-org/ratatui/blob/main/CONTRIBUTING.md)
for more information.

## Keep PRs small, intentional and focused

Try to do one pull request per change. The time taken to review a PR grows exponential with the size
of the change. Small focused PRs will generally be much more faster to review. PRs that include both
refactoring (or reformatting) with actual changes are more difficult to review as every line of the
change becomes a place where a bug may have been introduced. Consider splitting refactoring /
reformatting changes into a separate PR from those that make a behavioral change, as the tests help
guarantee that the behavior is unchanged.

## Search `tui-rs` for similar work

The original fork of Ratatui, [`tui-rs`](https://github.com/fdehau/tui-rs/), has a large amount of
history of the project. Please search, read, link, and summarize any relevant
[issues](https://github.com/fdehau/tui-rs/issues/),
[discussions](https://github.com/fdehau/tui-rs/discussions/) and
[pull requests](https://github.com/fdehau/tui-rs/pulls).

## Use conventional commits

We use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) and check for them as
a lint build step. To help adhere to the format, we recommend to install
[Commitizen](https://commitizen-tools.github.io/commitizen/). By using this tool you automatically
follow the configuration defined in .cz.toml. Your commit messages should have enough information to
help someone reading the CHANGELOG understand what is new just from the title. The summary helps
expand on that to provide information that helps provide more context, describes the nature of the
problem that the commit is solving and any unintuitive effects of the change. It's rare that code
changes can easily communicate intent, so make sure this is clearly documented.

## Clean up your commits

The final version of your PR that will be committed to the repository should be rebased and tested
against main. Every commit will end up as a line in the changelog, so please squash commits that are
only formatting or incremental fixes to things brought up as part of the PR review. Aim for a single
commit (unless there is a strong reason to stack the commits). See
[Git Best Practices - On Sausage Making](https://sethrobertson.github.io/GitBestPractices/#sausage)
for more on this.

## Run CI tests before pushing a PR

We're using [cargo-husky](https://github.com/rhysd/cargo-husky) to automatically run git hooks,
which will run `cargo make ci` before each push. To initialize the hook run `cargo test`. If
`cargo-make` is not installed, it will provide instructions to install it for you. This will ensure
that your code is formatted, compiles and passes all tests before you push. If you need to skip this
check, you can use `git push --no-verify`.

## Sign your commits

We use commit signature verification, which will block commits from being merged via the UI unless
they are signed. To set up your machine to sign commits, see
[managing commit signature verification](https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification)
in GitHub docs.

## Setup

Clone the repo and build it using [cargo-make](https://sagiegurari.github.io/cargo-make/)

Ratatui is an ordinary Rust project where common tasks are managed with
[cargo-make](https://github.com/sagiegurari/cargo-make/). It wraps common `cargo` commands with sane
defaults depending on your platform of choice. Building the project should be as easy as running
`cargo make build`.

```shell
git clone https://github.com/ratatui-org/ratatui.git
cd ratatui
cargo make build
```

## Tests

The [test coverage](https://app.codecov.io/gh/ratatui-org/ratatui) of the crate is reasonably good,
but this can always be improved. Focus on keeping the tests simple and obvious and write unit tests
for all new or modified code. Beside the usual doc and unit tests, one of the most valuable test you
can write for Ratatui is a test against the `TestBackend`. It allows you to assert the content of
the output buffer that would have been flushed to the terminal after a given draw call. See
`widgets_block_renders` in tests/widgets_block.rs for an example.

When writing tests, generally prefer to write unit tests and doc tests directly in the code file
being tested rather than integration tests in the `tests/` folder.

If an area that you're making a change in is not tested, write tests to characterize the existing
behavior before changing it. This helps ensure that we don't introduce bugs to existing software
using Ratatui (and helps make it easy to migrate apps still using `tui-rs`).

For coverage, we have two [bacon](https://dystroy.org/bacon/) jobs (one for all tests, and one for
unit tests, keyboard shortcuts `v` and `u` respectively) that run
[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) to report the coverage. Several plugins
exist to show coverage directly in your editor. E.g.:

- <https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters>
- <https://github.com/alepez/vim-llvmcov>

## Use of unsafe for optimization purposes

We don't currently use any unsafe code in Ratatui, and would like to keep it that way. However there
may be specific cases that this becomes necessary in order to avoid slowness. Please see
[this discussion](https://github.com/ratatui-org/ratatui/discussions/66) for more about the
decision.
