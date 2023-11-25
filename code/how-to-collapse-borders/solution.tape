# This requires vhs installed from source, for the screenshot command and theme
# go install github.com/charmbracelet/vhs@main
Output collapse-borders-solution.gif
Set Theme "Aardvark Blue"
Set Width 800
Set Height 450
Type "cargo run --bin solution"
Enter
Sleep 2s
# requires vhs installed from source until the next version drops
Screenshot collapse-borders-solution.png
Sleep 1s
Type q
Sleep 2s
