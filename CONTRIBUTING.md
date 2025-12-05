# Contributing to Ratatui Website

Thanks for helping improve [Ratatui](https://ratatui.rs)'s website! 🐭

This repo contains the Astro/Starlight site plus a Rust workspace with the code samples under
[code/](code/).

## Setup

- Node 24.x (lts), pnpm (recommended) or npm, and Git LFS.
- Rust (MSRV 1.86) (only if you change anything in [code/](code/)).

```sh
git clone https://github.com/ratatui/ratatui-website
cd ratatui-website
git lfs install && git lfs pull
pnpm install && pnpm dev
# or: npm install && npm run dev
```

> [!NOTE] `npm install` (and `pnpm install`) triggers Playwright's browser download; you can skip
> the heavy Chromium bundle by hitting Ctrl+C once that download starts if you do not plan to run
> the end-to-end tests.

## Commands

- Website (Astro/Starlight):

  - `pnpm dev` or `npm run dev` for viewing locally
  - Format: `pnpm format` / `npm run format -- --write` (write) or `npm run format -- --check` (note
    the double `--`)
  - Type-check: `pnpm astro check` or `npm run astro check`
  - Build: `pnpm build` or `npm run build` (runs `astro check`)
  - Tests: `pnpm test` or `npm test`

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

Open an issue or start a discussion for non-trivial changes. Keep PRs focused with a short test
note.

## App showcase criteria

See the current criteria and discussion in <https://github.com/ratatui/ratatui-website/issues/986>
before adding apps to the showcase.

## License

By contributing, you agree that your contributions are licensed under the [MIT license](LICENSE).
