# Third party widget showcase

This folder has [VHS] tape files to create gifs for the [Widget Showcase]. To run them, install VHS from
main (the theme and screenshot commands are not yet released).

```shell
go install vhs@main
```

Then run the tape.

```shell
~/go/bin/vhs tapefile.tape
```

Then copy the resulting gif over to `/src/content/docs/showcase/widgets/` and update
`src/content/docs/showcase/widgets/index.mdx`

Generally, try to avoid making animation super-frantic, and leave enough pauses for people to read
the text before it changes. If animation doesn't help the understanding of the way the widget works,
consider using a static image instead (use the `Screenshot` command in VHS to generate a `PNG`). We
use [git-lfs] to store the images in the repo to avoid repo bloat.

[VHS]: https://github.com/charmbracelet/vhs
[Widget Showcase]: https://ratatui.rs/showcase/widgets/#third-party-widgets
[git-lfs]: https://git-lfs.com/
