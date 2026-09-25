import type { Code } from "mdast";
import path from "node:path";
import remarkParse from "remark-parse";
import { unified } from "unified";
import { visit } from "unist-util-visit";
import { VFile } from "vfile";
import remarkIncludeCode from "../plugins/remark-code-import";

/** Expand code includes without reserializing surrounding Markdown or MDX. */
export function expandIncludes(source: string, filePath: string | undefined): string {
  const file = new VFile({
    value: source,
    path: filePath ? path.resolve(filePath) : undefined,
  });
  const processor = unified().use(remarkParse).use(remarkIncludeCode);
  const tree = processor.parse(file);
  const blocks: { node: Code; value: string }[] = [];
  visit(tree, "code", (node) => {
    blocks.push({ node, value: node.value });
  });

  // Let the same plugin used by the site decide which directives to expand.
  // Errors must fail the build rather than publish an incomplete export.
  processor.runSync(tree, file);

  let result = source;
  // Replace only changed blocks, from the end so source offsets remain valid.
  for (const { node, value } of blocks.reverse()) {
    if (node.value === value) continue;
    const start = node.position!.start.offset!;
    const end = node.position!.end.offset!;
    const original = source.slice(start, end);
    const lineStart = source.lastIndexOf("\n", start - 1) + 1;
    // A list marker on the opening line becomes indentation on following lines.
    const prefix = source
      .slice(lineStart, start)
      .replace(/(?:[*+-]|\d+[.)])([ \t]+)/g, (marker) => " ".repeat(marker.length));
    const replacement = renderCode(node.value, original, value, prefix).replace(
      /\n/g,
      `\n${prefix}`,
    );
    result = result.slice(0, start) + replacement + result.slice(end);
  }
  return result;
}

/** Keep fence metadata and widen fences if included code contains the marker. */
function renderCode(
  value: string,
  original: string,
  previousValue: string,
  prefix: string,
): string {
  const openingFence = original.match(/^(`{3,}|~{3,})([^\n]*)/);
  if (openingFence) {
    const marker = openingFence[1][0];
    const runs = value.match(marker === "`" ? /`+/g : /~+/g) ?? [];
    const length = Math.max(openingFence[1].length, ...runs.map((run) => run.length + 1));
    const fence = marker.repeat(length);
    return `${fence}${openingFence[2]}\n${value}\n${fence}`;
  }
  // Tabs may straddle the container/code boundary. Keep only the columns
  // removed by the parser; previousValue already contains any remaining spaces.
  let sourceIndent = original.match(/^[ \t]*/)![0];
  let contentIndent = previousValue.match(/^[ \t]*/)![0];
  // Ignore whitespace retained verbatim as code, including any literal tabs.
  while (sourceIndent && contentIndent && sourceIndent.at(-1) === contentIndent.at(-1)) {
    sourceIndent = sourceIndent.slice(0, -1);
    contentIndent = contentIndent.slice(0, -1);
  }
  const prefixWidth = columnWidth(prefix);
  const indentWidth = columnWidth(prefix + sourceIndent) - prefixWidth - columnWidth(contentIndent);
  const indent = " ".repeat(indentWidth);
  return value
    .split("\n")
    .map((line) => indent + line)
    .join("\n");
}

/** Markdown advances tabs to the next multiple of four columns. */
function columnWidth(text: string): number {
  let column = 0;
  for (const character of text) {
    column += character === "\t" ? 4 - (column % 4) : 1;
  }
  return column;
}
