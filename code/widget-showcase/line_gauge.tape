# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/showcase/widgets/line_gauge.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 320
Type "cargo run -- -w line-gauge" Enter
Sleep 2s
Screenshot "../../src/content/docs/showcase/widgets/line_gauge.png"
Sleep 1s