# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "block.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 300
Type "cargo run -- -w block" Enter
Sleep 2s
Screenshot "block.png"
Sleep 1s