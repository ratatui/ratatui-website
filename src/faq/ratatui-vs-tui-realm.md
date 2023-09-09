# `ratatui` vs `tui-realm`

Fundamentally, the difference is that **`ratatui` is a library** but
**[`tui-realm`](https://github.com/veeso/tui-realm/) is a framework**.

The terms library and framework are often used interchangeably in software development, but they
serve different purposes and have distinct characteristics.

## Library

- **Usage**: A library is a collection of functions and procedures that a programmer can call in
  their application. The library provides specific functionality, but it's the developer's
  responsibility to explicitly call and use it.
- **Control Flow**: In the case of a library, the control flow remains with the developer's
  application. The developer chooses when and where to use the library.
- **Passivity**: Libraries are passive in nature. They wait for the application's code to invoke
  their methods.
- **Example**: Imagine you're building a house. A library would be like a toolbox with tools
  (functions) that you can use at will. You decide when and where to use each tool.

## Framework

- **Usage**: A framework is a pre-built structure or scaffold that developers build their
  application within. It provides a foundation, enforcing a particular way of creating an
  application.
- **Control Flow**: With a framework, the control flow is inverted. The framework decides the flow
  of control by providing places for the developer to plug in their own logic (often referred to as
  "Inversion of Control" or IoC).
- **Activeness**: Frameworks are active and have a predefined flow of their own. The developer fills
  in specific pieces of the framework with their own code.
- **Example**: Using the house-building analogy, a framework would be like a prefabricated house
  where the main structure is already built. You're tasked with filling in the interiors and decor,
  but you have to follow the design and architecture already provided by the prefabricated design.

While `ratatui` provides tools (widgets) for building terminal UIs, it doesn't dictate or enforce a
specific way to structure your application. You need to decide how to best use the library in your
particular context, giving you more flexibility.

In contrast, `tui-realm` might provide more guidelines and enforcements about how your application
should be structured or how data flows through it. And, for the price of that freedom, you get more
features out of the box with `tui-realm` and potentially lesser code in your application to do the
same thing that you would with `ratatui`.
