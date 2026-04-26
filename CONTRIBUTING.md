# Contributing to Ratatui Website

Thanks for helping improve [Ratatui](https://ratatui.rs)'s website! 🐭

This repo contains the Astro/Starlight site plus a Rust workspace with the code samples under
[code/](code/).

## Setup

- [mise](https://mise.jdx.dev/) and Git LFS. `mise` is a tool version manager. In this repo it
  installs the right Node.js version and enables the JavaScript package manager used by the site.
- Rust (MSRV 1.86) (only if you change anything in [code/](code/)).

```sh
git clone https://github.com/ratatui/ratatui-website
cd ratatui-website
git lfs install
git lfs pull

mise run pnpm-install
mise run dev
```

`mise run pnpm-install` installs the website's JavaScript dependencies. `mise run dev` starts the
local Astro/Starlight development server and prints a local URL you can open in your browser.

If you are not using mise, use Node 24 and run the package manager directly:

```sh
corepack enable
pnpm install
pnpm dev
```

`corepack enable` makes the `pnpm` command available. The exact pnpm version is pinned in
`package.json`, so you do not need to install pnpm globally.

`pnpm install` triggers Playwright's browser download. You can skip the heavy Chromium bundle by
hitting Ctrl+C once that download starts if you do not plan to run the end-to-end tests.

## Commands

- Website (Astro/Starlight):

  - `mise run dev` for viewing locally
  - Format: `pnpm format` to rewrite files, or `pnpm format:check` to check formatting only
  - Type-check: `pnpm astro check`
  - Build: `mise run build` (runs `astro check`)
  - Tests: `mise run test`

- Code examples:

  - All snippets should live in `code/`, included using `{{ #include @code/... }}`
  - `cargo fmt --all --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`

## Writing content

- Markdown lint keeps lines to 100 chars; limited inline HTML is allowed for embeds.
- Prefer root-anchored links with a trailing slash (e.g. `/concepts/backends/`).

## Deployment

Cloudflare Pages should build the site with `pnpm build` and publish the `dist` directory. If the
Cloudflare project does not read `.node-version`, set its Node.js version to `24`.

## Assets

`.png`, `.gif`, `.svg`, `.webp` and `.xcf` files are tracked by Git LFS. Add new images with LFS to
avoid bloating the repo.

## Pull requests

Open an issue or start a discussion for non-trivial changes. Keep PRs focused with a short test
note.

## App showcase criteria

Before adding an app or third-party widget to the showcase, read:

- <https://ratatui.rs/recipes/apps/submitting-to-the-showcase/>
- <https://github.com/ratatui/ratatui-website/issues/new?template=showcase-submission.yml>
- <https://github.com/ratatui/ratatui-website/issues/986>

The website showcase is curated and intentionally selective. If your project is not a good fit for
the showcase yet, consider sharing it on Awesome Ratatui, the forum, or Discord first.

## License

By contributing, you agree that your contributions are licensed under the [MIT license](LICENSE).
