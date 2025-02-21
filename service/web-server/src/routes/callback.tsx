import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useAuth } from "react-oidc-context";
import { useEffect } from "react";

// example usage of react-oidc-context https://github.com/authts/react-oidc-context?tab=readme-ov-file
export const Route = createFileRoute("/callback")({
  component,
});

export function component() {
  const auth = useAuth();
  const navigate = useNavigate();

  switch (auth.activeNavigator) {
    case "signinSilent":
      console.log("Silent signin...");
      return <div>Signing you in...</div>;
    case "signoutRedirect":
      console.log("Redirecting to signout...");

      // auth.removeUser();

      // const redirectUrl = localStorage.getItem("redirectUrl");
      // if (redirectUrl) {
      //   localStorage.removeItem("redirectUrl");
      //   navigate({ to: redirectUrl });
      //   return <div>Redirecting...</div>;
      // }

      return <div>Signing you out...</div>;
  }

  if (auth.error) {
    return <div>Sign in error... {auth.error.message}</div>;
  }

  if (auth.isAuthenticated) {
    const redirectUrl = localStorage.getItem("redirectUrl");

    if (redirectUrl) {
      localStorage.removeItem("redirectUrl");
      navigate({ to: redirectUrl });
      return <div>Successful login; Redirecting...</div>;
    }

    navigate({ to: "/" });
    return <div>Hello {auth.user?.profile.sub}</div>;
  }

  return <div>Loading...</div>;
}
