---
title: Conclusion
---

We touched on the basic framework for building an `async` application with Ratatui, namely using
`tokio` and `crossterm`'s async features to create an `Event` and `Action` enum that contain
`Render` variants. We also saw how we could use `tokio` channels to send `Action`s to run domain
specific async operations concurrently.

There's more information in the documentation for a template that covers setting up a
[`Component` based architecture](/concepts/application-patterns/component-architecture/).
