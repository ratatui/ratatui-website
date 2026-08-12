// Utility functions to fetch real statistics from APIs

import { Octokit } from "octokit";
import type { CratesIoResponse, CratesIoReverseDepsResponse, GitHubRepo } from "~/models/stats";

const token = import.meta.env.GITHUB_TOKEN ?? process.env.GITHUB_TOKEN;
const octokit = new Octokit({
  auth: token,
  // Keep retries and rate-limit waits inside our bounded, logged policy below.
  retry: { enabled: false },
  throttle: { enabled: false },
});

const GITHUB_STATS_TIMEOUT_MS = 5_000;
const GITHUB_STATS_RETRY_DELAY_MS = 500;
const GITHUB_STATS_ATTEMPTS = 2;

type GitHubStatsRequest = (owner: string, repo: string, signal: AbortSignal) => Promise<GitHubRepo>;

/** Dependency overrides for deterministic timeout and retry tests. */
interface GitHubStatsOptions {
  request?: GitHubStatsRequest;
  timeoutMs?: number;
  retryDelayMs?: number;
  sleep?: (milliseconds: number) => Promise<void>;
}

class InvalidGitHubResponseError extends Error {}

class GitHubStatsTimeoutError extends Error {}

async function requestGitHubStats(
  owner: string,
  repo: string,
  signal: AbortSignal,
): Promise<GitHubRepo> {
  const { data } = await octokit.rest.repos.get({
    owner,
    repo,
    request: { signal },
  });

  if (typeof data.stargazers_count !== "number" || typeof data.forks_count !== "number") {
    throw new InvalidGitHubResponseError("invalid response structure");
  }

  return {
    stars: data.stargazers_count,
    forks: data.forks_count,
  };
}

/** Runs one request attempt and aborts its underlying fetch when the deadline expires. */
async function withTimeout<T>(
  request: (signal: AbortSignal) => Promise<T>,
  timeoutMs: number,
): Promise<T> {
  const controller = new AbortController();
  let timeoutId: ReturnType<typeof setTimeout>;
  const timeout = new Promise<never>((_, reject) => {
    timeoutId = setTimeout(() => {
      controller.abort();
      reject(new GitHubStatsTimeoutError(`timed out after ${timeoutMs}ms`));
    }, timeoutMs);
  });

  try {
    return await Promise.race([request(controller.signal), timeout]);
  } finally {
    clearTimeout(timeoutId!);
  }
}

function errorStatus(error: unknown): number | undefined {
  if (typeof error !== "object" || error === null || !("status" in error)) {
    return undefined;
  }
  return typeof error.status === "number" ? error.status : undefined;
}

function describeGitHubError(error: unknown): string {
  const status = errorStatus(error);
  if (status !== undefined) {
    return `HTTP ${status}`;
  }
  return error instanceof Error ? error.message : "unknown error";
}

/** Retries network failures, timeouts, request timeouts, and selected transient server errors. */
function shouldRetryGitHubError(error: unknown): boolean {
  if (error instanceof InvalidGitHubResponseError) {
    return false;
  }

  const status = errorStatus(error);
  if (status === undefined) {
    return true;
  }
  return status === 408 || status === 500 || status === 502 || status === 503 || status === 504;
}

/**
 * Fetches star and fork counts for a repository without making site availability depend on GitHub.
 *
 * Production requests have a five-second deadline and at most one retry after a short backoff.
 * Authentication, permission, rate-limit, malformed-response, and other non-transient HTTP failures
 * are not retried. Every attempt and outcome is logged with the repository identity; if all eligible
 * attempts fail, this function logs the fallback and returns stable fallback values instead of
 * throwing.
 */
export async function getGitHubStats(
  owner: string,
  repo: string,
  options: GitHubStatsOptions = {},
): Promise<GitHubRepo> {
  const fallback = { stars: 13418, forks: 500 };
  if (import.meta.env.DEV && options.request === undefined) {
    return fallback;
  }

  const identity = `${owner}/${repo}`;
  const request = options.request ?? requestGitHubStats;
  const timeoutMs = options.timeoutMs ?? GITHUB_STATS_TIMEOUT_MS;
  const retryDelayMs = options.retryDelayMs ?? GITHUB_STATS_RETRY_DELAY_MS;
  const sleep =
    options.sleep ??
    ((milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds)));
  const startedAt = Date.now();

  for (let attempt = 1; attempt <= GITHUB_STATS_ATTEMPTS; attempt += 1) {
    console.info(
      `[github-stats] ${identity}: pending (attempt ${attempt}/${GITHUB_STATS_ATTEMPTS}, timeout ${timeoutMs}ms)`,
    );

    try {
      const stats = await withTimeout((signal) => request(owner, repo, signal), timeoutMs);
      console.info(`[github-stats] ${identity}: succeeded in ${Date.now() - startedAt}ms`);
      return stats;
    } catch (error) {
      const elapsed = Date.now() - startedAt;
      const retry = attempt < GITHUB_STATS_ATTEMPTS && shouldRetryGitHubError(error);
      if (!retry) {
        console.warn(
          `[github-stats] ${identity}: fell back after ${elapsed}ms (${describeGitHubError(error)})`,
        );
        return fallback;
      }

      console.warn(
        `[github-stats] ${identity}: retrying after ${elapsed}ms (${describeGitHubError(error)}; backoff ${retryDelayMs}ms)`,
      );
      await sleep(retryDelayMs);
    }
  }

  return fallback;
}

export async function getCratesStats(): Promise<{ downloads: number }> {
  const fallback = { downloads: 7124990 };
  if (import.meta.env.DEV) {
    return fallback;
  }

  try {
    const response = await fetch("https://crates.io/api/v1/crates/ratatui", {
      headers: {
        "User-Agent": "ratatui-website (https://ratatui.rs)",
      },
    });

    if (!response.ok) {
      console.error(`Crates.io API error: ${response.status} ${response.statusText}`);
      console.error("Response headers:", Object.fromEntries(response.headers.entries()));
      const errorText = await response.text();
      console.error("Response body:", errorText);
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data: CratesIoResponse = await response.json();

    if (!data.crate || typeof data.crate.downloads !== "number") {
      console.error("Invalid crates.io response structure:", data);
      throw new Error("Invalid response structure from crates.io API");
    }

    return {
      downloads: data.crate.downloads,
    };
  } catch (error) {
    console.error("Failed to fetch crates.io stats:");
    console.error("Error type:", error?.constructor?.name);
    console.error("Error message:", (error as Error)?.message || "Unknown error");
    console.error("Full error:", error);

    // Fallback value
    return fallback;
  }
}

export async function getShowcaseAppsCount(): Promise<{ count: number }> {
  const fallback = { count: 1049 };
  if (import.meta.env.DEV) {
    return fallback;
  }

  try {
    // Fetch reverse dependencies from crates.io API
    const response = await fetch(
      "https://crates.io/api/v1/crates/ratatui/reverse_dependencies?per_page=1",
      {
        headers: {
          "User-Agent": "ratatui-website (https://ratatui.rs)",
        },
      },
    );

    if (!response.ok) {
      console.error(`Crates.io reverse deps API error: ${response.status} ${response.statusText}`);
      console.error("Response headers:", Object.fromEntries(response.headers.entries()));
      const errorText = await response.text();
      console.error("Response body:", errorText);
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data: CratesIoReverseDepsResponse = await response.json();

    if (!data.meta || typeof data.meta.total !== "number") {
      console.error("Invalid crates.io reverse deps response structure:", data);
      throw new Error("Invalid response structure from crates.io reverse deps API");
    }

    return { count: data.meta.total };
  } catch (error) {
    console.error("Failed to fetch reverse dependencies count:");
    console.error("Error type:", error?.constructor?.name);
    console.error("Error message:", (error as Error)?.message || "Unknown error");
    console.error("Full error:", error);

    // Fallback value
    return fallback;
  }
}

export function formatNumber(num: number): string {
  if (num >= 1000000) {
    return `${Math.round(num / 100000) / 10}M`;
  } else if (num >= 1000) {
    const thousands = Math.round(num / 100) / 10;
    // Special case: show "1.0k" instead of "1k" to make it look more substantial
    if (thousands === 1) {
      return "1.0k";
    }
    return `${thousands}k`;
  }
  return num.toString();
}

export function formatCratesNumber(num: number): string {
  // Round down to nearest 100 for crates count and add "+" to indicate "at least this many"
  const roundedDown = Math.floor(num / 100) * 100;
  return `${roundedDown}+`;
}

export async function getAllStats(owner: string, repo: string) {
  const [github, crates, showcase] = await Promise.all([
    getGitHubStats(owner, repo),
    getCratesStats(),
    getShowcaseAppsCount(),
  ]);

  return {
    crates: formatCratesNumber(showcase.count), // Show actual count rounded down to nearest 100
    stars: formatNumber(github.stars),
    downloads: formatNumber(crates.downloads),
  };
}
