# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main

# for running against the tui-tree-widget git repo https://git.sr.ht/~jaxter184/tui-nodes
Output "tui-nodes.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 400
Set CursorBlink false
Hide
Type "cargo run --example tiny" Enter
Sleep 4s
Show
Sleep 2s