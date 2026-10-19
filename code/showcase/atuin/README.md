# Atuin showcase

A short Ratatui website demo: type at the shell prompt, open history with Ctrl+R, narrow the query
to `cargo --v`, recall `cargo --version` with Tab, and execute it with Enter. History is synthetic;
the recalled command actually runs and prints the installed Cargo version without changing files or
using the network.

Run from the repository root with Atuin, Python 3, Bash, Cargo, and Betamax installed:

```sh
python3 code/showcase/atuin/prepare-showcase.py
betamax run code/showcase/atuin/atuin.tape
```

The recording was checked with Atuin 18.23.0, Betamax 0.1.22, and Cargo 1.99.0. Outputs are ignored
build artifacts in `target/showcase`: `atuin-run.gif`, `atuin-run.webp`, and screenshots of each
stage. The animation uses a 1200×720 canvas, an Atuin website palette, 22px text, and key overlays
below the terminal. `KeyboardOverlay Input` shows typing as well as shortcuts. The tape checks that
history and search results are visible during capture.

The preparation script recreates only `target/showcase/atuin`, imports an artificial zsh history,
and replaces imported directory and hostname metadata with demo values. Automatic sync, update
checks, and the daemon are disabled. The recording shell uses a minimal Ctrl+R Readline binding to
return the selected command to the prompt without enabling history capture hooks.

Regeneration uses relative timestamps, so their labels change with the recording time. After
reviewing the generated animation, update the website asset:

```sh
cp target/showcase/atuin-run.webp src/content/docs/showcase/apps/atuin.webp
```

The animated WebP is tracked with Git LFS alongside the other app showcase media. WebP preserves
smooth transparency around the border, captions, and shortcut overlays. The GIF is an optional local
output; only WebP is embedded. Synthetic databases, screenshots, and logs stay under the ignored
`target/` directory. This clip demonstrates local search, recall, and execution; it does not
demonstrate sync.

With jj, clean the asset into an LFS pointer before snapshotting; jj does not apply Git's clean
filter automatically:

```sh
git lfs clean < src/content/docs/showcase/apps/atuin.webp > target/showcase/atuin-lfs-pointer
cp target/showcase/atuin-lfs-pointer src/content/docs/showcase/apps/atuin.webp
jj status
```

Hydrate the image again for local preview with `git lfs smudge`, and use
`jj --ignore-working-copy diff` to inspect the saved change while images are hydrated.

The frame has a 6px transparent margin and no shadow. This gives the rounded green border room while
blending into the page background. Regenerate with a Betamax build that includes the WebP
transparency and color fixes; the original Betamax 0.1.22 recording predates those fixes.

The tape preserves the approved 1200×720 recording and its complete terminal palette. The original
Atuin source tape defaulted to 960×540; this copy uses the approved recording dimensions.

The palette follows [Atuin's website](https://atuin.sh/): midnight blue (`#101620` and `#0f1722`),
off-white (`#e8f0e9`), muted grey (`#8fa1a8`), and green (`#38c85a`, `#a8ecba`). `atuin-theme.toml`
applies those colors inside Atuin; the tape sets matching terminal, caption and key-overlay colors.
The outer frame uses the Ratatui website background described above.

ANSI blue (`#0088d1`) and bright blue (`#4ec2f8`) come from the
[brand turtle SVG](https://atuin.sh/assets/brand/atuin-turtle.svg). Red, gold, lavender, and teal
complete the 16-color palette with distinct normal and bright entries. The full terminal palette is
embedded in the tape's `Theme` setting; the Atuin theme also uses blue for timestamps and flags,
gold for strings, lavender for variables, and teal for operators.
