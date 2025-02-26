import type { ProgrammingLanguageID } from "./programmingLanguages"

export interface GetGithubRepositoriesAPIParams {
  page?: number
  perPage?: number
  language: string
}

export interface GetGithubRepositoriesParams {
  filters: {
    language: ProgrammingLanguageID
  },
  pagination: {
    page: number
    perPage: number
  }
}


export interface GithubRepositoryAPI {
  id: number,
  url: string,
  name: string,
  private: boolean,
  avatar_url: string,
  description?: string,
  stars_count: number,
  open_issues_count: number,
  has_issues: boolean,
  license?: string,
}

export interface GetGithubRepositoryResponseAPI {
  total_count: number,
  items: GithubRepositoryAPI[]
}

export interface GithubRepository {
  id: number,
  url: string,
  name: string,
  private: boolean,
  avatarUrl: string,
  description?: string,
  starsCount: number,
  openIssuesCount: number,
  hasIssues: boolean,
  license?: string,
}

export interface GetGithubRepositoriesResponse {
  totalCount: number,
  repositories: GithubRepository[]
}