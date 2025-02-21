import { createLazyFileRoute } from "@tanstack/react-router";
import { useAuth, withAuthenticationRequired } from "react-oidc-context";

function component() {
  return <div>Hello "/protected"!</div>;
}

const PrivateProtectedRoute = withAuthenticationRequired(component, {
  OnRedirecting: () => {
    // localStorage.setItem("redirectUrl", window.location.pathname);
    return <div>Redirecting to the login page...</div>;
  },
});

export const Route = createLazyFileRoute("/_app/protected")({
  component: PrivateProtectedRoute,
});
