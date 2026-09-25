import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import remarkParse from "remark-parse";
import { unified } from "unified";
import { visit } from "unist-util-visit";
import { afterEach, beforeEach, describe, expect, test } from "vitest";
import { expandIncludes } from "./expand-includes";

describe("Markdown code exports", () => {
  let directory: string;
  let filePath: string;

  beforeEach(() => {
    directory = fs.mkdtempSync(path.join(os.tmpdir(), "markdown-export-"));
    filePath = path.join(directory, "page.md");
    fs.writeFileSync(path.join(directory, "example.rs"), "fn main() {\n    example();\n}");
  });

  afterEach(() => fs.rmSync(directory, { recursive: true, force: true }));

  test.each(["{{#include ./example.rs}}", "{{ #include ./example.rs }}"])(
    "expands %s while preserving MDX and metadata",
    (directive) => {
      const before = 'import Demo from "./Demo.astro";\n\n<Demo />\n\n';
      const source = `${before}\`\`\`rust title="example.rs"\n${directive}\n\`\`\`\n\nAfter.\n`;
      expect(expandIncludes(source, filePath)).toBe(
        source.replace(directive, "fn main() {\n    example();\n}"),
      );
    },
  );

  test.each([
    ["1. Step\n\n    ", "    "],
    ["1. ", "   "],
    ["> ", "> "],
    ["> 1. Step\n>\n>    ", ">    "],
  ])("preserves container prefixes: %j", (opening, continuation) => {
    const source = `${opening}\`\`\`rust\n${continuation}{{ #include ./example.rs }}\n${continuation}\`\`\`\n\nAfter.\n`;
    const result = expandIncludes(source, filePath);
    expect(result).toBe(
      source.replace(
        "{{ #include ./example.rs }}",
        `fn main() {\n${continuation}    example();\n${continuation}}`,
      ),
    );
    const tree = unified().use(remarkParse).parse(result);
    const code: string[] = [];
    visit(tree, "code", (node) => {
      code.push(node.value);
    });
    expect(code).toEqual(["fn main() {\n    example();\n}"]);
    expect(tree.children.at(-1)).toMatchObject({
      type: "paragraph",
      children: [{ value: "After." }],
    });
  });

  test.each(["```", "~~~"])("widens %s fences around included fences", (fence) => {
    fs.writeFileSync(path.join(directory, "example.rs"), `before\n${fence}\nafter`);
    const source = `${fence}rust\n{{#include ./example.rs}}\n${fence}`;
    const result = expandIncludes(source, filePath);
    const tree = unified().use(remarkParse).parse(result);
    expect(tree.children).toHaveLength(1);
    expect(tree.children[0]).toMatchObject({ type: "code", value: `before\n${fence}\nafter` });
  });

  test("preserves ignored directives and unchanged code verbatim", () => {
    const source =
      "~~~markdown include=ignore\n{{#include ./missing.rs}}\n~~~\n\n```rust\nlet x = 1;\n```";
    expect(expandIncludes(source, filePath)).toBe(source);
  });

  test("fails when an include cannot be expanded", () => {
    expect(() => expandIncludes("```rust\n{{#include ./missing.rs}}\n```", filePath)).toThrow(
      "Unable to include file",
    );
  });

  test("expands indented code", () => {
    expect(expandIncludes("    {{#include ./example.rs}}\n", filePath)).toBe(
      "    fn main() {\n        example();\n    }\n",
    );
  });

  test.each([
    ["1. Step\n\n\t\t", " fn main() {\n    example();\n}"],
    [">\t\t", "  fn main() {\n    example();\n}"],
    ["\t ", " fn main() {\n    example();\n}"],
    ["1. Step\n\n\t\t\t", " \tfn main() {\n    example();\n}"],
    ["> 1. Step\n>\n> \t\t\t", "   fn main() {\n    example();\n}"],
  ])("preserves code values with tab indentation: %j", (prefix, expected) => {
    const source = `${prefix}{{#include ./example.rs}}\n`;
    const result = expandIncludes(source, filePath);
    const values: string[] = [];
    visit(unified().use(remarkParse).parse(result), "code", (node) => {
      values.push(node.value);
    });
    expect(values).toEqual([expected]);
  });

  test.each(["grid", "collapse-borders"])("exports the existing %s recipe", (recipe) => {
    const recipePath = `src/content/docs/recipes/layout/${recipe}.md`;
    const source = fs.readFileSync(recipePath, "utf8");
    const result = expandIncludes(source, recipePath);
    expect(result).not.toMatch(/\{\{\s*#include/);
    const blocks = (markdown: string) => {
      const code: { lang: string | null | undefined; value: string }[] = [];
      visit(unified().use(remarkParse).parse(markdown), "code", (node) => {
        code.push({ lang: node.lang, value: node.value });
      });
      return code;
    };
    expect(blocks(result).map((node) => node.lang)).toEqual(
      blocks(source).map((node) => node.lang),
    );
    expect(blocks(result).every((node) => node.value.trim().length > 0)).toBe(true);
  });
});
