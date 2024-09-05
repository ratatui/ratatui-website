import fs from "fs";
import path from "path";
import Parser, { Query } from "tree-sitter";
import Rust from "tree-sitter-rust";
import type { Node } from "unist";
import { visit } from "unist-util-visit";
import { rx } from "verbose-regexp"; // https://www.npmjs.com/package/verbose-regexp
import type { VFile } from "vfile";

export default remarkIncludeCode;

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
const includeRegex = rx`    // global (matches all instances in the file)
  \{\{                      // literal "{{"
  \s*                       // optional whitespace
  #include                  // literal "#include"
  \s*                       // optional whitespace
  ${pathRegex}              // filepath
  (:${anchorNameRegex})?    // optional anchor name
  \s*                       // optional whitespace after anchor
  \}\}                      // literal "}}"
`;

interface MetaNode extends CodeNode {
  meta?: string;
}

function remarkIncludeCode() {
  // The plugin function, working with a Markdown tree and associated file
  return (tree: Node, markdownFile: VFile) => {
    // Visit each 'code' node in the Markdown AST
    visit(tree, "code", (node: MetaNode) => visitNode(node, markdownFile));
  };
}

function visitNode(node: MetaNode, markdownFile: VFile) {
  if (node.lang === "markdown" && node.meta == "include=ignore") {
    return;
  }
  let match;
  // There can be multiple includes in a code block
  // Perform matches one by one, replacing text in AST along the way
  while ((match = includeRegex.exec(node.value)) !== null) {
    try {
      const includePath = match.groups!.path;
      const anchor = match.groups!.anchor;

      let fullPath = relativePath(markdownFile, includePath);
      let content = include(fullPath, anchor);
      // Replace the include directive with the file content
      node.value = node.value.replace(match[0], content);
    } catch (err) {
      let message = (err as Error).message;
      let file = markdownFile.path ? markdownFile.path : "unknown file";
      throw new Error(`Unable to process includes for ${file}. ${message}`);
    }
  }
}

// Inspired by astro aliases: https://docs.astro.build/en/guides/aliases/
// Allows for specifying path from root directory using @
function relativePath(markdownFile: VFile, includePath: string): string {
  if (includePath.startsWith("@")) {
    return includePath.replace("@", "./");
  }
  let root = markdownFile.path ? path.dirname(markdownFile.path) : "";
  return path.resolve(root, includePath);
}

function include(includePath: string, anchor?: string): string {
  try {
    let fileContent = fs.readFileSync(includePath, "utf8");

    if (anchor) {
      if (anchor.search(":") != -1) {
        // if the anchor is in the format "start:end", extract the lines between the start and end
        const [start, end] = anchor.split(":");
        fileContent = extractLines(fileContent, start, end);
      } else if (anchor.endsWith("()")) {
        // If an anchor is specified with a function call, extract the relevant section from the file between
        // the start of the function and the end of the function.
        const name = anchor.slice(0, -2);
        fileContent = extractFunction(fileContent, name);
      } else {
        // If an anchor is specified, extract the relevant section from the file between
        // the start and end anchor comments. This is done using a
        fileContent = extractBetweenComments(fileContent, anchor);
      }
    }
    // Remove lines containing start and end anchor comments
    return fileContent
      .split("\n")
      .filter((line) => !line.includes("// ANCHOR: ") && !line.includes("// ANCHOR_END: "))
      .join("\n");
  } catch (err) {
    let message = (err as Error).message;
    throw new Error(`Unable to include file '${includePath}'. ${message}`);
  }
}

function extractLines(content: string, start: string, end: string) {
  const lines = content.split("\n");
  const startIndex = start ? parseInt(start) - 1 : 0;
  const endIndex = end ? parseInt(end) : lines.length;
  content = lines.slice(startIndex, endIndex).join("\n");
  return content;
}

function extractFunction(content: string, name: string) {
  const parser = new Parser();
  parser.setLanguage(Rust);
  const tree = parser.parse(content);
  let query = new Query(
    Rust,
    `
    (
      (line_comment (doc_comment))+?
      .
      (attribute_item)+?
      .
      (function_item name: (identifier) @name (#eq? @name "${name}"))
    ) @fn
    `,
  );
  const captures = query.captures(tree.rootNode);
  if (captures.length == 0) {
    throw new Error(`Function '${name}' not found`);
  }
  return captures
    .map((capture) => {
      if (capture.name !== "fn") {
        return "";
      }
      let indent = capture.node.startPosition.column;
      if (capture.node.type === "line_comment") {
        // line comments include the newline character in their range
        return content.slice(capture.node.startIndex - indent - 1, capture.node.endIndex - 1);
      }
      return content.slice(capture.node.startIndex - indent, capture.node.endIndex);
    })
    .join("\n")
    .trimEnd();
}

function extractBetweenComments(content: string, name: string) {
  const commentRegex = rx`
        \s*         // optional whitespace
        (/{2,}|#)   // two or more forward slashes or a #
      `;
  const startAnchorRegex = rx`
        ${commentRegex} // two or more forward slashes or a #
        \s*             // optional whitespace
        ANCHOR:         // the literal string "ANCHOR:" which precedes the anchor name
        \s*             // optional whitespace characters
        ${name}       // the anchor name
        \n{1,}          // one or more newline characters, accommodating variations in line breaks
      `;
  const endAnchorRegex = rx`
        \n{1,}          // one or more newline characters before the end anchor comment
        ${commentRegex} // two or more forward slashes or a #
        \s*             // optional whitespace
        ANCHOR_END:     // the literal string "ANCHOR_END:" which precedes the anchor name
        \s*             // optional whitespace characters
        ${name}       // the anchor name
      `;
  // The "m" flag (multiline) allows the regex to match across multiple lines.
  const anchorRegex = rx.m`   // multiline (matches across multiple lines)
        ${startAnchorRegex}       // matches the start anchor
        (?<content>\s*[\s\S]*?)   // a non-greedy match for any characters (including newlines), capturing the content
        ${endAnchorRegex}         // matches the end anchor
      `;
  const anchorContent = content.match(anchorRegex)?.groups?.content;
  if (!anchorContent) throw new Error(`Anchor '${name}' not found`);
  content = anchorContent;
  return content;
}
