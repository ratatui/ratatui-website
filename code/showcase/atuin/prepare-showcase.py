"""Seed isolated, synthetic history for the Betamax showcase."""

import os
from pathlib import Path
import shutil
import sqlite3
import subprocess
import time

root = Path("target/showcase").resolve()
root.mkdir(parents=True, exist_ok=True)
home = root / "atuin"
# This directory belongs exclusively to the demo, so each render starts clean.
if home.exists():
    shutil.rmtree(home)
home.mkdir()
config = f'''db_path = "{home}/history.db"
record_store_path = "{home}/records.db"
key_path = "{home}/key"
session_path = "{home}/session"
auto_sync = false
update_check = false
style = "full"
inline_height = 0
show_help = true
show_preview = true
enter_accept = false
filter_mode = "global"
search_mode = "fuzzy"
[daemon]
enabled = false
[theme]
name = "atuin-site"
'''
(home / "config.toml").write_text(config)
(home / "themes").mkdir()
shutil.copyfile(Path(__file__).with_name("atuin-theme.toml"), home / "themes/atuin-site.toml")
commands = [
    "cargo build --release",
    "jj status",
    "cargo nextest run --workspace",
    "rg 'Terminal::new' examples/",
    "cargo run --example demo",
    "cargo doc --open",
    "ssh staging.example.com",
    "docker compose up -d postgres",
    "cargo run --example constraint-explorer",
    "cargo clippy --all-targets -- -D warnings",
    "curl -I https://ratatui.rs",
    "cargo run --example colors",
    "cargo +nightly fmt --check",
    "cargo run --example demo -- --tick-rate 100",
    "jj log -r 'trunk()..@'",
    "cargo run --example calendar",
    "cargo nextest run -p ratatui",
    "cargo run --example flex",
    "rustc --version",
    "cargo --version",
]
history = root / "history.zsh"
now = int(time.time())
history.write_text("".join(
    f": {now - (len(commands) - i) * 317}:{i % 8 + 1};{command}\n"
    for i, command in enumerate(commands)
))
env = dict(os.environ, ATUIN_HOME=str(home), ATUIN_CONFIG_DIR=str(home),
           XDG_DATA_HOME=str(root / "data"), HISTFILE=str(history))
subprocess.run(["atuin", "import", "zsh"], env=env, check=True)
with sqlite3.connect(home / "history.db") as db:
    db.execute("UPDATE history SET hostname = ?, cwd = ?, exit = 0",
               ("workstation:demo", "/home/demo/ratatui"))
