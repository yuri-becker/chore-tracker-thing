import { Button } from '@ariakit/react'
import { LogOut, Plus, Settings, X } from 'react-feather'
import { NavLink } from 'react-router'
import { useUser } from '../use-user.tsx'

export const MainMenu = (props: { isMenuOpen:boolean, setIsMenuOpen: (isMenuOpen:boolean) => void }) => {
  const user = useUser()
  return <aside className={props.isMenuOpen ? 'open main-menu' : 'main-menu'}>
    <div>
      <Button className={'outline secondary flat'} onClick={() => props.setIsMenuOpen(!props.isMenuOpen)}>
        <X />
      </Button>
    </div>
    <nav>
      <ul>
        <li>
          <Plus />&nbsp;Create Task
        </li>
        <li>
          <NavLink to="/settings"><Settings />&nbsp;Settings</NavLink>
        </li>
        <li>
          <a className="contrast" href="/oidc/logout"><LogOut />&nbsp;Logout {user!.name}</a>
        </li>
      </ul>
    </nav>
  </aside>
}
