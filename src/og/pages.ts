import { getCollection, type CollectionEntry } from "astro:content";
import { createHash } from "node:crypto";
import { renderOgCard, type OgCard } from "./card";
import { sectionTitle } from "./text";

/** Builds card content for every docs page, keyed by entry id. */
async function loadOgCards(): Promise<Map<string, OgCard>> {
  const docs: CollectionEntry<"docs">[] = await getCollection("docs");
  const titles = new Map(docs.map((entry) => [entry.id, entry.data.title]));
  return new Map(
    docs.map((entry) => [
      entry.id,
      {
        title: entry.data.title,
        description: entry.data.description,
        section: sectionTitle(entry.id, titles) || undefined,
      },
    ]),
  );
}

let cards: Promise<Map<string, OgCard>> | undefined;
export function getOgCards() {
  cards ??= loadOgCards();
  return cards;
}

// Each card is rendered once and shared by the image endpoint and the page's meta tags.
const images = new Map<string, Promise<{ png: Buffer; hash: string }>>();
export function getOgImage(id: string) {
  let image = images.get(id);
  if (!image) {
    image = (async () => {
      const card = (await getOgCards()).get(id);
      if (!card) throw new Error(`No docs entry for OG image ${id}`);
      const png = await renderOgCard(card);
      return { png, hash: createHash("sha256").update(png).digest("hex").slice(0, 8) };
    })();
    images.set(id, image);
  }
  return image;
}

/**
 * URL path for a docs entry's image, e.g. `/og/concepts/layout.png?v=3f9a1c2e`. The hash changes
 * whenever the image does, so link preview caches that key on the URL fetch the new image.
 */
export async function ogImageUrl(id: string) {
  return `/og/${id}.png?v=${(await getOgImage(id)).hash}`;
}
