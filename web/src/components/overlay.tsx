import { useCallback, useContext } from 'react'
import { MainMenuContext } from '../global/main-menu.context.tsx'
import './overlay.css'

export const Overlay = () => {
  const { isMenuOpen, setIsMenuOpen } = useContext(MainMenuContext)
  const closeMenu = useCallback(() => {
    setIsMenuOpen(false)
  }, [setIsMenuOpen])
  return (
    <div aria-hidden className={isMenuOpen ? 'overlay' : 'overlay hidden'} onClick={closeMenu} />
  )
}
