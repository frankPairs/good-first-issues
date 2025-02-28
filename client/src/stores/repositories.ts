import { deepMap, type BaseDeepMap } from "nanostores";
import type { GetGithubRepositoriesParams } from "../models/repositories";

type RepositoriesParamsStore = GetGithubRepositoriesParams & BaseDeepMap;

export const initialState: RepositoriesParamsStore = {
  filters: {
    language: "rust",
  },
  pagination: {
    page: 0,
    perPage: 50,
  },
};

export const $repositoriesParamsStore =
  deepMap<RepositoriesParamsStore>(initialState);
