import { rehype } from "rehype";
import { describe, expect, test } from "vitest";
import rehypeMermaidClient from "./rehype-mermaid-client";

const render = async (html: string): Promise<string> =>
  String(
    await rehype().data("settings", { fragment: true }).use(rehypeMermaidClient).process(html),
  );

describe("rehypeMermaidClient", () => {
  test("prepares Mermaid code blocks for browser rendering", async () => {
    const html = '<pre><code class="language-mermaid">graph TD\nA--&gt;B</code></pre>';

    await expect(render(html)).resolves.toBe('<pre class="mermaid">graph TD\nA-->B</pre>');
  });

  test("leaves other code blocks unchanged", async () => {
    const html = '<pre><code class="language-rust">fn main() {}</code></pre>';

    await expect(render(html)).resolves.toBe(html);
  });
});
