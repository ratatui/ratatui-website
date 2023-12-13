# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main

# for running against the throbber-widgets-tui git repo https://github.com/arkbig/throbber-widgets-tui
Output "throbber-widgets-tui.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 350
Hide
Type "cargo run --example demo" Enter
Sleep 4s
Show
Sleep 2s
