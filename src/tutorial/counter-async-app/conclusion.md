# Conclusion

We touched on the basic framework for building an `async` application with Ratatui, namely using
`tokio` and `crossterm`'s async features to create an `Event` and `Action` enum that contain
`Render` variants. We also saw how we could use `tokio` channels to send `Action`s to run domain
specific async operations concurrently.

There's more information in
[`ratatui-async-template`](https://github.com/ratatui-org/ratatui-async-template) about structuring
an `async` application. The template also covers setting up a
[`Component` based architecture](../../concepts/application-patterns/component-architecture.md).

For more information, refer to the documentation for the template:
<https://ratatui-org.github.io/ratatui-async-template/>
