import * as React from "react";
import { useContext, useEffect } from "react";

import { NavUser } from "@/components/nav-user";
import { SectionSwitcher, type Section } from "@/components/section-switcher";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from "@/components/ui/sidebar";
import { SectionContext } from "@/contexts/SectionContext";
import { defaultUser, UserContext } from "@/contexts/UserContext";
import { sections } from "@/data/sections";
import { useAuth } from "react-oidc-context";

import { Button } from "@/components/ui/button";
import { LogIn, User2 } from "lucide-react";
import { type User } from "@/types/user";

export type { Section };

export interface AppSidebarProps extends React.ComponentProps<typeof Sidebar> {}

export function AppSidebar({
  children,
  ...props
}: React.PropsWithChildren<AppSidebarProps>) {
  const { activeSection } = useContext(SectionContext);
  const { user, setUser } = useContext(UserContext);
  const auth = useAuth();

  useEffect(() => {
    if (auth.isAuthenticated && auth.user) {
      setUser({
        name: auth.user.profile.name ?? "",
        email: auth.user.profile.email ?? "",
        avatar: auth.user.profile.picture ?? "",
      });

      if (import.meta.env.DEV) {
        console.log("OIDC user details:", auth.user.profile);
      }
    } else if (!auth.isAuthenticated) {
      setUser(defaultUser); // reset user info
    }
  }, [auth]);

  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <SectionSwitcher sections={sections} />
      </SidebarHeader>
      <SidebarContent>
        {activeSection && <activeSection.sidebarContent />}
      </SidebarContent>
      <SidebarFooter>
        {auth.isAuthenticated ? (
          <NavUser user={user} />
        ) : (
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton
                onClick={() => {
                  localStorage.setItem("redirectUrl", window.location.pathname);
                  auth.signinRedirect();
                }}
                tooltip="Log in"
                variant="outline_colored"
              >
                <LogIn />
                <span className="whitespace-nowrap">Log in</span>
                <User2 className="ml-auto" />
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        )}
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
