import { useHousehold } from '../../hooks/use-household.tsx'
import CreateTaskPage from '../create-task'

const Page = () => {
  const current = useHousehold()
  return (<>
      <div>{current?.name}</div>
      <CreateTaskPage />
    </>
  )
}

export default Page
