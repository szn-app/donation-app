import React, {
  createContext,
  useState,
  PropsWithChildren,
  useEffect,
} from "react";
import { AuthProviderProps, AuthProvider, useAuth } from "react-oidc-context";
import { Log, WebStorageStateStore, UserManagerSettings } from "oidc-client-ts";
import { User } from "@/types/user";
import { useNavigate } from "@tanstack/react-router";
import { handleSigninCallback, handleSignoutCallback } from "@/modules/auth";

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

// NOTE: implicit OIDC config added: https://auth.donation-app.local/authorize/.well-known/openid-configuration
// https://auth.donation-app.local/authorize/.well-known/jwks.json
// https://github.com/authts/oidc-client-ts/blob/main/docs/protocols/authorization-code-grant-with-pkce.md
// https://github.com/authts/react-oidc-context/blob/main/src/AuthProvider.tsx
// https://github.com/authts/react-oidc-context/blob/fba9a553ed3b24c45d5b03c0bedc6a94d73b7fb6/docs/react-oidc-context.api.md?plain=1#L88
export const oidcConfig: AuthProviderProps = {
  userStore: new WebStorageStateStore({ store: window.localStorage }), // persist user session in localStorage to enable silently re-establishing session on reload or new tab
  authority: import.meta.env.VITE_AUTHORIZATION_URL, // Ory Hydra server URL
  client_id: "frontend-client", // OIDC client ID
  redirect_uri: `${import.meta.env.VITE_APP_URL}/callback/login`, // redirect URI
  response_type: "code", // NOTE: "code id_token" response type doesn't seem to be supported by react-oidc-context (thus id_token can be fetched from userinfo endpoint with additional request instead)
  scope: "offline_access openid email profile", // OIDC scopes
  loadUserInfo: true,
  filterProtocolClaims: true,
  revokeTokensOnSignout: false, // NOTE: revokation requires server modification to expose endpoint for revoking the token
  automaticSilentRenew: true,
  silent_redirect_uri: "${import.meta.env.VITE_APP_URL}/api/oauth2_token",
  post_logout_redirect_uri: `${import.meta.env.VITE_APP_URL}/callback/logout`,
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
  // Enable storage events to synchronize between tabs
  monitorSession: true,

  /**
   * gets execute every time when react-oidc-context gets loaded
   *
   * Determines if the current URL matches the post_logout_redirect redirect URI, then calls handleSignoutCallback accordingly
   * @param args - The UserManagerSettings containing configuration
   * @returns boolean indicating if current URL matches logout redirect URI
   */
  matchSignoutCallback: (args: UserManagerSettings): boolean => {
    return !!(
      window &&
      args.post_logout_redirect_uri &&
      window.location.href === args.post_logout_redirect_uri
    );
  },

  onSignoutCallback: handleSignoutCallback,
  // executed when react-oidc-context finishes a signin and user is now authenticated
  onSigninCallback: handleSigninCallback,
  onRemoveUser: () => {
    console.log("OIDC user removed from storage");
    return Promise.resolve();
  },
};

const REQUIRED_ENV_VARS = [
  "VITE_AUTHORIZATION_URL",
  "VITE_AUTHENTICATION_URL",
  "VITE_APP_URL",
  "VITE_AUTH_BASE_URL",
  "VITE_DOMAIN_NAME",
  "VITE_GRAPHQL_ENDPOINT",
  "VITE_REST_ENDPOINT",
];

// validate that environment variables are set
function validateRequiredEnv() {
  const missing = REQUIRED_ENV_VARS.filter((key) => {
    const val = import.meta.env[key];
    return val === undefined || val === null || val === "";
  });

  if (missing.length > 0) {
    throw new Error(
      `Missing crucial environment variables: ${missing.join(", ")}`,
    );
  }
}

export function AuthContextProvider({ children }: PropsWithChildren<{}>) {
  useEffect(() => {
    if (import.meta.env.DEV) {
      Log.setLogger(console);
      Log.setLevel(Log.DEBUG);
    }
  }, []);

  validateRequiredEnv();

  return <AuthProvider {...oidcConfig}>{children}</AuthProvider>;
}
