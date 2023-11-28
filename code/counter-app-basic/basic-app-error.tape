# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/tutorials/counter-app/basic-app/basic-app-error.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 400
Type "cargo run" Enter
Sleep 3s
Set TypingSpeed 500ms
Left
Sleep 1s
Screenshot "../../src/content/docs/tutorials/counter-app/basic-app/basic-app-error.png"
Sleep 1s
