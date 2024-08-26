# Widget Showcase

The purpose of this project is to create small single examples for display at
<https://ratatui.rs/showcase/widgets>. Contributions welcome!

It's encourage to make examples of third party widgets too using this project, though if they are
more advanced than the ones already in this repo (i.e. require anything beyond just rendering), you
may need to add another binary to the source project.

## Contributing

To add each widget:

- Add a widget to `enum Widget` in [main.rs](./src/main.rs)
- Add a new module to [examples.rs](./src/examples.rs)
- Add a render function to the module
- Call the render function from `render_frame` in `main.rs`
- Create a VHS tape (copy from the existing tapes) and edit the output and screenshot
- Run `~/go/bin/vhs <tapefile>.tape` (Install VHS from main to get the Aardvark Blue theme and the
  Screenshot command).
- Add the image to the [widgets showcase](../../src/content/docs/showcase/widgets/index.mdx)

## Design guidelines

For each example

- Make it look nice
- Keep it consistent with the other widgets
- Keep it simple (less is more) - example barchart has 3 items not 20.
- Avoid animation unless necessary (probably)
