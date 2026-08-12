type Mermaid = typeof import("mermaid").default;

interface Diagram {
  element: HTMLElement;
  source: string;
}

export interface MermaidController {
  disconnect(): void;
  render(): Promise<void>;
}

interface MermaidOptions {
  document?: Document;
  loadMermaid?: () => Promise<Mermaid>;
  observe?: (callback: MutationCallback) => MutationObserver;
  reportError?: (error: unknown) => void;
}

/**
 * Render Markdown Mermaid fences in the browser and keep them in sync with the site theme.
 *
 * Mermaid is loaded only when the page contains a diagram. Rendering stays client-side so the
 * Cloudflare build does not need Playwright, while the original text is retained as a fallback if
 * Mermaid rejects a diagram.
 */
export async function initializeMermaidDiagrams({
  document: page = document,
  loadMermaid = async () => (await import("mermaid")).default,
  observe = (callback) => new MutationObserver(callback),
  reportError = (error) => console.error("Unable to render Mermaid diagram", error),
}: MermaidOptions = {}): Promise<MermaidController | undefined> {
  const diagrams = findDiagrams(page);
  if (diagrams.length === 0) return;

  const mermaid = await loadMermaid();
  const render = serialize(() => renderDiagrams(page, mermaid, diagrams, reportError));
  await render();

  const observer = observe(() => void render());
  observer.observe(page.documentElement, { attributeFilter: ["data-theme"] });

  return {
    disconnect: () => observer.disconnect(),
    render,
  };
}

/** Find client-renderable diagram elements and preserve their authored Mermaid source. */
function findDiagrams(page: Document): Diagram[] {
  return Array.from(page.querySelectorAll<HTMLElement>("pre.mermaid"), (element) => ({
    element,
    source: element.textContent ?? "",
  }));
}

/**
 * Queue calls to an asynchronous action so a new render cannot overlap one already in progress.
 */
function serialize(action: () => Promise<void>): () => Promise<void> {
  let pending = Promise.resolve();
  return () => (pending = pending.then(action));
}

/** Configure Mermaid for the current site theme and render every diagram in document order. */
async function renderDiagrams(
  page: Document,
  mermaid: Mermaid,
  diagrams: Diagram[],
  reportError: (error: unknown) => void,
): Promise<void> {
  mermaid.initialize({
    securityLevel: "strict",
    startOnLoad: false,
    theme: page.documentElement.dataset.theme === "dark" ? "dark" : "default",
  });

  for (const diagram of diagrams) {
    await renderDiagram(mermaid, diagram, reportError);
  }
}

/**
 * Restore a diagram's authored source before rendering it, and retain that source as the fallback
 * when Mermaid cannot parse or render the diagram.
 */
async function renderDiagram(
  mermaid: Mermaid,
  { element, source }: Diagram,
  reportError: (error: unknown) => void,
): Promise<void> {
  element.removeAttribute("data-processed");
  element.textContent = source;

  try {
    await mermaid.run({ nodes: [element] });
  } catch (error) {
    element.textContent = source;
    reportError(error);
  }
}
