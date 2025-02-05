import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { navMain, projects, teams, user } from "@/data/sidebar-navigation";

import { Outlet, createLazyFileRoute, useRouter } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { Separator } from "@/components/ui/separator";
import { SidebarTrigger } from "@/components/ui/sidebar";
import {
  BreadcrumbItem,
  BreadcrumbListComponent,
  generateBreadcrumbs,
} from "@/components/breadcrumb-list";

export const Route = createLazyFileRoute("/_app")({
  component,
  notFoundComponent: () => {
    return <h1>Not Found! from _app</h1>;
  },
});

// TODO: create context for active team and share it with breadcrumb

function component() {
  const router = useRouter();
  const [pathLinks, setPathLinks] = useState<BreadcrumbItem[]>([]);

  useEffect(() => {
    setPathLinks(generateBreadcrumbs(router.state.location.pathname));
  }, [router.state.location]);

  return (
    <SidebarProvider>
      <AppSidebar
        user={user}
        teams={teams}
        navItems={navMain}
        projects={projects}
      />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
          <div className="flex items-center gap-2 px-4">
            <SidebarTrigger className="-ml-1" />
            <Separator orientation="vertical" className="mr-2 h-4" />
            <BreadcrumbListComponent items={pathLinks} />
          </div>
        </header>
        <Outlet />
      </SidebarInset>
    </SidebarProvider>
  );
}
