# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-piechart git repo https://github.com/sorinirimies/tui-piechart
Output "tui-piechart.gif"
Set Theme "Aardvark Blue"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --example high_resolution"
Enter
Sleep 2s
Show

# Start in Standard resolution mode - let animation play
Sleep 4s

# Toggle to High resolution (Braille) to see the smooth rendering
Space
Sleep 4s

# Toggle back to Standard to show the difference
Space
Sleep 3s

# Final toggle to High resolution for showcase of smooth animation
Space
Sleep 5s

Hide
Type "q"
Sleep 0.3s
