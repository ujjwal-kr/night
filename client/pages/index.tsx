import type { NextPage } from "next";
import Link from "next/link";
import styles from "../styles/Home.module.css";
import { Button } from "@mantine/core";
import Particles from "react-tsparticles";
import Head from "next/head";
const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>Night Protocol</title>
        <meta
          name="description"
          content="A minimal blockchain protocol written in Rust for beginners to understand the modern infrastructure of blockchain."
        />
        <meta property="og:title" content="Night Protocol" />
        <meta
          property="og:description"
          content="A minimal blockchain protocol written in Rust for beginners to understand the modern infrastructure of blockchain."
        />
      </Head>
      <div className={styles.top_container}>
        <div>
          <h1 className={styles.heading}>Night Protocol</h1>
          <p className={styles.description}>
            A minimal blockchain protocol written in Rust for beginners to
            understand <br /> the modern infrastructure of blockchain.
          </p>
          <Link href="/explore">
            <Button
              size="lg"
              className={styles.text}
              variant="gradient"
              gradient={{ from: "orange", to: "red" }}
            >
              Explore
            </Button>
          </Link>

          <Link href="/dashboard">
            <Button
            style={{marginLeft: 1+'rem'}}
              size="lg"
              className={styles.text}
              variant="gradient"
              gradient={{ from: "orange", to: "red" }}
            >
              Dashboard
            </Button>
          </Link>
        </div>

        <Particles
          params={{
            particles: {
              number: {
                value: 140,
                density: {
                  enable: true,
                  value_area: 1500,
                },
              },
              line_linked: {
                enable: true,
                opacity: 0.02,
              },
              move: {
                direction: "right",
                speed: 2,
              },
              size: {
                value: 2,
              },
              opacity: {
                anim: {
                  enable: true,
                  speed: 1,
                  opacity_min: 0.05,
                },
              },
            },
            interactivity: {
              events: {
                onclick: {
                  enable: false,
                },
              },
              modes: {
                push: {
                  particles_nb: 1,
                },
              },
            },
            retina_detect: true,
          }}
        />
      </div>
    </>
  );
};

export default Home;
