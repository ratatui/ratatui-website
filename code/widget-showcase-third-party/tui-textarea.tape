# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-textarea git repo https://github.com/rhysd/tui-textarea
Output "tui-textarea.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 450
Hide
Type "cargo run --example editor --features search -- README.md" Enter
Sleep 4s
Show
Down @200ms 10
Sleep 2s
Type "This is a demo of the tui-textarea crate." Enter
Sleep 2s
Ctrl+G
Type "term.draw" 
Sleep 2s
Enter
Sleep 2s
Down @200ms 10
Sleep 2s
Down @200ms 10
Sleep 2s
