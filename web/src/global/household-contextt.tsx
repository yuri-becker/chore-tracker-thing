import { createContext } from 'react'
import { household } from '../domain/household.tsx'

export const HouseholdContext = createContext<household[] | undefined>(null!)
