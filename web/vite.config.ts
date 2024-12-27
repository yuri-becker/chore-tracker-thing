import * as path from 'node:path'
import react from '@vitejs/plugin-react-swc'
import { defineConfig, loadEnv, UserConfig } from 'vite'

export default ({ mode }: UserConfig) => {
  // eslint-disable-next-line no-undef
  const projectRoot = path.join(process.cwd(), '../')
  const envPrefix = 'CHORES'
  const env = loadEnv(mode!, projectRoot, envPrefix)
  const backendUrl = `http://127.0.0.1:${env.CHORES_PORT ?? '8001'}`

  return defineConfig({
    plugins: [react()],
    envDir: projectRoot,
    envPrefix,
    server: {
      host: '127.0.0.1',
      port: parseInt(env.CHORES_VITE_PORT),
      proxy: {
        '/oidc': { target: backendUrl },
        '/api': { target: backendUrl }
      }
    }
  })
}
