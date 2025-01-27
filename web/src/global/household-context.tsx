import { createContext } from 'react'
import { Household } from '../domain/household.tsx'

export const HouseholdContext = createContext<Household[] | undefined>(null!)
