import { Button } from '@ariakit/react'
import classNames from 'classnames'
import { useCallback, useContext } from 'react'
import { LogOut, Plus, Settings, X } from 'react-feather'
import { NavLink } from 'react-router'
import { HouseholdContext } from '../global/household-context.tsx'
import { MainMenuContext } from '../global/main-menu.context.tsx'
import { useUser } from '../global/use-user.tsx'
import classes from './main-menu.module.css'

export const MainMenu = () => {
  const { isMenuOpen, setIsMenuOpen } = useContext(MainMenuContext)
  const households = useContext(HouseholdContext)
  const user = useUser()
  const closeMenu = useCallback(() => {
    setIsMenuOpen(false)
  }, [setIsMenuOpen])
  return <aside className={classNames(classes.mainMenu, { [classes.open]: isMenuOpen })}>
    <div>
      <Button className={'outline secondary flat'} onClick={closeMenu}>
        <X />
      </Button>
    </div>
    <nav>
      <ul>
        {!households && <li aria-busy='true'></li>}
        {households && households.length >= 2 && households.map(household => <li key={household.id}>
          <NavLink to={`/household/${household.id}`} onClick={closeMenu} >
            {household.name}
          </NavLink>
        </li>)}
      </ul>
      <ul>
        <li>
          <Plus />&nbsp;Create Task
        </li>
        <li>
          <NavLink className="contrast" to="/settings" onClick={closeMenu}><Settings />&nbsp;Settings</NavLink>
        </li>
        <li>
          <a className="contrast" href="/oidc/logout"><LogOut />&nbsp;Logout {user!.name}</a>
        </li>
      </ul>
    </nav>
  </aside>
}
