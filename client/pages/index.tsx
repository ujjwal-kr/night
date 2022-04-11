import type { NextPage } from "next";
import styles from "../styles/Home.module.css";
import { Text, Title, Button } from "@mantine/core";
import Particles from "react-tsparticles";

const Home: NextPage = () => {
  return (
    <>
      <Top_Container />
    </>
  );
};

const Top_Container = () => {
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
        
        <Particles
          params={{
            "particles": {
              "number": {
                "value": 160,
                "density": {
                  "enable": true,
                  "value_area": 1500
                }
              },
              "line_linked": {
                "enable": true,
                "opacity": 0.02
              },
              "move": {
                "direction": "right",
                "speed": 2
              },
              "size": {
                "value": 2
              },
              "opacity": {
                "anim": {
                  "enable": true,
                  "speed": 1,
                  "opacity_min": 0.05
                }
              }
            },
            "interactivity": {
              "events": {
                "onclick": {
                  "enable": false,
                }
              },
              "modes": {
                "push": {
                  "particles_nb": 1
                }
              }
            },
            "retina_detect": true
          }} />
      </div>
    </>
  )
}

export default Home;
