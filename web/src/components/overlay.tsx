import classNames from 'classnames'
import { useCallback, useContext } from 'react'
import { MainMenuContext } from '../global/main-menu.context.tsx'
import classes from './overlay.module.css'

export const Overlay = () => {
  const { isMenuOpen, setIsMenuOpen } = useContext(MainMenuContext)
  const closeMenu = useCallback(() => {
    setIsMenuOpen(false)
  }, [setIsMenuOpen])
  return (
    <div aria-hidden className={classNames(classes.overlay, { [classes.hidden]: !isMenuOpen })} onClick={closeMenu} />
  )
}
