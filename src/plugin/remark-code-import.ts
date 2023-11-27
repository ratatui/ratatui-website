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
      // Regular expression to match custom include directives in the format:
      // {{#include filepath[:anchor]}}
      //
      // Explanation of the regex pattern:
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
      // "{{#include ../../path/to/file}}" - relative path to file from current markdown file
      // "{{#include @path/to/file}}" - path to file from root directory
      const includeRegex = /\{\{#include (.*?)(?::(.*?))?\}\}/g;
      let match;
      // There can be multiple includes in a code block
      // Perform matches one by one, replacing text in AST along the way
      while ((match = includeRegex.exec(node.value)) !== null) {
        const filePath = match[1];
        const anchor = match[2];
        let fullPath;
        if (filePath.startsWith("@")) {
          // Inspired by astro aliases: https://docs.astro.build/en/guides/aliases/
          // Allows for specifying path from root directory
          fullPath = filePath.replace("@", "./");
        } else {
          fullPath = path.resolve(path.dirname(file.path), filePath);
        }
        try {
          let fileContent = fs.readFileSync(fullPath, "utf8");
          if (anchor && anchor.search(":") != -1) {
            const extrema = anchor.split(":");
            const lines = fileContent.split("\n");
            const startLine = extrema[0];
            const endLine = extrema[1];
            const startIdx = startLine ? parseInt(startLine) - 1 : 0;
            const endIdx = endLine ? parseInt(endLine) : lines.length;
            fileContent = lines.slice(startIdx, endIdx).join("\n");
          } else if (anchor) {
            // If an anchor is specified, extract the relevant section from the file
            // Regular expression for extracting a section of content based on anchors in a file.
            //
            // Explanation of the regex pattern:
            //
            // `\\s*/{2,} ANCHOR: ${anchor}\\n{1,}` - This part matches the start of the anchored section:
            //   \\s*        - Matches any whitespace characters (including none) before the anchor comment.
            //   /{2,}       - Matches two or more forward slashes, accommodating different comment styles.
            //   ANCHOR:     - Matches the literal string " ANCHOR:" which precedes the anchor name.
            //   ${anchor}   - Dynamically inserts the anchor name into the regex.
            //   \\n{1,}     - Matches one or more newline characters, accommodating variations in line breaks.
            //
            // `(\\s*[\\s\\S]*?)` - This is the capturing group that matches the content of the anchored section:
            //   \\s*        - Matches any whitespace characters (including none) at the beginning of the content.
            //   [\\s\\S]*?  - A non-greedy match for any characters (including newlines). This captures the
            //                 content until it reaches the end anchor comment.
            //
            // `\\n{1,}\\s*/{2,} ANCHOR_END: ${anchor}` - This part matches the end of the anchored section:
            //   \\n{1,}     - Matches one or more newline characters before the end anchor comment.
            //   \\s*        - Matches any whitespace characters (including none) before the end anchor comment.
            //   /{2,}       - Matches two or more forward slashes, similar to the start anchor.
            //   ANCHOR_END: - Matches the literal string " ANCHOR_END:" which precedes the anchor name.
            //   ${anchor}   - Dynamically inserts the anchor name, ensuring it matches the corresponding start anchor.
            //
            // The "m" flag (multiline) allows '^' and '$' to match the start and end of lines, not just the start
            // and end of the string. This is important for matching anchors that are not at the very start or end
            // of the file content.
            //
            // This regex is designed to be flexible with line endings and whitespace, accommodating variations in file formatting.
            const anchorRegex = new RegExp(
              `\\s*/{2,} ANCHOR: ${anchor}\\n{1,}(\\s*[\\s\\S]*?)\\n{1,}\\s*/{2,} ANCHOR_END: ${anchor}`,
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
            .filter((line) => !line.includes("// ANCHOR: ") && !line.includes("// ANCHOR_END: "))
            .join("\n");
          // Replace the include directive with the indented file content
          node.value = node.value.replace(match[0], fileContent);
        } catch (err) {
          if (err instanceof Error) {
            throw new Error(`Error reading file '${fullPath}': ${err.message}`);
          }
        }
      }
    });
  };
};

export default remarkIncludeCode;
