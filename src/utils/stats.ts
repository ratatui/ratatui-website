// Utility functions to fetch real statistics from APIs

import { Octokit } from "octokit";
import type { CratesIoResponse, CratesIoReverseDepsResponse, GitHubRepo } from "~/models/stats";

const token = import.meta.env.GITHUB_TOKEN ?? process.env.GITHUB_TOKEN;
const octokit = new Octokit({
  auth: token,
});

export async function getGitHubStats(owner: string, repo: string): Promise<GitHubRepo> {
  try {
    // Use Octokit's rest API for repo info
    const { data } = await octokit.rest.repos.get({
      owner,
      repo,
    });

    if (typeof data.stargazers_count !== "number" || typeof data.forks_count !== "number") {
      console.error("Invalid GitHub response structure:", {
        stargazers_count: data.stargazers_count,
        forks_count: data.forks_count,
      });
      throw new Error("Invalid response structure from GitHub API");
    }

    return {
      stars: data.stargazers_count,
      forks: data.forks_count,
    };
  } catch (error) {
    console.error("Failed to fetch GitHub stats (Octokit):", error);
    // Fallback values
    return { stars: 13418, forks: 500 };
  }
}

export async function getCratesStats(): Promise<{ downloads: number }> {
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
    return { downloads: 7124990 };
  }
}

export async function getShowcaseAppsCount(): Promise<{ count: number }> {
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
    return { count: 1049 };
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
