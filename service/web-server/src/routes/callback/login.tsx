import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useAuth } from "react-oidc-context";
import { useEffect } from "react";
import { debug, handleRedirectAfterCallback } from "@/lib/auth";

// example usage of react-oidc-context https://github.com/authts/react-oidc-context?tab=readme-ov-file
export const Route = createFileRoute("/callback/login")({
  component,
  // component: debug.auth_state_component, // used for development debugging
});

export function component() {
  const auth = useAuth();
  const navigate = useNavigate();

  // Check if this is a return from Kratos logout
  useEffect(() => {
    if (import.meta.env.DEV) {
      console.log(auth);
    }

    /**
     * NOTE: react-oidc-context loads and detects URI parameters for OIDC process and signs user in with token exchange flow
     */

    if (
      auth.error ||
      auth.activeNavigator != "signoutRedirect" ||
      !auth.isLoading
    ) {
      handleRedirectAfterCallback(navigate);
    }
  }, [auth, navigate]);

  if (auth.error) {
    return <div>Sign-out error... {auth.error.message}</div>;
  } else if (auth.isLoading) {
    return <div>Loading...</div>;
  } else {
    return (
      <div>
        Redirecting...
        {auth.isAuthenticated && <div>Hello {auth.user?.profile.sub}</div>}
      </div>
    );
  }
}
