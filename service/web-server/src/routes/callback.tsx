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

  if (import.meta.env.DEV) {
    console.log(auth);
  }

  switch (auth.activeNavigator) {
    case "signinSilent":
      if (import.meta.env.DEV) {
        console.log("Silent signin...");
      }

      return <div>Signing you in...</div>;
    case "signoutRedirect":
      if (import.meta.env.DEV) {
        console.log("Redirecting to signout...");
      }

      // auth.removeUser();

      // const redirectUrl = localStorage.getItem("redirectUrl");
      // if (redirectUrl) {
      //   localStorage.removeItem("redirectUrl");
      //   navigate({ to: redirectUrl });
      //   return <div>Redirecting...</div>;
      // }

      Object.keys(window.localStorage).forEach((key) => {
        if (key.startsWith("oidc.user:")) {
          window.localStorage.removeItem(key);
        }
      });

      return <div>Signing you out...</div>;
  }

  if (auth.error) {
    return <div>Sign in error... {auth.error.message}</div>;
  }

  if (auth.isAuthenticated) {
    const redirectUrl = localStorage.getItem("redirectUrl");

    if (import.meta.env.DEV) {
      console.log("Setting user tokens in cookie for debugging purposes...");
      document.cookie = `dev_access_token=${auth.user?.access_token}; domain=.${window.location.hostname}; path=/; max-age=3600`;
      document.cookie = `dev_id_token=${auth.user?.id_token}; domain=.${window.location.hostname}; path=/; max-age=3600`;
      document.cookie = `dev_refresh_token=${auth.user?.refresh_token}; domain=.${window.location.hostname}; path=/; max-age=3600`;
    }

    if (redirectUrl) {
      localStorage.removeItem("redirectUrl");
      navigate({ to: redirectUrl });
      return <div>Successful login; Redirecting...</div>;
    }

    navigate({ to: "/" });
    return <div>Hello {auth.user?.profile.sub}</div>;
  }

  if (auth.isLoading) {
    return <div>Loading...</div>;
  }

  useEffect(() => {
    setTimeout(() => {
      auth.signinRedirect();
    }, 3000);
  }, [navigate]);

  return <div>Redirecting...</div>;
}
