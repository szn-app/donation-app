import Cookies from "universal-cookie";
import { AuthContextProps } from "react-oidc-context";
import { SignoutResponse, User } from "oidc-client-ts";
import {
  NavigateOptions,
  ToOptions,
  NavigateFn,
  Route,
} from "@tanstack/react-router";
import { useAuth } from "react-oidc-context";

/**
 * Defines the structure for the Kratos logout configuration.
 */
export interface KratosLogoutConfig {
  end_session_endpoint: string; // The URL for the OIDC end session endpoint (Kratos browser logout flow).
}

export const kratosLogoutConfig: KratosLogoutConfig = {
  // Kratos will add headers to request removing the http-only session cookie it manages and proceed to with Hydra redirect to continue with the logout flow
  end_session_endpoint: `${import.meta.env.VITE_AUTHENTICATION_URL}/self-service/logout/browser`, // https://www.ory.sh/docs/reference/api#tag/frontend/operation/createBrowserLogoutFlow
};

export namespace CookieUtil {
  const kratosCookies = [
    // 'ory_kratos_session', // NOTE: managed by Kratos with http-only flag (cannot be removed through Javascript)
    "dev_access_token",
    "dev_id_token",
    "dev_refresh_token",
  ];

  /**
   * Clears specific authentication cookies from the browser.
   * Includes logic to remove based on domain, as dev cookies are likely set this way.
   */
  export function clearAllAuthCookies(): void {
    const cookies = new Cookies();
    // Determine the domain likely used when setting the cookies
    // This should match the logic in setDevAuthCookies
    const domain = `.${window.location.hostname}`;

    kratosCookies.forEach((cookieName) => {
      // Attempt to remove the cookie specifying the domain and path
      cookies.remove(cookieName, { path: "/", domain: domain });
      // Also attempt to remove without specifying the domain, in case it was set differently
      cookies.remove(cookieName, { path: "/" });
    });
  }

  export function set_auth_cookies(
    access_token?: string | null,
    id_token?: string | null,
    refresh_token?: string | null,
  ) {
    if (import.meta.env.DEV) {
      document.cookie = `dev_access_token=${access_token}; domain=.${window.location.hostname}; path=/; max-age=3600`;
      document.cookie = `dev_id_token=${id_token}; domain=.${window.location.hostname}; path=/; max-age=3600`;
      document.cookie = `dev_refresh_token=${refresh_token}; domain=.${window.location.hostname}; path=/; max-age=3600`;
    }
  }
}

/**
 * begin login flow
 */
export async function handleLogin(
  auth: AuthContextProps,
  navigate: NavigateFn,
  options?: {
    saveRedirectPath?: boolean;
  },
) {
  const { saveRedirectPath = true } = options || {};

  // Save current location for redirect after future login if needed
  if (saveRedirectPath) {
    localStorage.setItem("redirectUrl", window.location.pathname);
  }

  auth.signinRedirect();

  const redirectUrl = localStorage.getItem("redirectUrl");
  localStorage.removeItem("redirectUrl");

  if (redirectUrl) {
    navigate({ to: redirectUrl });
  } else {
    navigate({ to: "/" });
  }
}

// Define a cleanup function to clear client-side state
const cleanupClientState = (auth: AuthContextProps) => {
  // Use wildcards to find and remove all OIDC-related items in localStorage
  const oidcPattern = /^oidc\./;

  // Clean localStorage items
  Object.keys(localStorage).forEach((key) => {
    if (oidcPattern.test(key)) {
      console.log(`Removing localStorage item: ${key}`);
      localStorage.removeItem(key);
    }
  });

  // Clean sessionStorage items
  Object.keys(sessionStorage).forEach((key) => {
    if (oidcPattern.test(key)) {
      console.log(`Removing sessionStorage item: ${key}`);
      sessionStorage.removeItem(key);
    }
  });

  // If using a specific auth object, also call its cleanup methods
  if (typeof auth !== "undefined" && auth) {
    // Force the auth context to update its internal state
    // This technically shouldn't be needed as the redirect will reload the page,
    // but it helps ensure clean state if something goes wrong with the redirect
    auth.removeUser(); // Internal method that clears user without redirect
    console.log("Auth user state cleared");
  }

  console.log("OIDC cleanup completed");
};

/**
 * begin logout flow
 */
export async function handleLogout(
  auth: AuthContextProps,
  navigate: NavigateFn,
  options?: {
    saveRedirectPath?: boolean;
  },
) {
  const { saveRedirectPath = true } = options || {};

  // Save current location for redirect after future login if needed
  if (saveRedirectPath) {
    localStorage.setItem("redirectUrl", window.location.pathname);
  }

  if (auth.user && auth.user.id_token) {
    // NOTE: the designed react-oidc-context signoutRedirect() method doesn't support a multiple stage logout involving Kratos and then Hydra (instead it directly logouts through the OIDC Hydra endpoint);

    try {
      // construct Hydra logout url + params (e.g. https://auth.donation-app.local/authorize/oauth2/sessions/logout?id_token_hint=<...>&post_logout_redirect_uri=<..>)
      const endpoint = auth.settings.metadata?.end_session_endpoint;
      if (!endpoint) {
        throw new Error(
          "Hydra end_session_endpoint not found in auth metadata.",
        );
      }
      const hydraLogoutUrl = new URL(endpoint);
      hydraLogoutUrl.searchParams.set("id_token_hint", auth.user.id_token);
      const post_logout_redirect_uri = auth.settings.post_logout_redirect_uri;
      if (!post_logout_redirect_uri) {
        throw new Error(
          "Hydra post_logout_redirect_uri not found in auth metadata.",
        );
      }
      hydraLogoutUrl.searchParams.set(
        "post_logout_redirect_uri",
        post_logout_redirect_uri,
      );

      // construct Kratos url + params with Hydra's logout url as parameter
      const kratosFetchLogoutUrl = new URL(
        kratosLogoutConfig.end_session_endpoint,
      );
      kratosFetchLogoutUrl.searchParams.set(
        "return_to",
        hydraLogoutUrl.toString(),
      );

      try {
        // retrieve constructed logout url (with parameters & cookie session)
        const response = await fetch(kratosFetchLogoutUrl.toString(), {
          method: "GET",
          credentials: "include", // This ensures cookies are sent
          headers: {
            Accept: "application/json",
          },
        });

        if (response.ok) {
          const logoutFlow = await response.json();

          if (logoutFlow.logout_url) {
            // Register beforeunload handler to clean up client state before redirect
            window.addEventListener(
              "beforeunload",
              () => cleanupClientState(auth),
              {
                once: true,
              },
            );

            // Initiate the server-side logout flow (app -> Kratos -> Hydra -> app)
            window.location.href = logoutFlow.logout_url;
            return; // Exit function after redirect is initiated
          }
        }
      } catch (error) {
        console.error("Error initiating Kratos logout redirect:", error);
      }

      // Fallback: If Kratos logout flow fails - if Kratos cookie session not preset or invalid, proceed to signout OIDC client by deleting user data from application
      auth.signoutRedirect({
        id_token_hint: auth.user.id_token,
        post_logout_redirect_uri: auth.settings.post_logout_redirect_uri,
      });

      // Clean up client state in case the redirect doesn't happen immediately
      cleanupClientState(auth);
    } catch (error) {
      console.error("Error during logout process:", error);
    }
  } else {
    const redirectUrl = localStorage.getItem("redirectUrl");
    localStorage.removeItem("redirectUrl");

    if (redirectUrl) {
      navigate({ to: redirectUrl });
    } else {
      navigate({ to: "/" });
    }
  }
}

/**
 * Callback function executed after sign-out
 * Clears cookies and local storage
 * @param resp - The sign-out response (can be undefined)
 */
export const handleSignoutCallback = (_: SignoutResponse | undefined): void => {
  if (import.meta.env.DEV) {
    console.log("onSignoutCallback");
  }

  CookieUtil.clearAllAuthCookies();
};

/**
 * Callback function executed after successful sign-in
 * Removes code and state from URL
 * @param user - The user that was signed in (can be undefined)
 */
export const handleSigninCallback = (user: User | undefined): void => {
  if (import.meta.env.DEV) {
    console.log("onSigninCallback", user);
  }

  /**
   * removes code and state from url after signin
   * see https://github.com/authts/react-oidc-context/blob/f175dcba6ab09871b027d6a2f2224a17712b67c5/src/AuthProvider.tsx#L20-L30
   */
  window.history.replaceState({}, document.title, window.location.pathname);

  CookieUtil.set_auth_cookies(
    user?.access_token,
    user?.id_token,
    user?.refresh_token,
  );
};

export namespace debug {
  export function auth_state_component() {
    const auth = useAuth();
    const redirectUrl = localStorage.getItem("redirectUrl");
    const cookies = new Cookies();

    const allCookies = cookies.getAll();

    const auth_state = JSON.stringify(
      {
        "auth.isLoading": auth.isLoading,
        "auth.isAuthenticated": auth.isAuthenticated,
        "auth.activeNavigator": auth.activeNavigator || false,
        "auth.user": auth.user,
        ory_kratos_session:
          "(NOTE: flagged as http-only cookie - cannot detect if cookie is present or not using Javascript)",
        "cookies:": allCookies,
      },
      null,
      2,
    );

    return (
      <div className="font-mono">
        <h1 className="font-bold">
          App's react-oidc-context state and Kratos auth state:
        </h1>
        <pre>
          {" "}
          {/* Use <pre> to preserve formatting */}
          {auth_state}
        </pre>
      </div>
    );
  }
}

/**
 * Checks localStorage for a redirectUrl and navigates accordingly after an optional timeout.
 * Navigates to the stored URL if found, otherwise navigates to the fallback path.
 *
 * @param navigate The navigate function from @tanstack/react-router's useNavigate hook.
 * @param fallbackPath The path to navigate to if no redirectUrl is found in localStorage (defaults to '/').
 * @param timeoutInSeconds Optional timeout before performing the redirect (in seconds). Defaults to 0 (immediate).
 */
export function handleRedirectAfterCallback(
  navigate: (opts: ToOptions & NavigateOptions) => Promise<void>,
  timeoutInSeconds: number = 0,
  fallbackPath: string = "/",
) {
  // localStorage.getItem returns string | null
  const redirectUrl: string | null = localStorage.getItem("redirectUrl");

  const performRedirect = () => {
    if (redirectUrl) {
      localStorage.removeItem("redirectUrl");
      // The 'to' option in navigate expects a string path.
      // We assert that redirectUrl is a valid route path type.
      // This tells TypeScript to trust us that this string will match one of the defined route paths.
      navigate({ to: redirectUrl as ToOptions["to"] });
    } else {
      navigate({ to: fallbackPath as ToOptions["to"] });
    }
  };

  if (timeoutInSeconds > 0) {
    // If a timeout is specified, wait before redirecting
    setTimeout(performRedirect, timeoutInSeconds * 1000);
  } else {
    // Otherwise, redirect immediately
    performRedirect();
  }
}
