import type {
  GetGithubRepositoryResponseAPI,
  GetGithubRepositoriesAPIParams,
  GetGithubRepositoriesResponse,
} from "../models/repositories";
import { FetchError } from "../errors";

const GITHUB_DEAFULT_PER_PAGE = 50;

export async function getGithubRepositories(
  params: GetGithubRepositoriesAPIParams,
  opts?: RequestInit,
): Promise<GetGithubRepositoriesResponse> {
  const url = new URL("http://localhost:5000/api/v1/github/repositories");

  url.searchParams.append("language", params.language);
  url.searchParams.append("page", String(params.page ?? 1));
  url.searchParams.append(
    "per_page",
    String(params.perPage ?? GITHUB_DEAFULT_PER_PAGE),
  );

  const res = await fetch(url.toString(), opts);

  if (!res.ok) {
    throw new FetchError(res.statusText, res.status);
  }

  const json: GetGithubRepositoryResponseAPI = await res.json();

  return {
    totalCount: json.total_count,
    repositories: json.items.map((item) => ({
      id: item.id,
      url: item.url,
      name: item.name,
      private: item.private,
      avatarUrl: item.avatar_url,
      description: item.description,
      starsCount: item.stars_count,
      openIssuesCount: item.open_issues_count,
      hasIssues: item.has_issues,
      license: item.license,
    })),
  };
}
