// Serves the raw Markdown source of every docs page at `<page path>.md`, e.g.
// `/installation/` is also available as `/installation.md`. This is what the
// "Copy page" button on each page copies, and what the "Open in ChatGPT" /
// "Open in Claude" links point at, so that an LLM can read the page without
// having to scrape the rendered HTML.
//
// `{{#include ...}}` directives are expanded (using the same remark plugin the
// site build uses) so the served Markdown contains real code, not directives.
import type { APIRoute, GetStaticPaths } from "astro";
import { getCollection, type CollectionEntry } from "astro:content";
import path from "node:path";
import remarkParse from "remark-parse";
import { unified } from "unified";
import { visit } from "unist-util-visit";
import { VFile } from "vfile";
import remarkIncludeCode from "~/plugins/remark-code-import";

interface Props {
  entry: CollectionEntry<"docs">;
}

export const getStaticPaths: GetStaticPaths = async () => {
  const docs = await getCollection("docs");
  return (
    docs
      // The site root is a splash page with no prose worth serving.
      .filter((entry) => entry.id !== "")
      .map((entry) => ({ params: { slug: entry.id }, props: { entry } }))
  );
};

export const GET: APIRoute<Props> = ({ props }) => {
  const { entry } = props;
  const title = entry.data.title;
  const description = entry.data.description;
  const metadata = [
    "---",
    `title: ${JSON.stringify(title)}`,
    ...(description ? [`description: ${JSON.stringify(description)}`] : []),
    "---",
  ].join("\n");
  const documentHeader = [`# ${title}`, description].filter(Boolean).join("\n\n");
  const body = expandIncludes(entry.body ?? "", entry.filePath);
  return new Response(`${metadata}\n\n${documentHeader}\n\n${body}`, {
    headers: { "Content-Type": "text/markdown; charset=utf-8" },
  });
};

/**
 * Expands `{{#include ...}}` directives inside fenced code blocks.
 *
 * The Markdown is parsed only to locate the code blocks; the replacement is
 * done by splicing the original source so that every other byte of the
 * document (directives, MDX, formatting) is preserved verbatim.
 */
function expandIncludes(source: string, filePath: string | undefined): string {
  try {
    return expandIncludesUnsafe(source, filePath);
  } catch (error) {
    // A broken include should not take a page's Markdown down with it: serve
    // the unexpanded source instead. The site build reports the error anyway.
    console.warn(`Could not expand includes for ${filePath ?? "unknown file"}: ${error}`);
    return source;
  }
}

function expandIncludesUnsafe(source: string, filePath: string | undefined): string {
  const file = new VFile({
    value: source,
    // The include plugin resolves relative paths against the Markdown file.
    path: filePath ? path.resolve(process.cwd(), filePath) : undefined,
  });
  const processor = unified().use(remarkParse).use(remarkIncludeCode);
  const tree = processor.runSync(processor.parse(file), file);

  const blocks: CodeBlock[] = [];
  visit(tree, "code", (node: CodeNode) => {
    const start = node.position?.start.offset;
    const end = node.position?.end.offset;
    if (start === undefined || end === undefined) return;
    blocks.push({ lang: node.lang, meta: node.meta, value: node.value, start, end });
  });

  let result = source;
  // Splice from the end so earlier offsets stay valid.
  for (const block of blocks.reverse()) {
    if (!source.slice(block.start, block.end).includes("{{#include")) continue;
    const fence = "`".repeat(Math.max(3, longestBacktickRun(block.value) + 1));
    const info = [block.lang, block.meta].filter(Boolean).join(" ");
    result =
      result.slice(0, block.start) +
      `${fence}${info}\n${block.value}\n${fence}` +
      result.slice(block.end);
  }
  return result;
}

interface CodeNode {
  lang?: string | null;
  meta?: string | null;
  value: string;
  position?: { start: { offset?: number }; end: { offset?: number } };
}

interface CodeBlock {
  lang?: string | null;
  meta?: string | null;
  value: string;
  start: number;
  end: number;
}

function longestBacktickRun(value: string): number {
  return Math.max(0, ...[...value.matchAll(/`+/g)].map((match) => match[0].length));
}
