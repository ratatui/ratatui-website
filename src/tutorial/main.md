# Main.rs

The `main` file in many ratatui applications is simply a place to store the startup loop, and occasionally event handling. 
(See more ways this can be handled in [Concepts]())

In this application, we will be using our `main` function to run the startup steps, and start the main loop. We will also put our main loop logic and event handling in this file.

## Main

In our main function, we will set up the terminal, create an application state and run our application, and finally reset the terminal to the state we found it in.

### Application pre-run steps

Because a `ratatui` application take the whole screen, and capture all of the keyboard input, we need to run some boilerplate at the beginning of our `main` function. 

```rust,no_run,noplayground 
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use std::io;
}
{{#include ../../ratatui-book-tutorial-project/src/main.rs:setup_boilerplate}}
```

For more information, please read the crossterm documentation:
**NOTE: ACTUALLY PUT LINKS HERE**

### State creation, and loop starting

Now that we have prepared the terminal for our application to run, it is time to actually run it.

First, we need to create an instance of our `ApplicationState` or `app`, to hold all of the program's state, and then we will call our function which handles the event and draw loop.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:application_startup}}
```


### Application post-run steps

Since our `ratatui` has changed the state of the terminal with our [pre-run boilerplate](#Application pre-run steps) (A/N: CHECK THIS WORKS), we need to undo what have did, and put the terminal back to the way we found it.

Most of these functions will simply be the inverse of what we have done above.

```rust,no_run,noplayground 
use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
}
{{#include ../../ratatui-book-tutorial-project/src/main.rs:setup_boilerplate}}
```

The last three lines are there to catch any errors and print them before closing. This is important because we do not want our application erroring out without running this releasing boilerplate. 
When an application exits without running this closing boilerplate, the terminal will act very strange, and the user will usually have to end the terminal session and start a new on. Thus it is important that we handle our error in such a way that we can call this last piece.

So, altogether, our finished function should looks like this:

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:main_all}}
```


## run_app

In this function, we will start to do some actual logic. 

### Method signature
Let's start with the method signature:

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:run_method_signature}}
...
```

This method accepts an object of type `Terminal` which implements the `ratatui::backed::Backend` trait. This included the three (four counting the `TestBackend`) officially supported backends included in `ratatui`, but allows for 3rd party backends to be implemented. `run_app` also requires mutable ownership (A/N: MAKE SURE THIS IS THE CORRECT TERMINOLOGY) to an application state object, as defined in this project.

### UI Loop

Because `ratatui` requires us to implement our own event/ui loop, we will simply use the following code to update our main loop.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:ui_loop}}
...
```

Let's unpack that `draw` call really quick.
`terminal` is the `Terminal<Backend>` that we take as an arguement, `draw` is the `ratatui` command to draw widgets to the frame. `|f| ui(f, &app)` tells `draw` that we want to take `f: <Frame>` and pass it to our function `ui`, and ui will return a drawable frame. Notice that we also pass a immutable borrow of our application state to the `ui` function. This will be important later.

TODO: the event loop/handler

