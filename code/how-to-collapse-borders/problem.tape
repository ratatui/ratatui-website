# This requires vhs installed from source, for the screenshot command and theme
# go install github.com/charmbracelet/vhs@main
Output collapse-borders-problem.gif
Set Theme "Aardvark Blue"
Set Width 800
Set Height 450
Type "cargo run --bin problem"
Enter
Sleep 2s
# requires vhs installed from source until the next version drops
Screenshot collapse-borders-problem.png
Sleep 1s
Type q
Sleep 2s
