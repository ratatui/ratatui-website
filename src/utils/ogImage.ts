import type { ImageMetadata } from "astro";

/** Fallback image used when a page has no featured media of its own. */
const DEFAULT_OG_IMAGE = "/ratatui-og.png";

/** Local images that pages may reference, loaded lazily so unused ones aren't processed. */
const localImages = import.meta.glob<{ default: ImageMetadata }>([
  "/src/assets/**/*.{png,jpg,jpeg,gif,webp,avif}",
  "/src/content/**/*.{png,jpg,jpeg,gif,webp,avif}",
]);

export interface OgImage {
  url: string;
  alt?: string;
  width?: number;
  height?: number;
}

/** Images that make poor link previews: badges and formats social platforms don't render. */
function isUsable(src: string): boolean {
  const url = src.startsWith("//") ? `https:${src}` : src;
  const path = url.split(/[?#]/, 1)[0] ?? "";
  if (/\.svgz?$/i.test(path)) return false;
  return !/^https?:\/\/(img\.shields\.io|badgen\.net|badge\.fury\.io)\//i.test(url);
}

/**
 * Remove fenced code blocks so that example snippets don't provide the preview image.
 *
 * A fence is only closed by at least as many of the same character it was opened with, so a block
 * opened with ```` ```` ```` isn't ended by a ```` ``` ```` line inside it.
 */
function stripCodeFences(body: string): string {
  const lines: string[] = [];
  let fence: string | undefined;

  for (const line of body.split("\n")) {
    const marker = line.match(/^ {0,3}(`{3,}|~{3,})/)?.[1];
    if (fence === undefined) {
      if (marker) fence = marker;
      else lines.push(line);
    } else if (
      marker &&
      marker[0] === fence[0] &&
      marker.length >= fence.length &&
      // A closing fence carries nothing but the fence itself.
      line.trimEnd().length === line.indexOf(marker) + marker.length
    ) {
      fence = undefined;
    }
  }

  return lines.join("\n");
}

/** Markdown (`![alt](src)`) and HTML (`<img src="…">`) images in a page body, in order. */
function findImages(body: string): { src: string; alt: string }[] {
  const content = stripCodeFences(body);

  const pattern =
    /!\[([^\]]*)\]\(\s*<?([^\s)>]+)>?(?:\s+["'][^"']*["'])?\s*\)|<img\b[^>]*?\bsrc\s*=\s*["']([^"']+)["'][^>]*>/gi;

  const images: { src: string; alt: string }[] = [];
  for (const [tag, markdownAlt, markdownSrc, htmlSrc] of content.matchAll(pattern)) {
    const src = markdownSrc ?? htmlSrc;
    if (!src || !isUsable(src)) continue;
    const alt = markdownSrc ? markdownAlt : tag.match(/\balt\s*=\s*["']([^"']*)["']/i)?.[1];
    images.push({ src, alt: alt ?? "" });
  }
  return images;
}

/** Resolve a path relative to a file, e.g. `("src/content/docs/a/b.md", "../c.png")`. */
function resolveRelative(filePath: string, src: string): string {
  const segments = `/${filePath}`.split("/").slice(0, -1).concat(src.split("/"));
  const resolved: string[] = [];
  for (const segment of segments) {
    if (segment === "" || segment === ".") continue;
    if (segment === "..") resolved.pop();
    else resolved.push(segment);
  }
  return `/${resolved.join("/")}`;
}

/**
 * Resolve an image reference from a page body into an absolute URL.
 *
 * Remote images are used as-is, root-relative paths are resolved against the site URL, and paths
 * relative to the page source are looked up in the bundled assets so that the built (hashed) URL
 * is used.
 */
async function resolveImage(
  src: string,
  filePath: string,
  site: URL | undefined,
): Promise<Omit<OgImage, "alt"> | undefined> {
  if (/^https?:\/\//i.test(src)) return { url: src };
  if (src.startsWith("//")) return { url: `https:${src}` };
  if (src.startsWith("data:")) return undefined;

  const isRootRelative = src.startsWith("/");
  const path = isRootRelative ? src : resolveRelative(filePath, src);

  const loadImage = localImages[path];
  if (!loadImage) {
    // Root-relative paths are images shipped in `public/`, served from the site root as-is.
    // Anything else failed to resolve to a bundled asset, so there is no URL worth advertising.
    return isRootRelative && site ? { url: new URL(path, site).href } : undefined;
  }

  const { default: image } = await loadImage();
  return {
    url: site ? new URL(image.src, site).href : image.src,
    width: image.width,
    height: image.height,
  };
}

/**
 * Get the Open Graph image for a page.
 *
 * Pages can set an `ogImage` in their frontmatter. Otherwise the first image in the page is used,
 * so that release highlights and showcase pages preview their own animations and screenshots. Pages
 * without any image fall back to the site-wide Open Graph image.
 */
export async function getOgImage(entry: {
  body?: string;
  filePath?: string;
  data: { title?: string; ogImage?: ImageMetadata | string };
  site?: URL;
}): Promise<OgImage> {
  const { body = "", filePath = "", data, site } = entry;
  const fallback = { url: site ? new URL(DEFAULT_OG_IMAGE, site).href : DEFAULT_OG_IMAGE };

  const frontmatter = data.ogImage;
  if (frontmatter) {
    if (typeof frontmatter === "string") {
      const resolved = await resolveImage(frontmatter, filePath, site);
      return { ...(resolved ?? fallback), alt: data.title };
    }
    return {
      url: site ? new URL(frontmatter.src, site).href : frontmatter.src,
      width: frontmatter.width,
      height: frontmatter.height,
      alt: data.title,
    };
  }

  for (const image of findImages(body)) {
    const resolved = await resolveImage(image.src, filePath, site);
    if (resolved) return { ...resolved, alt: image.alt || data.title };
  }

  return fallback;
}
