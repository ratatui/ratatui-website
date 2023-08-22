# Introduction

`ratatui` is based on the principle of immediate rendering with intermediate buffers.
This means that at each new frame you have to build all widgets that are supposed to be part of the UI.
In short, the `ratatui` library handles drawing to the terminal.

Additionally, the library does not provide any input handling nor any event system.
The responsibility of getting keyboard input events, modifying the state of your application based on those events and figuring out which widgets best reflect the view of the state of your application is on you.

## Who is ratatui for?

`ratatui` is designed for developers and enthusiasts who:

- appreciate the retro aesthetic of the terminal,
- want a lightweight alternative to graphical user interfaces (GUIs),
- need applications that are to be deployed in constrained environments, like on servers with limited resources, and
- prefer to have full control over input and events, allowing for a more customized and tailored user experience.
