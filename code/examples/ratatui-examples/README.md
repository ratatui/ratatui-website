# Ratatui Examples

This crate contains the source shown on the website's example pages. It is a flattened, buildable
copy of selected examples from the Ratatui `ratatui-v0.30.2` release:

- Tag: `ratatui-v0.30.2`
- Commit: `e665c36cb14752a61cd777fbd06dbef8474f2add`

Do not edit copied examples here. Make source changes in the
[Ratatui repository](https://github.com/ratatui/ratatui), then update this copy with `sync.sh`.

## Source mapping

The website crate combines two upstream source trees:

| Upstream source                               | Destination                        |
| --------------------------------------------- | ---------------------------------- |
| Selected `examples/apps/*/src` packages       | Stable flattened `examples/` names |
| Every `ratatui-widgets/examples/*.rs` example | Compatible `examples/` names       |

`sync.sh` is the exact file-by-file mapping. In particular:

- `advanced-widget-impl/src/main.rs` remains `widget_impl.rs`.
- `async-github/src/main.rs` remains `async.rs`.
- `color-explorer/src/main.rs` remains `colors.rs`.
- `logo.rs` remains `ratatui-logo.rs`.
- `line-gauge.rs` remains `line_gauge.rs`.
- The complete `demo` and `demo2` module trees are copied, including the Termina backend.
- New widget examples such as `collapsed-borders.rs` and `shadow.rs` are copied and exposed through
  standalone website pages.
- Image-backed upstream app packages use distinct directories such as `canvas-app/` when their names
  would otherwise collide with canonical widget examples.

These names preserve existing page includes, local commands, and public example routes.

## Intentional exceptions

The combined `examples/README.md`, `docsrs.rs`, and `layout.rs` are website-only compatibility
files. They are not overwritten. The README preserves the flattened crate's application and widget
index, commands, and design notes; the upstream application and widget READMEs describe two separate
package trees and cannot be copied into this combined layout without breaking their paths and
commands. The Layout page uses the maintained upstream `constraint-explorer` source while the old
`layout.rs` file remains runnable for existing local links and commands.

`user_input.rs` keeps the website's link to the Ratatui-maintained
[`ratatui-textarea`](https://github.com/ratatui/ratatui-textarea) repository from
[PR 1089](https://github.com/ratatui/ratatui-website/pull/1089). The release source still links to
the repository from before that project moved. `sync.sh` reapplies this single-line override and
fails if the expected upstream text is no longer present.

The image-backed `advanced-widget-impl`, `async-github`, `calendar-explorer`, `canvas`, `chart`,
`gauge`, `hyperlink`, `scrollbar`, `table`, `todo-list`, and `tracing` packages have standalone Apps
pages. Canvas, Chart, Gauge, Scrollbar, and Table remain separate from the existing Widgets pages
because the app packages are richer interactive demos while the widget pages show the canonical
`ratatui-widgets` examples.

The upstream images branch does not contain captures for `input-form`, `mouse-drawing`,
`release-header`, `volatility-surface`, `weather`, or `widget-ref-container`. Since every
established Apps page includes an upstream capture, those packages remain runnable from the upstream
workspace instead of being partially represented here.

## Reproduce the sync

Verify that the annotated tag still peels to the recorded commit:

```shell
git ls-remote https://github.com/ratatui/ratatui.git \
  refs/tags/ratatui-v0.30.2 'refs/tags/ratatui-v0.30.2^{}'
```

Download the immutable commit archive and run the explicit mapping:

```shell
commit=e665c36cb14752a61cd777fbd06dbef8474f2add
curl --fail --location \
  "https://github.com/ratatui/ratatui/archive/${commit}.tar.gz" \
  --output "/tmp/ratatui-${commit}.tar.gz"
tar -xzf "/tmp/ratatui-${commit}.tar.gz" -C /tmp
code/examples/ratatui-examples/sync.sh "/tmp/ratatui-${commit}"
```

Then format and validate the workspace before publishing the change.
