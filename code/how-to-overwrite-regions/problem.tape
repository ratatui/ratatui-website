# This requires vhs installed from source, for the screenshot command and theme
# go install github.com/charmbracelet/vhs@main
Output overwrite-regions-problem.gif
Set Theme "Aardvark Blue"
Set Width 800
Set Height 450
Hide
Type "cargo run --bin problem"
Enter
Sleep 2s
Show
# requires vhs installed from source until the next version drops
Screenshot overwrite-regions-problem.png
Sleep 1s
Hide
Type q
Sleep 2s
