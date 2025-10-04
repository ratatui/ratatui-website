---
title: Handle CLI arguments
sidebar:
  order: 1
  label: CLI Arguments
---

Command Line Interface (CLI) tools often require input parameters to dictate their behavior.
[`clap`](https://docs.rs/clap/latest/clap/) (Command Line Argument Parser) is a feature-rich Rust
library that facilitates the parsing of these arguments in an intuitive manner.

To use the derive syntax as shown in the following snippets, be sure to enable the `derive` feature
(i.e. by running `cargo add clap --features derive`).

## Defining Command Line Arguments

In this snippet, we utilize the `clap` library to define an `Args` struct, which will be used to
capture and structure the arguments passed to the application:

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = version(), about = "ratatui template with crossterm and tokio")]
struct Args {
  /// App tick rate
  #[arg(short, long, default_value_t = 1000)]
  app_tick_rate: u64,
}
```

Here, the Args struct defines one command-line arguments:

- `app_tick_rate`: Dictates the application's tick rate.

This is supplied with default values, ensuring that even if the user doesn't provide this argument,
the application can still proceed with its defaults.

## Displaying Version Information

One common convention in CLIs is the ability to display version information. Here, the version
information is presented as a combination of various parameters, including the Git commit hash.

The `version()` function, as seen in the snippet, fetches this information:

```rust
pub fn version() -> String {
  let author = clap::crate_authors!();

  let commit_hash = env!("RATATUI_TEMPLATE_GIT_INFO");

  // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
  let config_dir_path = get_config_dir().unwrap().display().to_string();
  let data_dir_path = get_data_dir().unwrap().display().to_string();

  format!(
    "\
{commit_hash}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
  )
}
```

This function uses the `get_data_dir()` and `get_config_dir()` from
[the section on XDG directories](../config-directories/).

This function also makes use of an environment variable `RATATUI_TEMPLATE_GIT_INFO` to derive the
Git commit hash. The variable can be populated during the build process by `build.rs`:

```rust
  println!("cargo:rustc-env=RATATUI_TEMPLATE_GIT_INFO={}", git_describe);
```

By invoking the CLI tool with the `--version` flag, users will be presented with the version
details, including the authors, commit hash, and the paths to the configuration and data
directories.

![--version output](https://user-images.githubusercontent.com/1813121/252721469-4d5ec38b-e868-46b4-b7b7-1c2c8bc496ac.png)

The `version()` function's output is just an example. You can easily adjust its content by amending
the string template code above.

Here's the full `build.rs` for your reference:

```rust
fn main() {
  let git_output = std::process::Command::new("git").args(["rev-parse", "--git-dir"]).output().ok();
  let git_dir = git_output.as_ref().and_then(|output| {
    std::str::from_utf8(&output.stdout).ok().and_then(|s| s.strip_suffix('\n').or_else(|| s.strip_suffix("\r\n")))
  });

  // Tell cargo to rebuild if the head or any relevant refs change.
  if let Some(git_dir) = git_dir {
    let git_path = std::path::Path::new(git_dir);
    let refs_path = git_path.join("refs");
    if git_path.join("HEAD").exists() {
      println!("cargo:rerun-if-changed={}/HEAD", git_dir);
    }
    if git_path.join("packed-refs").exists() {
      println!("cargo:rerun-if-changed={}/packed-refs", git_dir);
    }
    if refs_path.join("heads").exists() {
      println!("cargo:rerun-if-changed={}/refs/heads", git_dir);
    }
    if refs_path.join("tags").exists() {
      println!("cargo:rerun-if-changed={}/refs/tags", git_dir);
    }
  }

  let git_output =
    std::process::Command::new("git").args(["describe", "--always", "--tags", "--long", "--dirty"]).output().ok();
  let git_info = git_output.as_ref().and_then(|output| std::str::from_utf8(&output.stdout).ok().map(str::trim));
  let cargo_pkg_version = env!("CARGO_PKG_VERSION");

  // Default git_describe to cargo_pkg_version
  let mut git_describe = String::from(cargo_pkg_version);

  if let Some(git_info) = git_info {
    // If the `git_info` contains `CARGO_PKG_VERSION`, we simply use `git_info` as it is.
    // Otherwise, prepend `CARGO_PKG_VERSION` to `git_info`.
    if git_info.contains(cargo_pkg_version) {
      // Remove the 'g' before the commit sha
      let git_info = &git_info.replace('g', "");
      git_describe = git_info.to_string();
    } else {
      git_describe = format!("v{}-{}", cargo_pkg_version, git_info);
    }
  }

  println!("cargo:rustc-env=RATATUI_TEMPLATE_GIT_INFO={}", git_describe);
}

```
