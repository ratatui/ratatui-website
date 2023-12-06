# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/how-to/develop-apps/color-eyre/quit.gif"
Set Theme "Aardvark Blue"
Set Width 1280
Set Height 300
Type "cargo run" Enter
Sleep 2s
Type "q"
Screenshot "../../src/content/docs/how-to/develop-apps/color-eyre/quit.png"
Sleep 2s
