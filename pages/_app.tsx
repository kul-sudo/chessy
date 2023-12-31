import type { NextPage } from 'next'
import type { AppProps } from 'next/app'
import Head from 'next/head'
import { MantineProvider } from '@mantine/core'
import { Inter } from 'next/font/google'
import '.././styles/globals.css'

const font = Inter({ subsets: ['latin'] })

const App: NextPage<AppProps> = ({ Component, pageProps }) => {
  return (
    <>
      <Head>
        <title>Chessy</title>
        <meta
          name="viewport"
          content="minimum-scale=1, initial-scale=1, width=device-width"
        />
      </Head>

      <MantineProvider
        defaultColorScheme="dark"
        theme={{
          fontFamily: font.style.fontFamily
        }}
      >
        <Component {...pageProps} />
      </MantineProvider>
    </>
  )
}

export default App
