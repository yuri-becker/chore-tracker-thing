import { ReactNode, useEffect, useState } from 'react'
import { Household } from '../domain/household.tsx'
import { HouseholdContext } from './household-context.tsx'

export const HouseholdContextProvider = ({ children }: { children: ReactNode }) => {
  const [households, setHouseholds] = useState<Household[]>()
  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/no-floating-promises
    fetch('/api/household').then(res => res.json()).then(households => setHouseholds(households as Household[])).catch()
  }, [])
  return <HouseholdContext.Provider value={households}>
    {children}
  </HouseholdContext.Provider>
}
