import { Button } from '@ariakit/react'
import { Menu } from 'react-feather'

export const Header = (props: { isMenuOpen: boolean, setIsMenuOpen: (isMenuOpen: boolean) => void }) => <header>
  <Button id="burger-menu" className={'outline secondary flat'} onClick={() => props.setIsMenuOpen(!props.isMenuOpen)}>
    <Menu />
  </Button>
</header>
