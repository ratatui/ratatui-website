import type { Root } from "hast";
import { visit } from "unist-util-visit";

/**
 * Prepare Mermaid code fences for the browser renderer.
 *
 * Markdown emits `<pre><code class="language-mermaid">…</code></pre>`. Mermaid expects the source
 * directly inside `<pre class="mermaid">`, so this plugin changes only that wrapper and leaves the
 * authored diagram text untouched.
 */
export default function rehypeMermaidClient() {
  return (tree: Root) => {
    visit(tree, "element", (node) => {
      const code = node.children[0];
      if (
        node.tagName === "pre" &&
        code?.type === "element" &&
        code.tagName === "code" &&
        code.properties.className?.includes("language-mermaid")
      ) {
        node.properties.className = ["mermaid"];
        node.children = code.children;
      }
    });
  };
}
