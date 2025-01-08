import { ReactNode, useState } from 'react'
import { MainMenuContext } from './main-menu.context.tsx'

export const MainMenuContextProvider = ({ children }: { children: ReactNode }) => {
  const [isMenuOpen, setIsMenuOpen] = useState<boolean>(false)
  return <MainMenuContext.Provider
    value={{ isMenuOpen, setIsMenuOpen, toggleMenuOpen: () => setIsMenuOpen(!isMenuOpen) }}>
    {children}
  </MainMenuContext.Provider>
}
