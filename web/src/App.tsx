import { AppShell, Burger, Group } from '@mantine/core'
import { useDisclosure } from '@mantine/hooks'
import { BrowserRouter, Route, Routes } from 'react-router'
import { MainMenu } from './components/main-menu.tsx'
import { HouseholdContextProvider } from './global/household-context.provider.tsx'
import { useUser } from './global/use-user.tsx'
import CreateHouseholdPage from './pages/create-household'
import CreateTaskPage from './pages/create-task'
import HouseholdPage from './pages/household'
import LoginPage from './pages/login'
import SettingsPage from './pages/settings/'

function App () {
  const user = useUser()
  const [mobileOpened, { toggle: toggleMobile, close: closeMobile }] = useDisclosure()
  return (
    <BrowserRouter>
      { !user && <LoginPage />}

      { user &&
        <HouseholdContextProvider>
          <AppShell
            header={{ height: 60 }}
            navbar={{
              width: 300,
              breakpoint: 'sm',
              collapsed: { mobile: !mobileOpened, desktop: false }
            }}
            padding="md"
          >
            <AppShell.Header>
              <Group h="100%" px="md">
                <Burger opened={mobileOpened} onClick={toggleMobile} hiddenFrom="sm" size="sm" />
                Chore Tracker Thing
              </Group>
            </AppShell.Header>
            <AppShell.Navbar p="md">
              <MainMenu closeMobile={closeMobile} />
            </AppShell.Navbar>
            <AppShell.Main>
              Main
              <Routes>
                <Route path="/settings" element={<SettingsPage />} />
                <Route index path="/" element={<CreateHouseholdPage />} />
                <Route path='/household/:householdId' element={<HouseholdPage />} />
                <Route path="/household/:householdId/create-task" element={<CreateTaskPage />} />
              </Routes>
            </AppShell.Main>
          </AppShell>
        </HouseholdContextProvider>
      }
    </BrowserRouter>
  )
}

export default App
