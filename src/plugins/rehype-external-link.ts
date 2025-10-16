import { visit } from "unist-util-visit";
export default rehypeExternalLink;
/**
 * Rehype plugin: automatically adds the Starlight-style external link icon
 * to all external <a> tags in Markdown content.
 */
function rehypeExternalLink() {
  // SVG copied from Starlight’s <IconExternalLink /> (Heroicons outline)
  const iconSVG = `<svg aria-hidden="true" class="inline-block" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;">
<path d="M19.33 10.18a1 1 0 0 1-.77 0 1 1 0 0 1-.62-.93l.01-1.83-8.2 8.2a1 1 0 0 1-1.41-1.42l8.2-8.2H14.7a1 1 0 0 1 0-2h4.25a1 1 0 0 1 1 1v4.25a1 1 0 0 1-.62.93Z"></path>
<path d="M11 4a1 1 0 1 1 0 2H7a1 1 0 0 0-1 1v10a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1v-4a1 1 0 1 1 2 0v4a3 3 0 0 1-3 3H7a3 3 0 0 1-3-3V7a3 3 0 0 1 3-3h4Z">
</path>
</svg>`;

  return (tree: Node) => {
    visit(tree, "element", (node: any) => {
      if (node.tagName === "a" && node.properties?.href) {
        const href: string = node.properties.href;
        const isExternal = /^https?:\/\//.test(href);

        if (isExternal) {
          node.properties.target = "_blank";
          node.children.push({
            type: "raw",
            value: iconSVG,
          });
        }
      }
    });
  };
}
