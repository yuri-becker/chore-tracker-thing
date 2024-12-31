import '@picocss/pico/css/pico.css'
import { useState } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router'
import { Header } from './components/header.tsx'
import { MainMenu } from './components/main-menu.tsx'
import CreateHouseholdPage from './pages/create-household'
import LoginPage from './pages/login'
import SettingsPage from './pages/settings/'
import { useUser } from './use-user.tsx'

function App () {
  const user = useUser()
  const [isMenuOpen, setIsMenuOpen] = useState<boolean>(false)
  return (
    <BrowserRouter>
      { !user && <LoginPage />}

      {user && <>
        <MainMenu isMenuOpen={isMenuOpen} setIsMenuOpen={setIsMenuOpen} />
        <main className="container">
          <Header setIsMenuOpen={setIsMenuOpen} isMenuOpen={isMenuOpen} />
          <Routes>
            <Route path="/settings" element={<SettingsPage />} />
            <Route index path="/" element={<CreateHouseholdPage />} />
          </Routes>
        </main>
      </>
      }
    </BrowserRouter>
  )
}

export default App
