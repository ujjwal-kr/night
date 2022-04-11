import "../styles/globals.css"
import type { AppProps } from "next/app"
import { MantineProvider, Button } from "@mantine/core";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <MantineProvider theme={{ colorScheme: "dark" }} withGlobalStyles >
      <Component {...pageProps} />
    </MantineProvider>
  )
}

export default MyApp
