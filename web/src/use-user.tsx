import { useCallback, useEffect, useState } from 'react'
import Cookies from 'js-cookie'

export interface User {
  name: string
}

const parseCookie = () => {
  const cookie = Cookies.get('user')
  if (cookie === undefined) return undefined
  return JSON.parse(cookie) as User
}

export const useUser = () => {
  const [user, setUser] = useState<User | undefined>(parseCookie)
  const storageListener = useCallback((event: StorageEvent) => {
    if (event.key === 'cookieSync') {
      setUser(parseCookie())
    }
  }, [setUser])

  useEffect(() => {
    window.addEventListener('storage', storageListener)
    return () => {
      window.removeEventListener('storage', storageListener)
    }
  }, [storageListener])
  return user
}