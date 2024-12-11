import { useCallback, useEffect, useState } from 'react'

export const useIsLoggedIn = () => {
  const [isLoggedIn, setIsLoggedIn] = useState<boolean>(
    document.cookie.includes('logged-in='),
  )
  const storageListener = useCallback((event: StorageEvent) => {
    if (event.key === 'cookieSync') {
      setIsLoggedIn(document.cookie.includes('logged-in='))
    }
  }, [setIsLoggedIn])

  useEffect(() => {
    window.addEventListener("storage", storageListener);
    return () => {
      window.removeEventListener("storage", storageListener)
    }
  }, [storageListener])
  return isLoggedIn
}