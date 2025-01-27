type RecurrenceUnit = 'Days | Weeks | Months'

export interface Task {
  title: string,
  recurrenceUnit: RecurrenceUnit,
  recurrenceInterval: number
}
