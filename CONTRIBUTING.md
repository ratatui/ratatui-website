# Contributing to Ratatui Website

Thanks for helping improve [Ratatui](https://ratatui.rs)'s website! 🐭

This repo contains the Astro/Starlight site plus a Rust workspace with the code samples under [code/](code/).

## Setup

- Node 22.x, NPM, and Git LFS.
- Rust (MSRV 1.74) (only if you change anything in [code/](code/)).

```sh
git clone https://github.com/ratatui/ratatui-website
cd ratatui-website
git lfs install && git lfs pull
npm install && npm run dev
```

## Commands

- Website (Astro/Starlight): 
  - `npm run dev` for viewing locally
  - `npm run format`
  - `npm run astro check`
  - `npm run build` (runs `astro check`)
  - `npm test`

- Code examples:
  - All snippets should live in `code/`, included using `{{ #include @code/... }}`
  - `cargo fmt --all --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`

## Writing content

- Markdown lint keeps lines to 100 chars; limited inline HTML is allowed for embeds.
- Prefer root-anchored links with a trailing slash (e.g. `/concepts/backends/`).

## Assets

`.png` and `.gif` files are tracked by Git LFS. Add new images with LFS to avoid bloating the repo.

## Pull requests

Open an issue or start a discussion for non-trivial changes. Keep PRs focused with a short test note.

## App showcase criteria

See the current criteria and discussion in <https://github.com/ratatui/ratatui-website/issues/986>
before adding apps to the showcase.

## License

By contributing, you agree that your contributions are licensed under the [MIT license](LICENSE).
