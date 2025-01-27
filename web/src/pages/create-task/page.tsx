import { Form, FormError, FormInput, FormLabel, FormProvider, FormSubmit, useFormStore } from '@ariakit/react/form'
import { SelectItem } from '@ariakit/react/select'
import { FormSelect } from '../../components/form-select.tsx'
import { useHousehold } from '../../hooks/useHousehold.tsx'
import '@picocss/pico/css/pico.css'

const Page = () => {
  const form = useFormStore({ defaultValues: { title: '', recurrenceUnit: 'Days', recurrenceInterval: 1 } })
  const current = useHousehold()
  form.useSubmit(async (state) => {
    await fetch(`/api/household/${current!.id}/task`, {
      method: 'POST',
      body: JSON.stringify(state.values)
    })
  })

  form.useValidate(store => {
    if (store.values.title.length === 0) {
      store.errors.title = 'Name is required'
    }
    if (isNaN(parseInt(store.values.recurrenceInterval as unknown as string))) {
      store.errors.recurrenceInterval = 'Invalid interval - Must be numeric'
    } else {
      store.values.recurrenceInterval = parseInt(store.values.recurrenceInterval as unknown as string)
    }
  })

  return (
    <FormProvider>
      <Form store={form}>
        <h2>New Task</h2>
         <div>
          <FormLabel name={form.names.title}>Task Name</FormLabel>
          <FormInput name={form.names.title} placeholder="Vacuum the living room" />
          <FormError name={form.names.title} />
         </div>
         <div>
          <FormLabel name={form.names.recurrenceInterval}>Recurs every...</FormLabel>
          <FormInput name={form.names.recurrenceInterval} placeholder="1" />
          <FormError name={form.names.recurrenceInterval} />
         </div>
         <div>
          <FormSelect name={form.names.recurrenceUnit}>
            <SelectItem value="Days" />
            <SelectItem value="Weeks" />
            <SelectItem value="Months" />
          </FormSelect>
          <FormError name={form.names.title} />
         </div>
        <div>
          <FormSubmit disabled={ !!form.getError('name') || form.getState().submitting}
                      aria-busy={form.getState().submitting}>
            Add
          </FormSubmit>
        </div>
      </Form>
    </FormProvider>
  )
}

export default Page
