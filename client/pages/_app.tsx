import "../styles/globals.css";
import type { AppProps } from "next/app";
import { createTheme, NextUIProvider, Text } from "@nextui-org/react";

const theme = createTheme({
  type: "dark",
  theme: {
    colors: {
      // brand colors
      primaryLight: "$green200",
      primary: "#4ADE7B",
      primaryDark: "$green600",
      darkBg: "#121212",
      headBg: "#101429",
      headText: "#e8ebe9",
      subText: "#d9dbda",
    },
    space: {},
    fonts: {},
  },
});

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <NextUIProvider theme={theme}>
      <Component {...pageProps} />
    </NextUIProvider>
  );
}

export default MyApp;
