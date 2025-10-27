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
Sleep 3s
Show
Sleep 2s

# Introduction - show the initial state
Sleep 2s

# Basic Checkbox Section (Index 0)
# Currently checked - toggle off
Space
Sleep 1s
# Toggle back on
Space
Sleep 2s

# Skip over Styled Checkboxes (Info, Warning, Error)
# Just move through them quickly
Down
Sleep 0.3s
Down
Sleep 0.3s
Down
Sleep 0.3s

# Move to Emoji & Unicode Section
Down
Sleep 1s

# Index 4: Check mark emoji - currently checked
# Toggle to show both states
Space
Sleep 1.5s
Space
Sleep 2s

# Index 5: Circle - currently checked
Down
Sleep 0.8s
Space
Sleep 1.5s
Space
Sleep 2s

# Index 6: Diamond - currently checked
Down
Sleep 0.8s
Space
Sleep 1.5s
Space
Sleep 2s

# Move to Custom ASCII Section - THIS IS THE MAIN FOCUS
Down
Sleep 1.5s

# Index 7: ASCII [X] / [ ] - currently checked
# Show both states
Space
Sleep 2s
Space
Sleep 2s

# Index 8: Asterisk [*] / [ ] - currently unchecked
Down
Sleep 1s
# Check it to show [*]
Space
Sleep 2s
# Uncheck to show [ ]
Space
Sleep 2s

# Index 9: Plus/Minus [+] / [-] (DYNAMIC LABEL!)
# This is special - label changes based on state
Down
Sleep 1s
# Currently shows "Plus sign" with [+]
Sleep 1s
# Toggle to show "Minus sign" with [-]
Space
Sleep 3s
# Toggle back to show "Plus sign" with [+]
Space
Sleep 3s

# Index 10: X/O style (X) / (O) - currently unchecked shows (O)
Down
Sleep 1s
# Show (O) state
Sleep 1s
# Toggle to show (X)
Space
Sleep 2s
# Toggle back to (O)
Space
Sleep 2s
# Toggle once more to (X)
Space
Sleep 2s

# Navigate back to top to show final overview
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s
Up
Sleep 0.3s

# Show final state with all custom symbols visible
Sleep 3s

Hide
Escape
Sleep 1s
