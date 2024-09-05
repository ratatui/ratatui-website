import fs from "fs";
import remarkParse from "remark-parse";
import remarkStringify from "remark-stringify";
import dedent from "ts-dedent";
import { unified } from "unified";
import { VFile } from "vfile";
import { beforeEach, describe, expect, test, vi } from "vitest";
import remarkIncludeCode from "./remark-code-import";

vi.mock("fs");

const start = "```markdown";
const end = "```\n";
const markdownFile = new VFile({ path: "/Users/ratatui/test.md" });

describe("remarkIncludeCode", () => {
  const processor = unified().use(remarkParse).use(remarkIncludeCode).use(remarkStringify);

  beforeEach(() => {
    vi.clearAllMocks();
  });

  test("should include file for raw markdown content", () => {
    const mockFileContent = "This is the content of the included file.";
    vi.spyOn(fs, "readFileSync").mockReturnValue(mockFileContent);

    const markdown = dedent`
      ${start}
      {{#include ./included-file.md}}
      ${end}`;
    const expected = dedent`
      ${start}
      This is the content of the included file.
      ${end}`;

    const result = processor.processSync(markdown).toString();
    expect(result).toBe(expected);
  });

  test("should include file content without anchor", () => {
    const mockFileContent = "This is the content of the included file.";
    vi.spyOn(fs, "readFileSync").mockReturnValue(mockFileContent);

    markdownFile.value = dedent`
      ${start}
      {{#include ./included-file.md}}
      ${end}`;
    const expected = dedent`
      ${start}
      This is the content of the included file.
      ${end}`;

    const result = processor.processSync(markdownFile).toString();
    expect(result).toBe(expected);
  });

  test("should include multiple includes in a code block", () => {
    const mockFileContent = "This is the content of the included file.";
    vi.spyOn(fs, "readFileSync").mockReturnValue(mockFileContent);

    markdownFile.value = dedent`
      ${start}
      {{#include ./included-file.md}}
      {{#include ./included-file.md}}
      ${end}`;
    const expected = dedent`
      ${start}
      This is the content of the included file.
      This is the content of the included file.
      ${end}`;

    const result = processor.processSync(markdownFile).toString();
    expect(result).toBe(expected);
  });

  test("should include file content with line range anchor", () => {
    const mockFileContent = dedent`
      Line 1
      Line 2
      Line 3
      Line 4
      Line 5`;
    vi.spyOn(fs, "readFileSync").mockReturnValue(mockFileContent);

    markdownFile.value = dedent`
      ${start}
      {{#include ./included-file.md:2:4}}
      ${end}`;
    const expected = dedent`
      ${start}
      Line 2
      Line 3
      Line 4
      ${end}`;

    const result = processor.processSync(markdownFile).toString();
    expect(result).toBe(expected);
  });

  test("should include file content with named anchor", () => {
    const mockFileContent = dedent`
      // ANCHOR: start
      This is the content of the included file.
      // ANCHOR_END: start`;
    vi.spyOn(fs, "readFileSync").mockReturnValue(mockFileContent);

    markdownFile.value = dedent`
      ${start}
      {{#include ./included-file.md:start}}
      ${end}`;
    const expected = dedent`
      ${start}
      This is the content of the included file.
      ${end}`;

    const result = processor.processSync(markdownFile).toString();
    expect(result).toBe(expected);
  });

  test("should include file content with function anchor", () => {
    const mockFileContent = dedent`
      fn exampleFunction() {
        println("foo");
      }`;
    vi.spyOn(fs, "readFileSync").mockReturnValue(mockFileContent);

    markdownFile.value = dedent`
      ${start}
      {{#include ./included-file.md:exampleFunction()}}
      ${end}`;
    const expected = dedent`
      ${start}
      fn exampleFunction() {
        println("foo");
      }
      ${end}`;

    const result = processor.processSync(markdownFile).toString();
    expect(result).toBe(expected);
  });

  test("should throw error for file path", () => {
    vi.spyOn(fs, "readFileSync").mockImplementation(() => {
      throw new Error("File not found");
    });

    markdownFile.value = dedent`
      ${start}
      {{#include ./invalid-file.md}}
      ${end}`;

    expect(() => processor.processSync(markdownFile)).toThrow(
      "Unable to process includes for /Users/ratatui/test.md. Unable to include file '/Users/ratatui/invalid-file.md'. File not found",
    );
  });

  test("should throw error for missing anchor", () => {
    const mockFileContent = "This is the content of the included file.";
    vi.spyOn(fs, "readFileSync").mockReturnValue(mockFileContent);

    markdownFile.value = dedent`
      ${start}
      {{#include ./included-file.md:missingAnchor}}
      ${end}`;

    expect(() => processor.processSync(markdownFile)).toThrow(
      "Unable to process includes for /Users/ratatui/test.md. Unable to include file '/Users/ratatui/included-file.md'. Anchor 'missingAnchor' not found",
    );
  });

  test("should include file content with root path alias", () => {
    const mockFileContent = "This is the content of the included file.";
    vi.spyOn(fs, "readFileSync").mockReturnValue(mockFileContent);

    markdownFile.value = dedent`
      ${start}
      {{#include @/included-file.md}}
      ${end}`;
    const expected = dedent`
      ${start}
      This is the content of the included file.
      ${end}`;

    const result = processor.processSync(markdownFile).toString();
    expect(result).toBe(expected);
  });
});
