# This requires vhs installed from source for the updated theme and screenshot command
# go install github.com/charmbracelet/vhs@main
Output "../../src/content/docs/tutorials/counter-app/error-handling/error-full.gif"
Set Theme "Aardvark Blue"
Set Width 1280
Set Height 1850
Type "RUST_BACKTRACE=full cargo run" Enter
Sleep 2s
Right 3
Sleep 3s
Screenshot "../../src/content/docs/tutorials/counter-app/error-handling/error-full.png"
