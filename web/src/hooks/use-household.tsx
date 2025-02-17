import { useMemo } from 'react'
import { useParams } from 'react-router'
import { useHouseholdContext } from '../global/household-context.tsx'

export const useHousehold = () => {
  const params = useParams<{householdId: string}>()
  const { households } = useHouseholdContext()
  return useMemo(() => households?.find((e) => e.id === params.householdId),
    [params, households])
}
