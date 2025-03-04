import { subscribeKeys } from "nanostores";
import { useEffect, useState } from "react";
import { useInfiniteQuery, type InfiniteData } from "@tanstack/react-query";

import { $repositoriesParamsStore, initialState } from "../stores/repositories";
import { getGithubRepositories } from "../requests/repositories";
import styles from "./RepositoryList.module.css";
import type {
  GetGithubRepositoriesParams,
  GetGithubRepositoriesResponse,
} from "../models/repositories";
import { queryClient } from "../stores/queryClient";
import RepositoryCard from "./RepositoryCard";
import InfiniteScroll from "react-infinite-scroll-component";

/**
 * As Github can return duplicate items on different pages, we have to clean them before showing them.
 * In order to clean up them, we compare the latest result from the API with the one from the
 * cache, and remove the duplications.
 */
function getUniqueRepositories(
  apiResult: GetGithubRepositoriesResponse,
  cacheResult: InfiniteData<GetGithubRepositoriesResponse>,
): GetGithubRepositoriesResponse {
  const { pages } = cacheResult;
  const pagesLength = pages.length;
  const lastPage = pages[pagesLength - 1];
  const lastPageRepositoryIds = new Set(
    lastPage.repositories.map((repository) => repository.id),
  );

  return {
    totalCount: apiResult.totalCount,
    repositories: apiResult.repositories.filter(
      (repository) => !lastPageRepositoryIds.has(repository.id),
    ),
  };
}

function RepositoryList() {
  const [repositoriesParams, setRepositoriesParams] =
    useState<GetGithubRepositoriesParams>(initialState);
  const { data, isPending, isFetching, hasNextPage, fetchNextPage } =
    useInfiniteQuery(
      {
        initialPageParam: 1,
        gcTime: 0,
        queryKey: ["GET_REPOSITORY_LIST", repositoriesParams],
        async queryFn({ pageParam, signal, client }) {
          const apiResult = await getGithubRepositories(
            {
              language: repositoriesParams.filters.language,
              page: pageParam,
              perPage: repositoriesParams.pagination.perPage,
            },
            { signal },
          );
          const cacheResult = client.getQueryData<
            InfiniteData<GetGithubRepositoriesResponse>
          >(["GET_REPOSITORY_LIST", repositoriesParams]);

          return !cacheResult
            ? apiResult
            : getUniqueRepositories(apiResult, cacheResult);
        },
        getNextPageParam: (_, pages) => pages.length + 1,
        getPreviousPageParam: (_, pages) => pages.length - 1,
      },
      queryClient,
    );

  const repositories = data?.pages.flatMap((page) => page.repositories) ?? [];
  const totalCount = data?.pages[0].totalCount ?? 0;

  useEffect(() => {
    const unsubscribe = subscribeKeys(
      $repositoriesParamsStore,
      ["filters.language", "pagination.page"],
      (storeParams) => {
        setRepositoriesParams(storeParams);
      },
    );

    return () => unsubscribe();
  }, []);

  if (isPending) {
    return (
      <section className={styles.container}>
        <p>Loading...</p>
      </section>
    );
  }

  return (
    <section className={styles.container}>
      <InfiniteScroll
        dataLength={repositories.length ?? 0}
        next={fetchNextPage}
        hasMore={!isFetching && hasNextPage && repositories.length < totalCount}
        scrollThreshold={0.7}
        loader={<h4>Loading...</h4>}
        className={styles.infiniteScrollContainer}
      >
        {repositories.map((repository) => (
          <RepositoryCard key={repository.id} repository={repository} />
        ))}
      </InfiniteScroll>
    </section>
  );
}

export default RepositoryList;
