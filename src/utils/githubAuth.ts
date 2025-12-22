import { Octokit } from "octokit";

const token = import.meta.env.GITHUB_TOKEN;
const octokit = new Octokit({ auth: token });

const issues = await octokit.request("GET /repos/{owner}/{repo}/issues", {
  owner: "octocat",
  repo: "Spoon-Knife",
});
