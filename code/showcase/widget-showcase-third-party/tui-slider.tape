# A VHS tape. See https://github.com/charmbracelet/vhs

# for running against the tui-slider git repo https://github.com/sorinirimies/tui-slider
Output "tui-slider.gif"
Set Theme "Aardvark Blue"
Set Width 1600
Set Height 1000
Set FontSize 16
Set Padding 20

Hide
Type "cargo run --release --example horizontal"
Enter
Sleep 5s
Show

# Show horizontal sliders - page 1 (Standard Styles)
Sleep 1.5s

# Adjust some sliders quickly
Down
Sleep 0.2s
Right Right Right
Sleep 0.2s
Down
Sleep 0.2s
Left Left
Sleep 0.2s
Down
Sleep 0.2s
Right Right
Sleep 0.5s

# Navigate to page 2 (Specialty Styles)
Type "n"
Sleep 1s

# Adjust sliders on page 2
Down
Sleep 0.2s
Right Right
Sleep 0.2s
Down
Sleep 0.2s
Left
Sleep 0.8s

# Exit and show vertical sliders
Hide
Type "q"
Sleep 0.2s
Type "cargo run --release --example vertical"
Enter
Sleep 5s
Show

# Demonstrate vertical sliders
Sleep 1s
Down
Sleep 0.2s
Up Up
Sleep 0.2s
Right
Sleep 0.2s
Down
Sleep 0.2s
Up
Sleep 1s

Hide
Type "q"
Sleep 0.3s
