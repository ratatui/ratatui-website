# Conclusion

We touched on the basic framework for building an `async` application with Ratatui, namely using
`tokio` and `crossterm`'s async features to create `Event`s appropriately. We also saw how to spawn
a `tokio` task to schedule events using an `Action` channel.

There's a lot more information in
[`ratatui-async-template`](https://github.com/ratatui-org/ratatui-async-template) about structuring
an `async` application. The template also covers setting up a
[`Component` based architecture](../../concepts/application-patterns/component-architecture.md).

For more information, refer to the documentation for the template:
<https://ratatui-org.github.io/ratatui-async-template/>
