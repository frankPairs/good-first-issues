import {deepMap, type BaseDeepMap} from "nanostores"
import type { ProgrammingLanguageID } from "../models/programmingLanguages"
import type { GetGithubRepositoriesResponse } from "../models/repositories"

export interface RepositoriesStore extends BaseDeepMap {
  filters: {
    language: ProgrammingLanguageID,
  }
  result: GetGithubRepositoriesResponse 
}

export const $repositoriesStore = deepMap<RepositoriesStore>({
  filters: {
    language: "rust"
  },
  result: {
    repositories: [],
    totalCount: 0
  }
})

