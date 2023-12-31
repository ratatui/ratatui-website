# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/showcase/widgets/gauge.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 420
Type "cargo run -- -w gauge" Enter
Sleep 2s
Screenshot "../../src/content/docs/showcase/widgets/gauge.png"
Sleep 1s