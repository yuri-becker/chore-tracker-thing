import { Button } from '@ariakit/react'
import { useContext } from 'react'
import { LogOut, Plus, Settings, X } from 'react-feather'
import { NavLink } from 'react-router'
import { MainMenuContext } from '../global/main-menu.context.tsx'
import { useUser } from '../global/use-user.tsx'

export const MainMenu = () => {
  const { isMenuOpen, setIsMenuOpen } = useContext(MainMenuContext)
  const user = useUser()
  return <aside className={isMenuOpen ? 'open main-menu' : 'main-menu'}>
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
          <NavLink className="contrast" to="/settings"><Settings />&nbsp;Settings</NavLink>
        </li>
        <li>
          <a className="contrast" href="/oidc/logout"><LogOut />&nbsp;Logout {user!.name}</a>
        </li>
      </ul>
    </nav>
  </aside>
}
