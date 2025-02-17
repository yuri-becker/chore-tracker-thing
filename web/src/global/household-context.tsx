import { createContext, useContext } from 'react'
import { Household } from '../domain/household.tsx'

interface HouseholdContextType {
  households: Household[] | undefined,
  addHousehold: (household: Household) => void,
}
export const HouseholdContext = createContext<HouseholdContextType>(null!)

export const useHouseholdContext = () => useContext(HouseholdContext)
