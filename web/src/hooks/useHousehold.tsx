import { useContext, useMemo } from 'react'
import { useParams } from 'react-router'
import { HouseholdContext } from '../global/household-context.tsx'

export const useHousehold = () => {
  const params = useParams<{householdId: string}>()
  const households = useContext(HouseholdContext)
  return useMemo(() => households?.find((e) => e.id === params.householdId),
    [params, households])
}
