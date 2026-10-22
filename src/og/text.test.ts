import { describe, expect, test } from "vitest";
import { parseInlineCode, sectionTitle, wrap } from "./text";

describe("wrap", () => {
  test("breaks on spaces", () => {
    expect(wrap("Tui with Terminal and EventHandler", 13)).toEqual([
      "Tui with",
      "Terminal and",
      "EventHandler",
    ]);
  });

  test("breaks words longer than a line", () => {
    expect(wrap("Components/home.rs", 13)).toEqual(["Components/ho", "me.rs"]);
  });

  test("keeps short text on one line", () => {
    expect(wrap("Layout", 13)).toEqual(["Layout"]);
  });
});

describe("parseInlineCode", () => {
  test("strips backticks and marks code characters", () => {
    const { plain, isCode } = parseInlineCode("Use `eyre` here");
    expect(plain).toBe("Use eyre here");
    expect(isCode.map(Number).join("")).toBe("0000111100000");
  });
});

describe("sectionTitle", () => {
  const titles = new Map([
    ["recipes", "Recipes"],
    ["recipes/layout", "Layout"],
  ]);

  test("joins ancestor titles", () => {
    expect(sectionTitle("recipes/layout/grid", titles)).toBe("Recipes › Layout");
  });

  test("skips ancestors without a page", () => {
    expect(sectionTitle("recipes/missing/page", titles)).toBe("Recipes");
  });

  test("is empty for top-level pages", () => {
    expect(sectionTitle("faq", titles)).toBe("");
  });
});
