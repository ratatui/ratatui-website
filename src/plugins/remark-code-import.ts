import fs from "fs";
import path from "path";
import type { Node } from "unist";
import { visit } from "unist-util-visit";
import { rx } from "verbose-regexp"; // https://www.npmjs.com/package/verbose-regexp
import type { VFile } from "vfile";

interface CodeNode extends Node {
  lang?: string;
  value: string;
}

// Regular expression to match custom include directives in the format:
// {{#include filepath[:anchor]}}
//
// Example matches:
// - {{#include path/to/file}} - Matches "path/to/file" as the filepath.
// - {{#include path/to/file:anchorName}} - Matches "path/to/file" as the filepath and "anchorName" as the anchor.
// - {{#include ../../path/to/file}} - relative path to file from current markdown file
// - {{#include @path/to/file}} - path to file from root directory
const pathRegex = rx`(?<path>.+?)`; // Matches at least one character, but as few as possible
const anchorNameRegex = rx`(?<anchor>.*?)`; // Matches at least one character, but as few as possible
const includeRegex = rx.g`  // global (matches all instances in the file)
  \{\{                      // literal "{{"
  \s*                       // optional whitespace
  #include                  // literal "#include"
  \s*                       // optional whitespace
  ${pathRegex}              // filepath
  (:${anchorNameRegex})?  // optional anchor name
  \s*                       // optional whitespace after anchor
  \}\}                      // literal "}}"
`;

const remarkIncludeCode = () => {
  // The plugin function, working with a Markdown tree and associated file
  return (tree: Node, markdownFile: VFile) => {
    // Visit each 'code' node in the Markdown AST
    visit(tree, "code", (node: CodeNode) => {
      let match;
      // There can be multiple includes in a code block
      // Perform matches one by one, replacing text in AST along the way
      while ((match = includeRegex.exec(node.value)) !== null) {
        try {
          const includePath = match.groups?.path;
          const anchor = match.groups?.anchor;
          if (!includePath) {
            throw new Error("No file path specified");
          }
          // Inspired by astro aliases: https://docs.astro.build/en/guides/aliases/
          // Allows for specifying path from root directory
          let fullPath = includePath.startsWith("@")
            ? includePath.replace("@", "./")
            : path.resolve(path.dirname(markdownFile.path), includePath);
          let content = include(fullPath, anchor);
          // Replace the include directive with the file content
          node.value = node.value.replace(match[0], content);
        } catch (err) {
          if (err instanceof Error) {
            throw new Error(`Error including file: ${err.message}`);
          } else {
            throw new Error(`Error including file: ${err}`);
          }
        }
      }
    });
  };
};

export default remarkIncludeCode;

function include(includePath: string, anchor?: string): string {
  try {
    let fileContent = fs.readFileSync(includePath, "utf8");

    // if the anchor is in the format "start:end", extract the lines between the start and end
    if (anchor && anchor.search(":") != -1) {
      const [start, end] = anchor.split(":");
      const lines = fileContent.split("\n");
      const startIndex = start ? parseInt(start) - 1 : 0;
      const endIndex = end ? parseInt(end) : lines.length;
      fileContent = lines.slice(startIndex, endIndex).join("\n");
    } else if (anchor) {
      // If an anchor is specified, extract the relevant section from the file between
      // the start and end anchor comments. This is done using a
      const commentRegex = rx`
        \s*         // optional whitespace
        (/{2,}|#)   // two or more forward slashes or a #
      `;
      const startAnchorRegex = rx`
        ${commentRegex} // two or more forward slashes or a #
        \s*             // optional whitespace
        ANCHOR:         // the literal string "ANCHOR:" which precedes the anchor name
        \s*             // optional whitespace characters
        ${anchor}       // the anchor name
        \n{1,}          // one or more newline characters, accommodating variations in line breaks
      `;
      const endAnchorRegex = rx`
        \n{1,}          // one or more newline characters before the end anchor comment
        ${commentRegex} // two or more forward slashes or a #
        \s*             // optional whitespace
        ANCHOR_END:     // the literal string "ANCHOR_END:" which precedes the anchor name
        \s*             // optional whitespace characters
        ${anchor}       // the anchor name
      `;
      // The "m" flag (multiline) allows the regex to match across multiple lines.
      const anchorRegex = rx.m`   // multiline (matches across multiple lines)
        ${startAnchorRegex}       // matches the start anchor
        (?<content>\s*[\s\S]*?)   // a non-greedy match for any characters (including newlines), capturing the content
        ${endAnchorRegex}         // matches the end anchor
      `;
      const anchorContent = fileContent.match(anchorRegex)?.groups?.content;
      if (!anchorContent) throw new Error(`Anchor '${anchor}' not found in ${includePath}`);
      fileContent = anchorContent;
    }
    // Remove lines containing start and end anchor comments
    return fileContent
      .split("\n")
      .filter((line) => !line.includes("// ANCHOR: ") && !line.includes("// ANCHOR_END: "))
      .join("\n");
  } catch (err) {
    if (err instanceof Error) {
      throw new Error(`Error reading file '${includePath}': ${err.message}`);
    } else {
      throw new Error(`Error reading file '${includePath}': ${err}`);
    }
  }
}
