// Generates a link preview image for every docs page at build time.
import type { APIRoute, GetStaticPaths } from "astro";
import { getOgCards, getOgImage } from "~/og/pages";

export const getStaticPaths: GetStaticPaths = async () =>
  [...(await getOgCards()).keys()].map((id) => ({ params: { slug: id } }));

export const GET: APIRoute = async ({ params }) =>
  new Response(new Uint8Array((await getOgImage(params.slug!)).png), {
    headers: { "Content-Type": "image/png" },
  });
