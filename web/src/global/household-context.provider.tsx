import { ReactNode, useEffect, useState } from 'react'
import { household } from '../domain/household.tsx'
import { HouseholdContext } from './household-contextt.tsx'

export const HouseholdContextProvider = ({ children }: { children: ReactNode }) => {
  const [households, setHouseholds] = useState<household[]>()
  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    fetch('/api/household').then(res => res.json()).then(households => setHouseholds(households as household[])).catch()
  }, [])
  return <HouseholdContext.Provider value={households}>
    {children}
  </HouseholdContext.Provider>
}
