import { NavLink, Space } from '@mantine/core'
import { LogOut, Plus, Settings } from 'react-feather'
import { NavLink as RouterLink } from 'react-router'
import { useHouseholdContext } from '../global/household-context.tsx'
import { useUser } from '../global/use-user.tsx'
import classes from './main-menu.module.css'

interface MainMenuProps {closeMobile?: () => void}

export const MainMenu = ({ closeMobile }: MainMenuProps) => {
  const { households } = useHouseholdContext()
  const [user] = useUser()
  return (
    <>
      { !households && <li aria-busy="true"></li>}
      {households && households.length >= 2 && households.map(household =>
        <NavLink
          component={RouterLink}
          key={household.id}
          label={household.name}
          to={`/household/${household.id}`}
          onClick={closeMobile} />)
      }
      <Space className={classes.space} />
      <NavLink
        component={RouterLink}
        to={'/tasks'}
        label="Create Task"
        leftSection={<Plus />}
        onClick={closeMobile} />
      <NavLink
        component={RouterLink}
        to="/settings"
        label="Settings"
        leftSection={<Settings />}
        onClick={closeMobile} />
      <NavLink
        href="/oidc/logout"
        label={`Logout ${user!.name}`}
        leftSection={<LogOut />}
      />
    </>
  )
}
