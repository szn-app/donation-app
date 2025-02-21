import React, {
  createContext,
  useState,
  PropsWithChildren,
  useEffect,
} from "react";
import { AuthProviderProps, AuthProvider, useAuth } from "react-oidc-context";
import { Log, WebStorageStateStore } from "oidc-client-ts";
import { User } from "@/types/user";
import { useNavigate } from "@tanstack/react-router";

interface UserContextType {
  user: User;
  setUser: React.Dispatch<React.SetStateAction<User>>;
}

export const defaultUser: User = {
  name: "",
  email: "",
  avatar: "",
};

export const UserContext = createContext<UserContextType>({
  user: defaultUser,
  setUser: () => {},
});

export function UserProvider({ children }: PropsWithChildren<{}>) {
  const [user, setUser] = useState<User>(defaultUser);

  return (
    <UserContext.Provider value={{ user, setUser }}>
      {children}
    </UserContext.Provider>
  );
}

// NOTE: implicit OIDC config added: https://auth.donation-app.test/authorize/.well-known/openid-configuration
// https://auth.donation-app.test/authorize/.well-known/jwks.json
// https://github.com/authts/oidc-client-ts/blob/main/docs/protocols/authorization-code-grant-with-pkce.md
export const oidcConfig: AuthProviderProps = {
  userStore: new WebStorageStateStore({ store: window.localStorage }), // persist user session in localStorage to enable silently re-establishing session on reload or new tab
  authority: import.meta.env.VITE_AUTHORIZATION_URL, // Ory Hydra server URL
  client_id: "frontend-client", // OIDC client ID
  redirect_uri: `${import.meta.env.VITE_APP_URL}/callback`, // redirect URI
  response_type: "code", // NOTE: "code id_token" response type doesn't seem to be supported by react-oidc-context (thus id_token can be fetched from userinfo endpoint with additional request instead)
  scope: "offline_access openid email profile", // OIDC scopes
  loadUserInfo: true,
  filterProtocolClaims: true,
  revokeTokensOnSignout: false, // TODO: error is thrown when set to true after logout
  automaticSilentRenew: true,
  silent_redirect_uri: "${import.meta.env.VITE_APP_URL}/api/oauth2_token",
  post_logout_redirect_uri: `${import.meta.env.VITE_APP_URL}`,
  disablePKCE: false,
  // customize token endpoint to pass code for app backend exchange
  metadata: {
    authorization_endpoint: `${import.meta.env.VITE_AUTHORIZATION_URL}/oauth2/auth`, // Authorization endpoint
    // token_endpoint: "${import.meta.env.VITE_AUTH_BASE_URL}/authorize/oauth2/token", // Token endpoint
    token_endpoint: `${import.meta.env.VITE_APP_URL}/api/oauth2_token`, // custom Token endpoint which stores secrets
    userinfo_endpoint: `${import.meta.env.VITE_AUTHORIZATION_URL}/userinfo`, // UserInfo endpoint
    end_session_endpoint: `${import.meta.env.VITE_AUTHORIZATION_URL}/oauth2/sessions/logout`, // End session endpoint
    jwks_uri: `${import.meta.env.VITE_AUTHORIZATION_URL}/.well-known/jwks.json`, // JWKS endpoint
    issuer: `${import.meta.env.VITE_AUTHORIZATION_URL}/`, // Issuer URL
    revocation_endpoint: `${import.meta.env.VITE_APP_URL}/api/oauth2_revoke`, // token revocation endpoint
  },
  extraQueryParams: {},

  /**
   * removes code and state from url after signin
   * see https://github.com/authts/react-oidc-context/blob/f175dcba6ab09871b027d6a2f2224a17712b67c5/src/AuthProvider.tsx#L20-L30
   */
  onSigninCallback: () => {
    window.history.replaceState({}, document.title, window.location.pathname);
  },

  onSignoutCallback: () => {
    console.log("onSignoutCallback");

    const navigate = useNavigate();
    const auth = useAuth();

    Object.keys(window.localStorage).forEach((key) => {
      if (key.startsWith("oidc.user:")) {
        window.localStorage.removeItem(key);
      }
    });

    auth.removeUser();

    const redirectUrl = localStorage.getItem("redirectUrl");
    if (redirectUrl) {
      localStorage.removeItem("redirectUrl");
      navigate({ to: redirectUrl });
    } else {
      navigate({ to: "/" });
    }
  },
};

export function AuthContextProvider({ children }: PropsWithChildren<{}>) {
  useEffect(() => {
    // TODO:
    Log.setLogger(console);
    Log.setLevel(Log.DEBUG);
  }, []);

  return <AuthProvider {...oidcConfig}>{children}</AuthProvider>;
}
