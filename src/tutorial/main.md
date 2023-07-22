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
```

```rust,no_run,noplayground 
{{#include ../../ratatui-book-tutorial-project/src/main.rs:setup_boilerplate}}
```

You might notice that we are using `stderr` for our output. This is because we want to allow the user to pipe their completed json to other programs like `ratatui-tutorial > output.json`. To do this, we are utilizing the fact that `stderr` is piped differently than `stdout`, and rendering out project in `stderr`, and printout our completed json in `stdout`

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
```
```rust,no_run,noplayground 
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
```

Let's unpack that `draw` call really quick.
`terminal` is the `Terminal<Backend>` that we take as an arguement, `draw` is the `ratatui` command to draw widgets to the frame. `|f| ui(f, &app)` tells `draw` that we want to take `f: <Frame>` and pass it to our function `ui`, and ui will return a drawable frame. Notice that we also pass a immutable borrow of our application state to the `ui` function. This will be important later.

### Event handling
Now that we have started our app, and have set up the ui rendering, we will implement the event handling.


#### Polling 
Because we are using crossterm, we simply poll for keyboard events with

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:event_poll}}
```

and then match the results. Please note that the process for polling events will vary on the backend you are utilizing, and you will need to refer to the documentation of that backend for more information.

#### Main Screen
We will start with the keybinds and event handling for the `CurrentScreen::Main`. 

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:main_screen}}
```

After matching to the `Main` enum varient, we match the event. When the user is in the main screen, there are only two keybinds, and the rest are ignored. 

In this case, `KeyCode::Char('e')` changes the current screen to `CurrentScreen::Editing` and set the `CurrentlyEditing` to a `Some` and notes that the user should be editing the `Key` value field, as opposed to the `Value` field.

`KeyCode::Char('q')` is straightforward, as it simply switches the application to the `Exiting` screen, and allows the ui and future event handling runs to do the rest.

#### Exiting
The next handler we will prepare, will handle events while the application is on the `CurrentScreen::Exiting`. The job of this screen is to ask if the user wants to exit without outputing the json. It is simply a `y/n` question, so that is all we listen for. We also add an alternate exit key with `q`.
If the user chooses to output the json, we call `app.print_json()` to perform the serialization and printing for us.

```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:exiting_screen}}
```


#### Editing
Our final handler will be a bit more involved, as we will be changing the state of internal variables.


We would like the `Enter` key to serve two purposes. When the user is editing the `Key`, we want the enter key to switch the focus to editing the `Value`. However, if the `Value` is what is being currently edited, `Enter` will save the key-value pair, and return to the `Main` screen.
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:editing_enter}}
```


When `Backspace` is pressed, we need to first determine if the user is editing a `Key` or a `Value`, then `pop()` the endings of those strings accordingly.
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:backspace_editing}}
```

When `Escape` is pressed, we want to quit editing.
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:escape_editing}}
```

When `Tab` is pressed, we want the currently editing selection to switch.
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:tab_editing}}
```

And finally, if the user types a valid character, we want to capture that, and add it to the string that is the final key or value.
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:character_editing}}
```

Altogether, the event loop should look like this:
```rust,no_run,noplayground
{{#include ../../ratatui-book-tutorial-project/src/main.rs:event_poll}}
```

