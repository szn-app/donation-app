import { useMemo} from 'react';
import { useAuth } from 'react-oidc-context';

export function useAuthHeaders() {
  const auth = useAuth();
  
  const headers = useMemo<Record<string, string> | undefined>(() => {
      const token = auth?.user?.access_token;
      return token ? { Authorization: `Bearer ${token}` } : undefined; 
    }, 
    [auth]
  );

  return {
    headers,
    isAuthenticated: auth.isAuthenticated
  };
} 