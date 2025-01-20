import '@picocss/pico/css/pico.css'
import { BrowserRouter, Route, Routes } from 'react-router'
import { Header } from './components/header.tsx'
import { MainMenu } from './components/main-menu.tsx'
import { HouseholdContextProvider } from './global/household-context.provider.tsx'
import { MainMenuContextProvider } from './global/main-menu-context.provider.tsx'
import { useUser } from './global/use-user.tsx'
import CreateHouseholdPage from './pages/create-household'
import LoginPage from './pages/login'
import SettingsPage from './pages/settings/'

function App () {
  const user = useUser()
  return (
    <BrowserRouter>
      { !user && <LoginPage />}

      {user &&
        <MainMenuContextProvider>
          <HouseholdContextProvider>
            <MainMenu />
            <main className="container">
              <Header />
              <Routes>
                <Route path="/settings" element={<SettingsPage />} />
                <Route index path="/" element={<CreateHouseholdPage />} />
              </Routes>
            </main>
          </HouseholdContextProvider>
        </MainMenuContextProvider>
      }
    </BrowserRouter>
  )
}

export default App
