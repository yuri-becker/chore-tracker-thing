import { Button } from '@ariakit/react'
import '@picocss/pico/css/pico.css'
import HouseholdForm from './household-form.tsx'
import { useUser } from './use-user.tsx'

function App () {
  const user = useUser()

  return (
    <>
      {!user && <a href="/oidc/login">
        <Button>Login</Button>
      </a>}
      {user && <>
        <a href="/oidc/logout">
          <Button>Logout {user.name}</Button>
        </a>
        <HouseholdForm />
      </>
      }

    </>
  )
}

export default App
