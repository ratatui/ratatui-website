# Ratatui Book

The [ratatui-book](https://github.com/ratatui-org/ratatui-book) is written in
[`mdbook`](https://rust-lang.github.io/mdBook/).

The book is built as HTML pages as part of a
[GitHub Action](https://github.com/ratatui-org/ratatui-book/blob/main/.github/workflows/mdbook.yml)
and is available to view at <https://ratatui-org.github.io/ratatui-book/>.

Feel free to make contributions if you'd like to improve the documentation.

If you want to set up your local environment, you can run the following:

```bash
cargo install mdbook --version 0.4.35
cargo install mdbook-admonish --version 1.13.0
cargo install mdbook-svgbob2 --version 0.3.0
cargo install mdbook-linkcheck --version 0.7.7
cargo install mdbook-mermaid --version 0.12.6
cargo install mdbook-emojicodes --version 0.2.2
```

These plugins allow additional features.

## `mdbook-admonish`

The following raw markdown:

````markdown
```admonish note
This is a note
```

```admonish tip
This is a tip
```

```admonish warning
This is a warning
```

```admonish info
This is a info
```
````

will render as the following:

```admonish note
This is a note
```

```admonish tip
This is a tip
```

```admonish warning
This is a warning
```

```admonish info
This is a info
```

## `mdbook-mermaid`

The following raw markdown:

````markdown
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```
````

will render as the following:

```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```

## `mdbook-svgbob2`

The following raw markdown:

````markdown
```svgbob
       .---.
      /-o-/--
   .-/ / /->
  ( *  \/
   '-.  \
      \ /
       '
```
````

will render as the following:

```svgbob
       .---.
      /-o-/--
   .-/ / /->
  ( *  \/
   '-.  \
      \ /
       '
```

## `mdbook-emojicodes`

The following raw markdown:

```markdown
I love cats :cat: and dogs :dog:, I have two, one's gray, like a raccoon :raccoon:, and the other
one is black, like the night :night_with_stars:.
```

will render as the following:

I love cats :cat: and dogs :dog:, I have two, one's gray, like a raccoon :raccoon:, and the other
one is black, like the night :night_with_stars:.
