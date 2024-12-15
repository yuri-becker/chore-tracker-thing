import { Button } from '@ariakit/react'
import { useUser } from './use-user.tsx'
import '@picocss/pico/css/pico.css'

function App () {
  const user = useUser()

  return (
    <>
      {!user && <a href="/oidc/login">
          <Button >Login</Button>
      </a>}
      {user && <a href="/oidc/logout">
        <Button >Logout {user.name}</Button>
      </a>}
    </>
  )
}

export default App
