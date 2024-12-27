import * as Ariakit from '@ariakit/react'
import '@picocss/pico/css/pico.css'

export default function HouseholdForm () {
  const form = Ariakit.useFormStore({ defaultValues: { name: '' } })

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
    <Ariakit.FormProvider>
      <Ariakit.Form store={form}>
        <h2>New Household</h2>
        <div>
          <Ariakit.FormLabel name={form.names.name} >Household Name</Ariakit.FormLabel>
          <Ariakit.FormInput name={form.names.name} placeholder="Miya's House" />
          <Ariakit.FormError name={form.names.name} />
        </div>
        <div >
          <Ariakit.FormSubmit disabled={!!form.getError('name') || form.getState().submitting} aria-busy={form.getState().submitting}>
            Add
          </Ariakit.FormSubmit>
        </div>
      </Ariakit.Form>
    </Ariakit.FormProvider>
  )
}
