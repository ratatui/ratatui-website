# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/showcase/widgets/block.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 300
Type "cargo run -- -w block" Enter
Sleep 2s
Screenshot "../../src/content/docs/showcase/widgets/block.png"
Sleep 1s