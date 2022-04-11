import type { NextPage } from "next";
import { Button, Card, Container, Spacer, Text } from "@nextui-org/react";
import styles from "../styles/Home.module.css";

const Home: NextPage = () => {
  return (
    <>
      <Container
        css={{
          background: "$darkBg",
          height: "100vh",
          p: "0",
          m: "0",
          width: "100%",
          color: "$subText",
        }}
        className={styles.text}
      >
        <Card
          css={{
            background: "$headBg",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            height: "70%",
            width: "100%",
          }}
          className={styles.headCard}
        >
          <Text
            size={40}
            css={{ fontWeight: 700, color: "$headText" }}
            className={styles.text}
          >
            Night Protocol
          </Text>
          A blockchain protocol written in Rust
          <Spacer y={1} />
          <Button color="gradient" auto shadow size="lg">
            Get started{" "}
          </Button>
        </Card>
      </Container>
    </>
  );
};

export default Home;
