# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "calendar.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 300
Type "cargo run -- -w calendar" Enter
Sleep 2s
Screenshot "calendar.png"
Sleep 1s