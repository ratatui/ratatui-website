import { afterEach, describe, expect, it, vi } from "vitest";
import { getGitHubStats } from "./stats";

describe("getGitHubStats", () => {
  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it("times out, retries once, and falls back without waiting indefinitely", async () => {
    vi.useFakeTimers();
    vi.spyOn(console, "info").mockImplementation(() => {});
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const request = vi.fn(
      (_owner: string, _repo: string, _signal: AbortSignal) => new Promise<never>(() => {}),
    );

    const stats = getGitHubStats("ratatui", "ratatui", {
      request,
      timeoutMs: 100,
      retryDelayMs: 25,
    });
    await vi.advanceTimersByTimeAsync(225);

    await expect(stats).resolves.toEqual({ stars: 13418, forks: 500 });
    expect(request).toHaveBeenCalledTimes(2);
    expect(request.mock.calls.every(([, , signal]) => signal.aborted)).toBe(true);
    expect(warn).toHaveBeenLastCalledWith(
      expect.stringMatching(/fell back after \d+ms.*timed out/),
    );
  });

  it("retries a transient network failure with backoff and then succeeds", async () => {
    vi.spyOn(console, "info").mockImplementation(() => {});
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const request = vi
      .fn()
      .mockRejectedValueOnce(new TypeError("fetch failed"))
      .mockResolvedValueOnce({ stars: 42, forks: 7 });
    const sleep = vi.fn().mockResolvedValue(undefined);

    await expect(
      getGitHubStats("owner", "repo", { request, retryDelayMs: 250, sleep }),
    ).resolves.toEqual({ stars: 42, forks: 7 });
    expect(request).toHaveBeenCalledTimes(2);
    expect(sleep).toHaveBeenCalledWith(250);
    expect(warn).toHaveBeenCalledWith(expect.stringContaining("owner/repo: retrying"));
  });

  it.each([401, 403, 429])("does not retry HTTP %s failures", async (status) => {
    vi.spyOn(console, "info").mockImplementation(() => {});
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const request = vi
      .fn()
      .mockRejectedValue(Object.assign(new Error("request failed"), { status }));
    const sleep = vi.fn();

    await expect(getGitHubStats("owner", "repo", { request, sleep })).resolves.toEqual({
      stars: 13418,
      forks: 500,
    });
    expect(request).toHaveBeenCalledOnce();
    expect(sleep).not.toHaveBeenCalled();
    expect(warn).toHaveBeenCalledWith(
      expect.stringMatching(new RegExp(`owner/repo: fell back after \\d+ms \\(HTTP ${status}\\)`)),
    );
  });
});
