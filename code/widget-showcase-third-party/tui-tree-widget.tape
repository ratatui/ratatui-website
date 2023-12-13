# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main

# for running against the tui-tree-widget git repo https://github.com/EdJoPaTo/tui-rs-tree-widget
Output "tui-tree-widget.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 450
Hide
Type "cargo run --example example" Enter
Sleep 4s
Set TypingSpeed 200ms
Show
Down 2
Right @1s
Down 2
Right @1s
Down 2
Left @1s 5
Sleep 2s
