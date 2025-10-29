# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-checkbox git repo https://github.com/sorinirimies/tui-checkbox
Output "tui-checkbox.gif"
Set Theme "Aardvark Blue"
Set Width 1300
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example checkbox"
Enter
Sleep 1.5s
Show
Sleep 0.5s

# Basic Checkbox - toggle both states
Space
Sleep 0.3s
Space
Sleep 0.5s

# Navigate through styled checkboxes
Down
Down
Down
Down
Sleep 0.3s

# Emoji & Unicode - quick toggles
Space
Sleep 0.3s
Space
Sleep 0.5s

Down
Space
Sleep 0.3s
Space
Sleep 0.5s

Down
Space
Sleep 0.3s
Space
Sleep 0.5s

# Custom ASCII Section - MAIN FOCUS
Down
Sleep 0.5s

# ASCII [X] / [ ]
Space
Sleep 0.5s
Space
Sleep 0.5s

# Asterisk [*] / [ ]
Down
Space
Sleep 0.5s
Space
Sleep 0.5s

# Plus/Minus [+] / [-] - dynamic label
Down
Space
Sleep 1s
Space
Sleep 1s

# X/O style (X) / (O)
Down
Space
Sleep 0.5s
Space
Sleep 0.5s
Space
Sleep 0.5s

# Back to top
Up
Up
Up
Up
Up
Up
Up
Up
Up
Up
Sleep 1s

Hide
Escape
Sleep 0.3s
