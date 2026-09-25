# Repository Guidelines

## Project Structure & Module Organization

This repository builds ratatui.rs with Astro and Starlight. Documentation lives in
`src/content/docs/`; reusable UI in `src/components/`; content transforms in `src/plugins/`; and
browser scripts and helpers in `src/scripts/` and `src/utils/`. Assets live in `src/assets/`,
`public/`, and content directories. The `code/` Rust workspace contains examples, recipes,
tutorials, templates, and showcase programs. Generated output belongs in `dist/`.

## Build, Test, and Development Commands

Use Node.js 26 and the pnpm version pinned in `package.json`; `.mise.toml` configures both.

- `mise run pnpm-install`: install website dependencies.
- `pnpm dev`: start the Astro development server.
- `pnpm build`: run Astro checks and build the site.
- `pnpm preview`: serve the built site through Wrangler locally.
- `pnpm test --run`: run Vitest once; `pnpm test` enables watch mode.
- `pnpm format:check`: check Prettier formatting; `pnpm format` rewrites files.

## Coding Style & Naming Conventions

Follow Prettier: two-space indentation, double quotes, trailing commas, and 100-character lines.
Wrap Markdown prose at 100 characters and lint changed Markdown with `markdownlint-cli2`. Use
descriptive kebab-case content filenames, PascalCase Astro components, and root-relative internal
links with trailing slashes, such as `/showcase/apps/`. Keep executable Rust snippets in `code/` and
include them using `{{ #include @code/... }}`. Track images through Git LFS.

## Testing Guidelines

Colocate Vitest tests as `*.test.ts` beside implementation files. Add focused tests for changed
logic; no coverage threshold is configured. For content edits, check links and inspect affected
pages. For Rust changes, run `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
and `cargo test` from the repository root.

## Commit & Pull Request Guidelines

Keep changes focused. Prefer imperative Conventional Commit subjects, such as `docs: clarify setup`
or `fix: repair navigation`. Explain the change, link relevant issues, and report validation in PR
descriptions. Read `CONTRIBUTING.md`; showcase submissions require a linked submission issue.

### Branch Preview Links

When writing or updating PR bodies, include clickable Markdown links to the preview root and
specific changed pages. Infer URLs without waiting for deployment or asking the user:

- Pattern: `https://<branch-slug>-ratatui-website.cloudflare-ratatui.workers.dev/`.
- Guess the slug by lowercasing the head branch and replacing non-alphanumeric characters with `-`.
  For example, `joshka/showcase-submission-link` becomes `joshka-showcase-submission-link`.
- Map `src/content/docs/showcase/apps.md` to `/showcase/apps/` and `showcase/index.mdx` to
  `/showcase/`; append these paths to the preview root.
- Label unverified URLs as inferred preview links. Correct guesses when deployment output provides
  actual URLs; do not claim a preview was tested unless it was.
