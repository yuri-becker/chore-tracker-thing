import { useHousehold } from '../../hooks/useHousehold.tsx'

const Page = () => {
  const current = useHousehold()
  return (
    <div>{current?.name}</div>
  )
}

export default Page
