// An expressive code plugin that makes code blocks with titles collapsible.
// from: https://github.com/expressive-code/expressive-code/issues/109

import { select } from "hast-util-select";
import { h } from "hastscript";

export function collapsibleFrames() {
  return {
    name: "Collapsible Frames",
    baseStyles: `
      .collapsible-frame {
        & summary {
          cursor: pointer;
          &::marker {
            display: inline-block;
            content: "";
            width: 16px;
            height: 16px;
          }        
          &::-webkit-details-marker {
            display: none;
          }
        }
      }
    `,
    hooks: {
      postprocessRenderedBlock: ({ codeBlock, renderData }) => {
        if (!codeBlock.meta.includes("collapsible") && !codeBlock.meta.includes("collapsed"))
          return;
        const frame = select(".frame", renderData.blockAst);
        if (!frame) return;
        const header = select(".frame > .header:has(.title)", renderData.blockAst);
        if (!header) return;
        // Use the frame header as the `<summary>`
        const headerIndex = frame.children.indexOf(header);
        frame.children.splice(headerIndex, 1, h("summary", [header]));

        // Wrap all frame contents in a `<details>` element
        frame.children = [
          h(
            "details.collapsible-frame",
            {
              open: !codeBlock.meta.includes("collapsed"),
            },
            [frame.children],
          ),
        ];
      },
    },
  };
}
