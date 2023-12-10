# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "bar_chart.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 300
Type "cargo run -- -w bar-chart" Enter
Sleep 2s
Screenshot "bar_chart.png"
Sleep 1s