import { subscribeKeys } from "nanostores";
import { useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";

import { $repositoriesParamsStore, initialState } from "../stores/repositories";
import { getGithubRepositories } from "../requests/repositories";
import styles from "./RepositoryList.module.css";
import type { GetGithubRepositoriesParams } from "../models/repositories";
import { queryClient } from "../stores/queryClient";
import RepositoryCard from "./RepositoryCard";

function RepositoryList() {
  const [repositoriesParams, setRepositoriesParams] =
    useState<GetGithubRepositoriesParams>(initialState);
  const { data, isLoading } = useQuery(
    {
      queryKey: ["GET_REPOSITORY_LIST", repositoriesParams],
      queryFn: () =>
        getGithubRepositories({
          language: repositoriesParams.filters.language,
          page: repositoriesParams.pagination.page,
          perPage: repositoriesParams.pagination.perPage,
        }),
    },
    queryClient,
  );

  useEffect(() => {
    const unsubscribe = subscribeKeys(
      $repositoriesParamsStore,
      ["filters.language"],
      (storeParams) => {
        setRepositoriesParams(storeParams);
      },
    );

    return () => unsubscribe();
  }, []);

  if (isLoading) {
    return (
      <section className={styles.container}>
        <p>Loading...</p>
      </section>
    );
  }

  return (
    <section className={styles.container}>
      {data?.repositories.map((repository) => (
        <RepositoryCard key={repository.id} repository={repository} />
      ))}
    </section>
  );
}

export default RepositoryList;
