import { visit } from "unist-util-visit";
import fs from "fs";
import path from "path";
import { VFile } from "vfile";

const remarkIncludeCode = () => {
  return (tree: Node, file: VFile) => {
    visit(tree, "code", (node) => {
      if (node.lang === "rust") {
        const includeRegex = /\{\{#include (.*?)(?::(.*?))?\}\}/g;
        let match;
        while ((match = includeRegex.exec(node.value)) !== null) {
          const filePath = match[1];
          const anchor = match[2];
          const fullPath = path.resolve(path.dirname(file.path), filePath);
          try {
            let fileContent = fs.readFileSync(fullPath, "utf8");

            if (anchor) {
              const anchorRegex = new RegExp(
                `// ANCHOR: ${anchor}\\s*([\\s\\S]*?)// ANCHOR_END: ${anchor}`,
                "m",
              );
              const anchoredContent = fileContent.match(anchorRegex);
              fileContent = anchoredContent
                ? anchoredContent[1]
                : `// Anchor '${anchor}' not found in ${filePath}`;
            }
            fileContent = fileContent
              .split("\n")
              .filter((line) => !line.includes("// ANCHOR: "))
              .join("\n");
            fileContent = fileContent
              .split("\n")
              .filter((line) => !line.includes("// ANCHOR_END: "))
              .join("\n");
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
