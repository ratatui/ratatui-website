import { docsLoader } from "@astrojs/starlight/loaders";
import { docsSchema } from "@astrojs/starlight/schema";
import { defineCollection, z } from "astro:content";
import { autoSidebarLoader } from "starlight-auto-sidebar/loader";
import { autoSidebarSchema } from "starlight-auto-sidebar/schema";

export const collections = {
  docs: defineCollection({
    loader: docsLoader(),
    schema: docsSchema({
      extend: ({ image }) =>
        z.object({
          /**
           * Image used for link previews (Open Graph / Twitter cards).
           *
           * Either a path to a local image, or the URL of a remote one. Defaults to the first
           * image on the page, falling back to the site-wide preview image.
           */
          ogImage: z.union([image(), z.string()]).optional(),
        }),
    }),
  }),
  autoSidebar: defineCollection({
    loader: autoSidebarLoader(),
    schema: autoSidebarSchema(),
  }),
};
