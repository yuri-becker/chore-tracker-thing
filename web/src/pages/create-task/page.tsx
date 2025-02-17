import { Button, Group, NumberInput, Select, TextInput } from '@mantine/core'
import { useForm } from '@mantine/form'
import { useCallback } from 'react'
import { useApi } from '../../hooks/api/use-api.tsx'
import { useHousehold } from '../../hooks/use-household.tsx'

const Page = () => {
  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      title: '',
      recurrenceUnit: 'Weeks',
      recurrenceInterval: 1
    },
    validate: {
      title: (value) => (value.length === 0 ? 'Name is required' : null),
      recurrenceInterval: (value) => (isNaN(
        parseInt(value as unknown as string))
        ? 'Invalid interval - Must be numeric'
        : null)
    }
  })
  const current = useHousehold()
  const api = useApi(`/household/${current!.id}/task`)
  const submit = useCallback(
    (values: typeof form.values) => api().post(values).res(),
    [api, form]
  )
  return (
    <form onSubmit={form.onSubmit(submit)}>
      <TextInput
        label="Title"
        placeholder="Vacuum living room"
        key={form.key('title')}
        {...form.getInputProps('title')}
      />
      <Group>
        <NumberInput
          label="Recurs every..."
          placeholder="1"
          min={1}
          key={form.key('recurrenceInterval')}
          {...form.getInputProps('recurrenceInterval')}
        />
        <Select
          label="&nbsp;"
          data={['Days', 'Weeks', 'Months']}
          key={form.key('recurrenceUnit')}
          {...form.getInputProps('recurrenceUnit')}
        />
      </Group>
      <Button loading={form.submitting} type="submit">Add</Button>
    </form>
  )
}

export default Page
