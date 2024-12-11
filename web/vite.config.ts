import { defineConfig, loadEnv, UserConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import * as path from 'node:path'

export default ({ mode }: UserConfig) => {
  const projectRoot = path.join(process.cwd(), '../')
  const envPrefix = 'CHORES'
  const env = loadEnv(mode!, projectRoot, envPrefix)
  const backendUrl = `http://127.0.0.1:${env.CHORES_PORT ?? '8001'}`

  return defineConfig({
    plugins: [react()],
    envDir: projectRoot,
    envPrefix: envPrefix,
    server: {
      host: "127.0.0.1",
      port: parseInt(env["CHORES_VITE_PORT"]),
      proxy: {
        '/oidc': { target: backendUrl },
        '/api': { target: backendUrl },
      },
    },
  })
}

