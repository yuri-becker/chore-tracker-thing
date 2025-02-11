import { Button, TextInput } from '@mantine/core'
import { useForm } from '@mantine/form'

const Page = () => {
  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      name: ''
    },

    validate: {
      name: (value) => (value.length === 0 ? 'Name is required' : null)
    }
  })

  const handleSubmit = async (values: typeof form.values) => {
    await fetch('/api/household', {
      method: 'POST',
      body: JSON.stringify(values)
    })
  }

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
