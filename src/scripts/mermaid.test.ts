// @vitest-environment jsdom

import { beforeEach, describe, expect, test, vi } from "vitest";
import { initializeMermaidDiagrams } from "./mermaid";

const observe = vi.fn(() => ({ disconnect: vi.fn(), observe: vi.fn(), takeRecords: vi.fn() }));

describe("initializeMermaidDiagrams", () => {
  beforeEach(() => {
    document.documentElement.dataset.theme = "dark";
    document.body.innerHTML = "";
    vi.clearAllMocks();
  });

  test("does not load Mermaid on pages without diagrams", async () => {
    const loadMermaid = vi.fn();

    await expect(initializeMermaidDiagrams({ loadMermaid, observe })).resolves.toBeUndefined();

    expect(loadMermaid).not.toHaveBeenCalled();
  });

  test("renders with strict mode and follows theme changes", async () => {
    document.body.innerHTML = '<pre class="mermaid">graph TD\nA--&gt;B</pre>';
    const diagram = document.querySelector<HTMLElement>("pre.mermaid")!;
    const mermaid = createMermaid(async ({ nodes }) => {
      nodes[0].innerHTML = "<svg></svg>";
    });

    const controller = await initializeMermaidDiagrams({
      loadMermaid: async () => mermaid,
      observe,
    });

    expect(mermaid.initialize).toHaveBeenCalledWith({
      securityLevel: "strict",
      startOnLoad: false,
      theme: "dark",
    });
    expect(mermaid.run).toHaveBeenCalledWith({ nodes: [diagram] });
    expect(diagram.querySelector("svg")).not.toBeNull();

    document.documentElement.dataset.theme = "light";
    await controller!.render();

    expect(mermaid.initialize).toHaveBeenLastCalledWith(
      expect.objectContaining({ theme: "default" }),
    );
  });

  test("restores the source when rendering fails", async () => {
    document.body.innerHTML = '<pre class="mermaid">graph TD\nA--&gt;B</pre>';
    const reportError = vi.fn();
    const error = new Error("invalid diagram");
    const mermaid = createMermaid(async () => Promise.reject(error));

    await initializeMermaidDiagrams({ loadMermaid: async () => mermaid, observe, reportError });

    expect(document.querySelector("pre.mermaid")?.textContent).toBe("graph TD\nA-->B");
    expect(reportError).toHaveBeenCalledWith(error);
  });
});

function createMermaid(run: (options: { nodes: HTMLElement[] }) => Promise<void>) {
  return {
    initialize: vi.fn(),
    run: vi.fn(run),
  } as unknown as typeof import("mermaid").default;
}
