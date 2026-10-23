import { Resvg } from "@resvg/resvg-js";
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";
import satori from "satori";
import { parseInlineCode, wrap } from "./text";

export const OG_WIDTH = 1200;
export const OG_HEIGHT = 630;

export interface OgCard {
  title: string;
  description?: string;
  /** Breadcrumb shown in the border title, e.g. `Concepts`. */
  section?: string;
}

const colors = {
  background: "#0b1120",
  border: "#64748b",
  title: "#f8fafc",
  description: "#cbd5e1",
  muted: "#94a3b8",
  highlight: "#facc15",
  code: "#7dd3fc",
};

// Resolve from the project root: this module is bundled to a different location at build time.
const root = process.cwd();
const require = createRequire(join(root, "package.json"));

let assets: Promise<{ fonts: Parameters<typeof satori>[1]["fonts"]; logo: string }> | undefined;

function loadAssets() {
  assets ??= (async () => {
    const font = (weight: number) =>
      readFile(
        require.resolve(`@fontsource/fira-mono/files/fira-mono-latin-${weight}-normal.woff`),
      );
    const logo = await readFile(join(root, "src/assets/hero-dark.png"));
    return {
      fonts: [
        { name: "Fira Mono", data: await font(400), weight: 400, style: "normal" },
        { name: "Fira Mono", data: await font(700), weight: 700, style: "normal" },
      ],
      logo: `data:image/png;base64,${logo.toString("base64")}`,
    };
  })();
  return assets;
}

type Node = { type: string; props: Record<string, unknown> };

function h(type: string, style: Record<string, unknown>, ...children: unknown[]): Node {
  return { type, props: { style, children: children.length === 1 ? children[0] : children } };
}

// Lay everything out on a terminal-style character grid. One cell holds one 30px Fira Mono
// glyph (0.6em wide); the grid is centered in the card.
const cellWidth = 18;
const cellHeight = 36;
const fontSize = 30;
const cols = 64;
const rows = 16;
const originX = (OG_WIDTH - cols * cellWidth) / 2;
const originY = (OG_HEIGHT - rows * cellHeight) / 2;
const x = (col: number) => originX + col * cellWidth;
const y = (row: number) => originY + row * cellHeight;

// Box-drawing lines run through the middle of their cells, as terminals draw them.
const lineWidth = 3;
const lineX = (col: number) => x(col) + cellWidth / 2;
const lineY = (row: number) => y(row) + cellHeight / 2;

// The rat is 50x50 art pixels, so 400px draws each one as a crisp 8px square.
const logoSize = 400;
const logoCol = 2;
const textCol = 27;
// Leave two cells between the text and the right border.
const textCols = cols - 3 - textCol;
// The title uses double-size cells, like a terminal's double-width, double-height text.
const titleScale = 2;
const titleRow = 4;

function cardTree({ title, description, section }: OgCard, logo: string): Node {
  const at = (col: number, row: number, style: Record<string, unknown>, ...children: unknown[]) =>
    h(
      "div",
      { position: "absolute", left: x(col), top: y(row), display: "flex", ...style },
      ...children,
    );

  // A border title covers the line with a space on each side, like `─ ratatui.rs ─`.
  const borderTitle = (col: number, text: string, color: string) =>
    at(
      col,
      0,
      {
        width: (text.length + 2) * cellWidth,
        height: cellHeight,
        paddingLeft: cellWidth,
        backgroundColor: colors.background,
        color,
      },
      text,
    );
  const sectionText = section ?? "";

  // Each title line takes two rows; a blank row separates it from the description.
  const { plain: titleText, isCode } = parseInlineCode(title);
  const titleLines = wrap(titleText, Math.floor(textCols / titleScale) - 2);
  // Groups a title line into runs of plain text and inline code. `offset` is where the line starts
  // in the title; wrapping drops exactly one space between lines.
  const titleRuns = (line: string, offset: number) => {
    const runs: { text: string; code: boolean }[] = [];
    [...line].forEach((char, i) => {
      const code = isCode[offset + i];
      const last = runs[runs.length - 1];
      if (last && last.code === code) last.text += char;
      else runs.push({ text: char, code });
    });
    return runs;
  };
  const lineOffsets = titleLines.map((_, i) =>
    titleLines.slice(0, i).reduce((sum, line) => sum + line.length + 1, 0),
  );
  const descriptionRow = titleRow + titleLines.length * titleScale + 1;

  return h(
    "div",
    {
      position: "relative",
      display: "flex",
      width: OG_WIDTH,
      height: OG_HEIGHT,
      backgroundColor: colors.background,
      fontFamily: "Fira Mono",
      fontSize,
      lineHeight: `${cellHeight}px`,
    },
    h("div", {
      position: "absolute",
      left: lineX(0) - lineWidth / 2,
      top: lineY(0) - lineWidth / 2,
      width: lineX(cols - 1) - lineX(0) + lineWidth,
      height: lineY(rows - 1) - lineY(0) + lineWidth,
      border: `${lineWidth}px solid ${colors.border}`,
      // Match the curve of the rounded box-drawing corners `╭╮╰╯`.
      borderRadius: cellWidth / 2,
    }),
    borderTitle(1, "ratatui.rs", colors.muted),
    ...(section
      ? [borderTitle(cols - 3 - sectionText.length - 1, sectionText, colors.highlight)]
      : []),
    {
      type: "img",
      props: {
        src: logo,
        width: logoSize,
        height: logoSize,
        // Rest the rat on the bottom border.
        style: { position: "absolute", left: x(logoCol), top: lineY(rows - 1) - logoSize },
      },
    },
    ...titleLines.map((line, i) =>
      at(
        textCol,
        titleRow + i * titleScale,
        {
          fontSize: fontSize * titleScale,
          lineHeight: `${cellHeight * titleScale}px`,
          fontWeight: 700,
          color: colors.title,
        },
        // The prompt fills two cells on the first line; later lines indent to match.
        h(
          "div",
          { width: 2 * cellWidth * titleScale, flexShrink: 0, color: colors.highlight },
          i === 0 ? "$" : "",
        ),
        ...titleRuns(line, lineOffsets[i]).map(({ text, code }) =>
          // Satori trims spaces at the edges of each run, so use non-breaking spaces.
          h("div", { color: code ? colors.code : colors.title }, text.replaceAll(" ", "\u00a0")),
        ),
      ),
    ),
    ...(description
      ? wrap(description, textCols).map((line, i) =>
          at(textCol, descriptionRow + i, { color: colors.description }, line),
        )
      : []),
  );
}

export async function renderOgCard(card: OgCard): Promise<Buffer> {
  const { fonts, logo } = await loadAssets();
  const svg = await satori(cardTree(card, logo) as never, {
    width: OG_WIDTH,
    height: OG_HEIGHT,
    fonts,
  });
  // Satori already converts text to paths, so skip resvg's slow system font scan.
  return new Resvg(svg, { font: { loadSystemFonts: false } }).render().asPng();
}
