import '@picocss/pico/css/pico.css'
import { Button } from '@ariakit/react'
import { useState } from 'react'
import { Menu, LogOut, Settings, Plus, X } from 'react-feather'
import HouseholdForm from './household-form.tsx'
import { useUser } from './use-user.tsx'

function App () {
  const user = useUser()
  const [isMenuOpen, setIsMenuOpen] = useState<boolean>(false)

  return (
    <>
      {!user && <>
        <a href="/oidc/login">
          <Button>Login</Button>
        </a>
      </>}

      {user && <>
        <aside className={isMenuOpen ? 'open main-menu' : 'main-menu'}>
          <div>
            <Button className={'outline secondary flat'} onClick={() => setIsMenuOpen(!isMenuOpen)}>
              <X />
            </Button>
          </div>
          <nav>
            <ul>
              <li>
                <Plus />&nbsp;Create Task
              </li>
              <li>
                <Settings />&nbsp;Settings
              </li>
              <li>
                <a className="contrast" href="/oidc/logout"><LogOut />&nbsp;Logout {user.name}</a>
              </li>
            </ul>
          </nav>
        </aside>
        <main className="container">
          <header>
            <Button id="burger-menu" className={'outline secondary flat'} onClick={() => setIsMenuOpen(!isMenuOpen)}>
              <Menu />
            </Button>
          </header>
          <HouseholdForm />
        </main>
      </>
      }
    </>
  )
}

export default App
