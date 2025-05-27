import { useMemo} from 'react';
import { useAuth } from 'react-oidc-context';

export function useAuthHeaders() {
  const auth = useAuth();
  const token = auth?.user?.access_token;

  const headers = useMemo<Record<string, string> | undefined>(() => 
    token ? { Authorization: `Bearer ${token}` } : undefined,
    [token]
  );

  return {
    headers,
    isAuthenticated: auth.isAuthenticated,
    token
  };
} 