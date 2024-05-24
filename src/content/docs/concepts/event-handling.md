---
title: Event Handling
---

There are many ways to handle events with the `ratatui` library. Mostly because `ratatui` does not
directly expose any event catching; the programmer will depend on the chosen backend's library.

However, there are a few ways to think about event handling that may help you. While this is not an
exhaustive list, it covers a few of the more common implementations. But remember, the correct way,
is the one that works for you and your current application.

## Centralized event handling

This is the simplest way to handle events because it handles all of the events as they appear. It is
often simply a match on the results of `event::read()?` (in crossterm) on the different supported
keys.

Pros: This has the advantage of requiring no message passing, and allows the programmer to edit all
of the possible keyboard events in one place.

Cons: However, this particular way of handling events simply does not scale well. Because _all_
events are handled in one place, you will be unable to split different groups of keybinds out into
separate locations.

## Centralized catching, message passing

This way of handling events involves polling for events in one place, and then sending
messages/calling sub functions with the event that was caught.

Pros: This has a similar appeal to the first method in its simplicity. With this paradigm, you can
easily split extensive pattern matching into sub functions that can go in separate files. This way
is also the idea often used in basic multi-threaded applications because message channels are used
to pass multi-threaded safe messages.

Cons: This method requires a main loop to be running to consistently poll for events in a
centralized place.

## Distributed event loops/segmented applications

In this style, control of the `Terminal` and the main loop to a sub-module. In this case, the entire
rendering and event handling responsibilities can be safely passed to the sub-module. In theory, an
application built like this doesn't need a centralized event listener.

Pros: There is no centralized event loop that you need to update whenever a new sub-module is
created.

Cons: However, if several sub-modules in your application have similar event handling loops, this
way could lead to a lot of duplicated code.
