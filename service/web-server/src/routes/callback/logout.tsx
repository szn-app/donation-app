import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useAuth } from "react-oidc-context";
import { useEffect } from "react";
import { debug, handleRedirectAfterCallback } from "@/utility/auth";

// example usage of react-oidc-context https://github.com/authts/react-oidc-context?tab=readme-ov-file
export const Route = createFileRoute("/callback/logout")({
  component,
  // component: debug.auth_state_component, // used for development debugging
});

export function component() {
  const auth = useAuth();
  const navigate = useNavigate();

  /**
   * NOTE: Kratos redirects back to application and invalidates session cookie (which cannot be managed by Javascript because of the http-only flag)
   */

  // Check if this is a return from Kratos logout
  useEffect(() => {
    if (import.meta.env.DEV) {
      console.log(auth);
    }

    if (
      auth.error ||
      auth.activeNavigator != "signinRedirect" ||
      !auth.isLoading
    ) {
      handleRedirectAfterCallback(navigate, 5);
    }
  }, [auth, navigate]);

  if (auth.error) {
    return <div>Sign-in error... {auth.error.message}</div>;
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
