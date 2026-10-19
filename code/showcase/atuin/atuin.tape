Output target/showcase/atuin-run.gif
Output target/showcase/atuin-run.webp

Set Shell "bash"
Set Width 1200
Set Height 720
Set FontSize 22
Set Framerate 20
Set TypingSpeed 85ms
Set Theme '{"name":"Atuin","background":"#101620","foreground":"#e8f0e9","cursor":"#38c85a","black":"#273338","red":"#e78383","green":"#38c85a","yellow":"#d8bc78","blue":"#0088d1","magenta":"#b79cdb","cyan":"#55b9bd","white":"#cbd8d6","brightBlack":"#8fa1a8","brightRed":"#f2a3a3","brightGreen":"#a8ecba","brightYellow":"#f0d9a0","brightBlue":"#4ec2f8","brightMagenta":"#d2b9f2","brightCyan":"#8edce0","brightWhite":"#e8f0e9"}'
Set Margin 6
Set MarginFill transparent
Set Shadow false
Set WindowBarColor "#101620"
Set WindowBar Rings
Set WindowTitle "atuin · shell history"
Set WindowTitleColor "#a8ecba"
Set WindowBorderColor "#38c85a"
Set WindowBorderOpacity 0.3
Set BorderRadius 12
Set KeyboardOverlay Input
Set KeyboardOverlayLocation CaptionRow

Hide
Type@1ms "source code/showcase/atuin/showcase-shell.bash"
Enter
Sleep 1s
Caption "Shell history supercharged"
Show
Sleep 600ms
Type "cargo"
Sleep 700ms
Ctrl+R
Sleep 1s
AssertText "cargo run --example flex"
Sleep 1500ms
Screenshot target/showcase/history-run.png
Caption "Find the command you need"
Type " --v"
Sleep 1500ms
AssertText "cargo --version"
AssertAbsent "rustc --version"
Screenshot target/showcase/search-run.png
Caption "Recall it to your shell"
Tab
Sleep 1500ms
AssertText "❯ cargo --version"
AssertAbsent "Error:"
Screenshot target/showcase/recall-run.png
Caption "Run it"
Enter
Sleep 2s
AssertText "cargo 1."
AssertAbsent "Error:"
Screenshot target/showcase/executed-run.png
State target/showcase/executed-run.json
Hide
Ctrl+C
