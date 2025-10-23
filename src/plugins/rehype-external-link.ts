import { visit } from "unist-util-visit";
export default rehypeExternalLink;
/**
 * Rehype plugin: automatically adds the Starlight-style external link icon
 * to all external <a> tags in Markdown content.
 */
function rehypeExternalLink() {
  const iconNode = getExternalLinkSvgNode();
  let isATagInsideH2 = false;
  return (tree: Node) => {
    visit(tree, "element", (node: any) => {
      // Detect when a link is the first child of an <h2> element.
      // We set a flag so we can skip adding the external‑link icon for this specific link.
      if (node.tagName === "h2") {
        if (node.children[0].type === "element" && node.children[0].tagName === "a") {
          isATagInsideH2 = true;
        }
      }
      // Process every <a> element encountered during traversal.
      if (node.tagName === "a" && node.properties?.href) {
        // If the link was inside an <h2>, reset the flag and skip icon logic.
        if (isATagInsideH2) {
          isATagInsideH2 = false;
        } else {
          // Normal external-link handling
          const href: string = node.properties.href as string;
          const isExternal = /^https?:\/\//.test(href);

          if (isExternal) {
            node.properties.target = "_blank";
            node.properties.rel = "noopener noreferrer";
            node.children.push(iconNode);
          }
        }
      }
    });
  };
}
/**
 * Returns a virtual node representing the external‑link icon used in Starlight.
 * The icon is rendered as an SVG element with two path children that form the
 * double‑arrow shape.  The returned node follows the unist format that
 * rehype operates on, so it can be appended directly to link nodes.
 */
function getExternalLinkSvgNode(): any {
  return {
    type: "element",
    tagName: "svg",
    properties: {
      "aria-hidden": "true",
      className: ["inline-block"],
      width: "16",
      height: "16",
      viewBox: "0 0 24 24",
      fill: "currentColor",
      style: "transform: translateY(0.2em); -sl-icon-size: 1.5rem;",
    },
    children: [
      {
        type: "element",
        tagName: "path",
        properties: {
          d: "M19.33 10.18a1 1 0 0 1-.77 0 1 1 0 0 1-.62-.93l.01-1.83-8.2 8.2a1 1 0 0 1-1.41-1.42l8.2-8.2H14.7a1 1 0 0 1 0-2h4.25a1 1 0 0 1 1 1v4.25a1 1 0 0 1-.62.93Z",
        },
        children: [],
      },
      {
        type: "element",
        tagName: "path",
        properties: {
          d: "M11 4a1 1 0 1 1 0 2H7a1 1 0 0 0-1 1v10a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1v-4a1 1 0 1 1 2 0v4a3 3 0 0 1-3 3H7a3 3 0 0 1-3-3V7a3 3 0 0 1 3-3h4Z",
        },
        children: [],
      },
    ],
  };
}
