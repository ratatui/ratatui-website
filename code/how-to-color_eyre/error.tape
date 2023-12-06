# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/how-to/develop-apps/color-eyre/error.gif"
Set Theme "Aardvark Blue"
Set Width 1280
Set Height 450
Type "cargo run" Enter
Sleep 2s
Type "e"
Screenshot "../../src/content/docs/how-to/develop-apps/color-eyre/error.png"
Sleep 2s
