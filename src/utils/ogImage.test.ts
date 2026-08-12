import dedent from "ts-dedent";
import { describe, expect, test } from "vitest";
import { getOgImage } from "./ogImage";

const site = new URL("https://ratatui.rs");
const filePath = "src/content/docs/highlights/v0.30.md";

const page = (body: string, data: Record<string, unknown> = {}) => ({
  body,
  filePath,
  site,
  data: { title: "v0.30.0", ...data },
});

describe("getOgImage", () => {
  test("falls back to the site image when the page has no images", async () => {
    const image = await getOgImage(page("Just some text."));
    expect(image.url).toBe("https://ratatui.rs/ratatui-og.png");
  });

  test("uses the first image in the page", async () => {
    const image = await getOgImage(
      page(dedent`
        ![animation](https://example.com/animation.gif)

        ![screenshot](https://example.com/screenshot.png)
      `),
    );
    expect(image).toEqual({ url: "https://example.com/animation.gif", alt: "animation" });
  });

  test("uses the page title when an image has no alt text", async () => {
    const image = await getOgImage(page("![](https://example.com/animation.gif)"));
    expect(image.alt).toBe("v0.30.0");
  });

  test("supports HTML images", async () => {
    const image = await getOgImage(
      page(`<img src="https://example.com/demo.png" alt="demo" width="100" />`),
    );
    expect(image).toEqual({ url: "https://example.com/demo.png", alt: "demo" });
  });

  test("ignores images inside code blocks", async () => {
    const image = await getOgImage(
      page(dedent`
        \`\`\`markdown
        ![example](https://example.com/example.png)
        \`\`\`

        ![real](https://example.com/real.png)
      `),
    );
    expect(image.url).toBe("https://example.com/real.png");
  });

  test("ignores images in a code block containing a nested fence", async () => {
    const image = await getOgImage(
      page(dedent`
        \`\`\`\`markdown
        \`\`\`rust
        // example
        \`\`\`
        ![example](https://example.com/example.png)
        \`\`\`\`

        ![real](https://example.com/real.png)
      `),
    );
    expect(image.url).toBe("https://example.com/real.png");
  });

  test("skips badges and SVGs, which don't render in link previews", async () => {
    const image = await getOgImage(
      page(dedent`
        ![crates.io](https://img.shields.io/crates/v/ratatui)
        ![logo](https://example.com/logo.svg)
        ![demo](https://example.com/demo.gif)
      `),
    );
    expect(image.url).toBe("https://example.com/demo.gif");
  });

  test("skips protocol-relative badges", async () => {
    const image = await getOgImage(
      page(dedent`
        ![crates.io](//img.shields.io/crates/v/ratatui)
        ![demo](//example.com/demo.gif)
      `),
    );
    expect(image.url).toBe("https://example.com/demo.gif");
  });

  test("falls back when a page-relative image isn't a bundled asset", async () => {
    const image = await getOgImage(page("![missing](./missing.png)"));
    expect(image.url).toBe("https://ratatui.rs/ratatui-og.png");
  });

  test("resolves images served from the site root", async () => {
    const image = await getOgImage(page("![csvlens](/csvlens.gif)"));
    expect(image.url).toBe("https://ratatui.rs/csvlens.gif");
  });

  test("resolves bundled images referenced relative to the page", async () => {
    const image = await getOgImage(page("![animation](../../../assets/ratatui-animation.gif)"));
    // The exact URL depends on how Astro emits the asset (hashed in builds, `/@fs/…` in dev).
    expect(image.url).toMatch(/^https:\/\/ratatui\.rs\/.*ratatui-animation.*\.gif/);
    expect(image.width).toBe(1280);
    expect(image.height).toBe(640);
  });

  test("prefers the ogImage frontmatter over images in the page", async () => {
    const image = await getOgImage(
      page("![animation](https://example.com/animation.gif)", {
        ogImage: "https://example.com/custom.png",
      }),
    );
    expect(image).toEqual({ url: "https://example.com/custom.png", alt: "v0.30.0" });
  });

  test("supports ogImage frontmatter pointing at a bundled image", async () => {
    const image = await getOgImage(
      page("Just some text.", {
        ogImage: { src: "/_astro/hero.hash.png", width: 1200, height: 630, format: "png" },
      }),
    );
    expect(image).toEqual({
      url: "https://ratatui.rs/_astro/hero.hash.png",
      width: 1200,
      height: 630,
      alt: "v0.30.0",
    });
  });
});
