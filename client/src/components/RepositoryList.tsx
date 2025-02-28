import { subscribeKeys } from "nanostores";
import { useEffect, useState } from "react";
import { useInfiniteQuery } from "@tanstack/react-query";

import { $repositoriesParamsStore, initialState } from "../stores/repositories";
import { getGithubRepositories } from "../requests/repositories";
import styles from "./RepositoryList.module.css";
import type { GetGithubRepositoriesParams } from "../models/repositories";
import { queryClient } from "../stores/queryClient";
import RepositoryCard from "./RepositoryCard";
import InfiniteScroll from "react-infinite-scroll-component";

function RepositoryList() {
  const [repositoriesParams, setRepositoriesParams] =
    useState<GetGithubRepositoriesParams>(initialState);
  const { data, isPending, fetchNextPage } = useInfiniteQuery(
    {
      initialPageParam: 1,
      queryKey: ["GET_REPOSITORY_LIST", repositoriesParams],
      queryFn: ({ pageParam }) => {
        return getGithubRepositories({
          language: repositoriesParams.filters.language,
          page: pageParam,
          perPage: repositoriesParams.pagination.perPage,
        });
      },
      getNextPageParam(_, pages) {
        const pagesLength = pages.length;

        return pagesLength + 1;
      },
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
        hasMore={repositories.length < totalCount}
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
