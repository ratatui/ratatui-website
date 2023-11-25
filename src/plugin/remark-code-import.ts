import type { Node } from "unist";
import type { VFile } from "vfile";
import { visit } from "unist-util-visit";
import fs from "fs";
import path from "path";

interface CodeNode extends Node {
  lang?: string;
  value: string;
}

const remarkIncludeCode = () => {
  // The plugin function, working with a Markdown tree and associated file
  return (tree: Node, file: VFile) => {
    // Visit each 'code' node in the Markdown AST
    visit(tree, "code", (node: CodeNode) => {
      // Process only if the language is Rust
      if (node.lang === "rust") {
        // Regular expression to match custom include directives in the format:
        // {{#include filepath[:anchor]}}
        //
        // Explanation of the regex pattern:
        // /\{\{#include (.*?)(?::(.*?))?\}\}/g
        //
        // \{ - Escapes the '{' character because it has a special meaning in regex.
        //     We need two \{ to represent the two '{' in our target string.
        // #include - Matches the literal string "#include", which is part of our directive.
        // (.*?) - A non-greedy match for any characters. This is the first capturing group
        //         and it captures the filepath. The non-greedy `.*?` ensures that it captures
        //         the minimum number of characters until it reaches the next part of the pattern.
        //         It stops at the first colon (:) it encounters, or the end of the curly braces
        //         if there is no colon.
        // (?: - Starts a non-capturing group for the optional anchor part. The non-capturing
        //      group is used here because we don't need to capture the ':' itself, just the part after it.
        // : - Matches the literal colon character. This separates the filepath and the optional anchor.
        // (.*?) - Another non-greedy match for any characters. This is the second capturing group
        //         and it captures the anchor. As it's non-greedy, it stops at the first '}' it encounters.
        // )? - The entire non-capturing group is made optional by the '?', which means this part of the
        //      pattern (the colon and the anchor) might not be present.
        // \} - Escapes the '}' character. Like the '{', we need two \} to represent the two '}' in our target string.
        // /g - Global flag, meaning it will match all occurrences in the string, not just the first one.
        //
        // Example matches:
        // "{{#include path/to/file}}" - Matches and captures "path/to/file" as the filepath.
        // "{{#include path/to/file:anchorName}}" - Matches and captures "path/to/file" as the filepath
        //                                          and "anchorName" as the anchor.
        const includeRegex = /\{\{#include (.*?)(?::(.*?))?\}\}/g;
        let match;
        while ((match = includeRegex.exec(node.value)) !== null) {
          const filePath = match[1];
          const anchor = match[2];
          const fullPath = path.resolve(path.dirname(file.path), filePath);
          try {
            let fileContent = fs.readFileSync(fullPath, "utf8");
            // If an anchor is specified, extract the relevant section from the file
            if (anchor) {
              // Regular expression to match a specific anchored section in the file content.
              //
              // Explanation of the regex pattern:
              // `// ANCHOR: ${anchor}(\\s*[\\s\\S]*?)// ANCHOR_END: ${anchor}`
              //
              // // ANCHOR: ${anchor} - Matches the start anchor comment in the file.
              //                       This part of the regex dynamically inserts the value of
              //                       the `anchor` variable, allowing it to match a specific
              //                       anchor name. For example, if `anchor` is "setup", it
              //                       will match "// ANCHOR: setup".
              //
              // (\\s*[\\s\\S]*?) - This is a capturing group that matches the content between
              //                    the start and end anchors:
              //   \\s*           - Matches any whitespace characters (including none), ensuring
              //                    that the capture starts even if there is space after the
              //                    "// ANCHOR: ${anchor}" comment.
              //   [\\s\\S]*?     - A non-greedy match for any character (including newlines):
              //     [\\s\\S]     - Matches any character. \\s is any whitespace character, and
              //                    \\S is any non-whitespace character. Together, [\\s\\S] is
              //                    a character class that matches absolutely any character.
              //     *?           - The non-greedy quantifier, ensuring it captures the smallest
              //                    amount of content until it reaches the end anchor.
              //
              // // ANCHOR_END: ${anchor} - Matches the end anchor comment in the file, similar
              //                           to the start anchor. This part of the regex also
              //                           dynamically includes the `anchor` variable, ensuring
              //                           it matches the corresponding end anchor for the
              //                           specified start anchor.
              //
              // "m" - Multiline flag, allowing '^' and '$' to match start and end of lines, not
              //       just start and end of string. This is important for matching anchors that
              //       are not at the very start or end of the file content.
              //
              // This regular expression is used to extract a section of text from a file that
              // is marked by specific start and end anchor comments, allowing for selective
              // inclusion of file content based on these anchors.
              const anchorRegex = new RegExp(
                `// ANCHOR: ${anchor}(\\s*[\\s\\S]*?)// ANCHOR_END: ${anchor}`,
                "m",
              );
              const anchoredContent = fileContent.match(anchorRegex);
              fileContent = anchoredContent
                ? anchoredContent[1]
                : `// Anchor '${anchor}' not found in ${filePath}`;
            }
            // Remove lines containing start and end anchor comments
            fileContent = fileContent
              .split("\n")
              .filter((line) => !line.includes("// ANCHOR: "))
              .join("\n");
            fileContent = fileContent
              .split("\n")
              .filter((line) => !line.includes("// ANCHOR_END: "))
              .join("\n");
            // Replace the include directive with the indented file content
            node.value = node.value.replace(match[0], fileContent);
          } catch (err) {
            if (err instanceof Error) {
              console.error(`Error reading file '${fullPath}': ${err.message}`);
            }
          }
        }
      }
    });
  };
};

export default remarkIncludeCode;
