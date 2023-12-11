# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/showcase/widgets/chart.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 300
Type "cargo run -- -w chart" Enter
Sleep 2s
Screenshot "../../src/content/docs/showcase/widgets/chart.png"
Sleep 1s