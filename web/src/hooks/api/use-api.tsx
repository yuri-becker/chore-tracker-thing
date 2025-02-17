import { notifications } from '@mantine/notifications'
import { useCallback, useEffect, useState } from 'react'
import { AlertOctagon } from 'react-feather'
import originalWretch from 'wretch'
import { useUser } from '../../global/use-user.tsx'

export function useApi (url: string) {
  const [, clearUser] = useUser()
  return useCallback(
    () => originalWretch(`/api${url}`)
      .catcher(401, () => {
        clearUser()
      }).catcher(403, () => {
        notifications.show({
          title: 'Not allowed to view this page!',
          message: 'You are no longer a member of this household or never were.',
          icon: <AlertOctagon />,
          position: 'bottom-center',
          color: 'red',
          autoClose: false
        })
      }).catcherFallback(() => {
        notifications.show({
          title: 'Something just broke...',
          message: <>If this keeps happening, <a
          href="https://github.com/yuri-becker/chore-tracker-thing/issues/new">please
          report an issue</a>.</>,
          icon: <AlertOctagon />,
          position: 'bottom-center',
          color: 'red',
          autoClose: false
        })
      }),
    [clearUser, url]
  )
}

export function useGet<Response> (url: string) {
  const api = useApi(url)
  const [data, setData] = useState<Response | undefined>()
  useEffect(() => {
    api().get().json<Response>().then(setData)
  }, [api])
  return data
}
