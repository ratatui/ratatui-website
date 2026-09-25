// Serves docs source at `<page path>.md` for the "Copy page" action.
// Code includes are expanded; MDX and other surrounding source are preserved.
import type { APIRoute, GetStaticPaths } from "astro";
import { getCollection, type CollectionEntry } from "astro:content";
import { expandIncludes } from "~/utils/expand-includes";

interface Props {
  entry: CollectionEntry<"docs">;
}

export const getStaticPaths: GetStaticPaths = async () => {
  const docs = await getCollection("docs");
  return (
    docs
      // The site root is a splash page with no prose worth serving.
      .filter((entry) => entry.id !== "")
      .map((entry) => ({ params: { slug: entry.id }, props: { entry } }))
  );
};

export const GET: APIRoute<Props> = ({ props }) => {
  const { entry } = props;
  const title = entry.data.title;
  const description = entry.data.description;
  const metadata = [
    "---",
    `title: ${JSON.stringify(title)}`,
    ...(description ? [`description: ${JSON.stringify(description)}`] : []),
    "---",
  ].join("\n");
  const documentHeader = [`# ${title}`, description].filter(Boolean).join("\n\n");
  const body = expandIncludes(entry.body ?? "", entry.filePath);
  return new Response(`${metadata}\n\n${documentHeader}\n\n${body}`, {
    headers: { "Content-Type": "text/markdown; charset=utf-8" },
  });
};
