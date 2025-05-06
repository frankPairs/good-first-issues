import { FetchError } from "../errors";
import styles from "./ErrorSection.module.css";

type ErrorSectionProps = {
  error: FetchError;
};

function ErrorSection({ error }: Readonly<ErrorSectionProps>) {
  if (error.status === 429) {
    return (
      <div className={styles.container}>
        <h2 className={`${styles.text} ${styles.title}`}>
          429 - Everyone, Chill for a Sec
        </h2>

        <p className={styles.text}>
          Our servers are currently doing the Macarena to keep up.
        </p>
        <p className={styles.text}>
          Please slow your roll and try again in a bit.
        </p>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      <h2 className={`${styles.text} ${styles.title}`}>
        Internal server error
      </h2>

      <p className={styles.text}>
        "Something exploded in the server room. Engineers are crying."
      </p>
    </div>
  );
}

export default ErrorSection;
