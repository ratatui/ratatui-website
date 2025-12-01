# A VHS tape. See https://github.com/charmbracelet/vhs
Output "codex.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 1000

Hide
# Start Codex and ask it to build the sample app
Type "codex --model gpt-5-codex --config model_reasoning_effort=low"
Enter
Sleep 1s
Show
Sleep 1s
Type "Create a simple Ratatui app named dashboard-app that displays some fake data."
Enter
Sleep 5s

# Hide/show cadence to keep the runtime around three minutes
Hide
Sleep 25s
Show
Sleep 5s

Hide
Sleep 25s
Show
Sleep 5s

Hide
Sleep 25s
Show
Sleep 5s

Hide
Sleep 25s
Show
Sleep 5s

Hide
Sleep 55s
Show
Sleep 5s

Screenshot "codex.png"
Sleep 1s

# Clean exit
Ctrl+C
Ctrl+C
Sleep 1s
Type "YOLO: let's run it!"
Sleep 1s
Ctrl+U
Type "cargo run -p dashboard-app"
Enter
Sleep 5s
Type "q"
Enter
