import { defineRouteMiddleware } from "@astrojs/starlight/route-data";
import { OG_HEIGHT, OG_WIDTH } from "~/og/card";
import { ogImageUrl } from "~/og/pages";

// Point each page's link preview at its generated card.
export const onRequest = defineRouteMiddleware(async (context) => {
  const { entry, head } = context.locals.starlightRoute;
  // Route data gives the home page an empty id, while the docs collection calls it `index`.
  // Starlight's built-in 404 page isn't in the collection, so it reuses the home page card.
  const id = entry.id === "" || entry.id === "404" ? "index" : entry.id;
  const image = new URL(await ogImageUrl(id), context.site).href;
  for (const [key, value] of [
    ["og:image", image],
    ["og:image:width", String(OG_WIDTH)],
    ["og:image:height", String(OG_HEIGHT)],
    ["og:image:alt", entry.data.title],
  ]) {
    head.push({ tag: "meta", attrs: { property: key, content: value } });
  }
});
