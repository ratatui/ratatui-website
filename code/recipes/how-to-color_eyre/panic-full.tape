# A VHS tape. See https://github.com/charmbracelet/vhs
Output "../../src/content/docs/how-to/develop-apps/color-eyre/panic-full.gif"
Set Theme "Aardvark Blue"
Set Width 1280
Set Height 900
Type "RUST_BACKTRACE=full cargo run" Enter
Sleep 2s
Type "p"
Sleep 1s
Screenshot "../../src/content/docs/how-to/develop-apps/color-eyre/panic-full.png"
Sleep 2s
