import { ReactNode, useEffect, useState } from 'react'
import { Household } from '../domain/household.tsx'
import { useApi } from '../hooks/api/use-api.tsx'
import { HouseholdContext } from './household-context.tsx'

export const HouseholdContextProvider = ({ children }: {
  children: ReactNode
}) => {
  const api = useApi('/household')
  const [households, setHouseholds] = useState<Household[] | undefined>()
  useEffect(() => {
    api().get().json<Household[]>().then(setHouseholds)
  }, [api, setHouseholds])

  return <HouseholdContext.Provider value={{
    households,
    addHousehold: (household) => {
      setHouseholds(prev => [...(prev ?? []), household])
    }
  }}>
    {children}
  </HouseholdContext.Provider>
}
