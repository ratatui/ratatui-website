import { docsLoader } from "@astrojs/starlight/loaders";
import { docsSchema } from "@astrojs/starlight/schema";
import { z } from "astro/zod";
import { defineCollection } from "astro:content";
import { autoSidebarLoader } from "starlight-auto-sidebar/loader";
import { autoSidebarSchema } from "starlight-auto-sidebar/schema";

export const collections = {
  docs: defineCollection({
    loader: docsLoader(),
    // Require a description so every page has useful link previews and search snippets.
    schema: docsSchema({ extend: z.object({ description: z.string().min(1) }) }),
  }),
  autoSidebar: defineCollection({
    loader: autoSidebarLoader(),
    schema: autoSidebarSchema(),
  }),
};
