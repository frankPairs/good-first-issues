import type { GithubRepository } from "../models/repositories"
import IssuesIcon from "./icons/IssuesIcon"
import StarIcon from "./icons/StarIcon"
import styles from './RepositoryCard.module.css'

type Props = {
  repository: GithubRepository
}

function RepositoryCard({ repository }: Readonly<Props>) {
  return (
    <article className={styles.container}>
      <header className={styles.header}>
        <div className={styles.headerContent}>
          <img className={styles.headerAvatar} src={repository.avatarUrl} alt={repository.name} />

          <a href={repository.url} target="_blank" rel="noopener noreferrer">
            <h2 className={styles.headerTitle}>{repository.name}</h2>
          </a>
        </div>
      </header>

      <div className={styles.content}>
        <p className={styles.contentDescription}>{repository.description}</p>
      </div>

      <footer className={styles.footer}>
        <div className={styles.footerContent}> 
            <div className={styles.footerInfoContainer}>
              <StarIcon className={styles.footerIcon} />

              <p className={styles.footerText}>{repository.starsCount}</p>
            </div>  

            <div className={styles.footerInfoContainer}>
              <IssuesIcon className={styles.footerIcon} />

              <p className={styles.footerText}>{repository.openIssuesCount}</p>
            </div> 
        </div>
      </footer>
  </article>
  )
}

export default RepositoryCard