import { Form, FormError, FormInput, FormLabel, FormProvider, FormSubmit, useFormStore } from '@ariakit/react/form'
import '@picocss/pico/css/pico.css'

const Page = () => {
  const form = useFormStore({ defaultValues: { name: '' } })

  form.useSubmit(async (state) => {
    await fetch('/api/household', {
      method: 'POST',
      body: JSON.stringify(state.values)
    })
  })

  form.useValidate(store => {
    if (store.values.name.length === 0) {
      store.errors.name = 'Name is required'
    }
  })

  return (
    <FormProvider>
      <Form store={form}>
        <h2>New Household</h2>
        <div>
          <FormLabel name={form.names.name} >Household Name</FormLabel>
          <FormInput name={form.names.name} placeholder="Miya's House" />
          <FormError name={form.names.name} />
        </div>
        <div >
          <FormSubmit disabled={!!form.getError('name') || form.getState().submitting} aria-busy={form.getState().submitting}>
            Add
          </FormSubmit>
        </div>
      </Form>
    </FormProvider>
  )
}

export default Page
