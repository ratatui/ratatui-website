# This requires vhs installed from source, for the screenshot command and theme
# go install github.com/charmbracelet/vhs@main
Output overwrite-regions-solution.gif
Set Theme "Aardvark Blue"
Set Width 800
Set Height 450
Hide
Type "cargo run --bin solution"
Enter
Sleep 2s
Show
# requires vhs installed from source until the next version drops
Screenshot overwrite-regions-solution.png
Sleep 1s
Hide
Type q
Sleep 2s
