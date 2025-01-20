import { Button } from '@ariakit/react'
import { useContext } from 'react'
import { Menu } from 'react-feather'
import { MainMenuContext } from '../global/main-menu.context.tsx'

export const Header = () => {
  const { toggleMenuOpen } = useContext(MainMenuContext)
  return <header>
    <Button id="burger-menu" className={'outline secondary flat'}
            onClick={() => toggleMenuOpen() }>
      <Menu />
    </Button>
  </header>
}
