import type { NextPage } from "next";
import styles from "../styles/Home.module.css";
import { Text, Title, Button } from "@mantine/core";

const Home: NextPage = () => {
  return (
    <>
      <div className={styles.top_container}>
        <div>
          <Title order={1}>
            Night Protocol
          </Title>
          <Text size="lg" className={styles.text}>A blockchain protocol written in Rust</Text>
          <br />
          <Button className={styles.text} variant="gradient" gradient={{ from: 'orange', to: 'red' }} >Dashboard</Button>
        </div>
      </div>
    </>
  );
};

export default Home;
