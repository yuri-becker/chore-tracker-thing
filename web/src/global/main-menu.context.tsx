import { createContext } from 'react'

export interface mainMenuContextType {
  isMenuOpen: boolean,
  setIsMenuOpen: (isMenuOpen: boolean) => void,
  toggleMenuOpen: () => void
}

export const MainMenuContext = createContext<mainMenuContextType>(null!)
