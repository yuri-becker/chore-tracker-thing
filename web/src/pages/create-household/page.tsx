import { Button, TextInput } from '@mantine/core'
import { useForm } from '@mantine/form'
import { useCallback } from 'react'
import { Household } from '../../domain/household.tsx'
import { useHouseholdContext } from '../../global/household-context.tsx'
import { useApi } from '../../hooks/api/use-api.tsx'

const Page = () => {
  const api = useApi('/household')
  const { addHousehold } = useHouseholdContext()
  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      name: ''
    },

    validate: {
      name: (value) => (value.length === 0 ? 'Name is required' : null)
    }
  })

  const handleSubmit = useCallback(
    (values: typeof form.values) => api()
      .post(values)
      .json<Household>()
      .then(addHousehold),
    [api, addHousehold, form]
  )

  return (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <TextInput
        label="Name"
        placeholder="Miya's Household"
        key={form.key('name')}
        {...form.getInputProps('name')}
      />
      <Button loading={form.submitting} type="submit">Add</Button>
    </form>
  )
}

export default Page
