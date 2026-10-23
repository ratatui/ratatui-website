/**
 * Wraps `text` to `width` characters, breaking on spaces. Words longer than a line break mid-word,
 * as they would in a terminal.
 */
export function wrap(text: string, width: number) {
  const lines: string[] = [];
  let line = "";
  const words = text
    .split(" ")
    .flatMap((word) => word.match(new RegExp(`.{1,${width}}`, "g")) ?? [word]);
  for (const word of words) {
    if (line && line.length + 1 + word.length > width) {
      lines.push(line);
      line = word;
    } else {
      line = line ? `${line} ${word}` : word;
    }
  }
  return [...lines, line];
}

/** Splits Markdown inline code out of `text`, returning the plain text and which characters are code. */
export function parseInlineCode(text: string) {
  let plain = "";
  const isCode: boolean[] = [];
  text.split("`").forEach((part, i) => {
    plain += part;
    isCode.push(...Array<boolean>(part.length).fill(i % 2 === 1));
  });
  return { plain, isCode };
}

/**
 * Builds the breadcrumb for a docs entry from the titles of its ancestor pages, e.g.
 * `Recipes › Layout` for `recipes/layout/grid`. Ancestors without a page are skipped.
 */
export function sectionTitle(id: string, titles: Map<string, string>) {
  const segments = id.split("/");
  return segments
    .slice(0, -1)
    .map((_, i) => titles.get(segments.slice(0, i + 1).join("/")))
    .filter((title) => title !== undefined)
    .join(" › ");
}
