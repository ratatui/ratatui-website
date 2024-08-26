# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/tutorials/counter-app/basic-app/basic-app.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 400
Type "cargo run" Enter
Sleep 3s
Screenshot "../../src/content/docs/tutorials/counter-app/basic-app/basic-app.png"
Set TypingSpeed 500ms
Right 3
Sleep 1s
Left 3
Sleep 2s
Type "q"
Sleep 2s
