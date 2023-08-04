# Rendering
[Stub article]

Ratatui is different from GUI frameworks you might use, because it only updates when you tell it to. Only calling `<Backend>.draw()` will actually output anything to the screen. This is important to understand because what it means is *you*, the programmer, are responsible for keeping the TUI responsive. If you accidentally block the thread that updates the UI, it will not update until the thread unblocks. 

