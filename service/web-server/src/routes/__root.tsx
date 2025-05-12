import React, { Suspense, useState } from "react";
import { GlobalProvider } from "../contexts/GlobalContext";
import { HeroUIProvider } from "@heroui/react";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

if (import.meta.env.DEV) {
  console.log("--> development mode");
}

export const Route = createRootRoute({
  component: () => (
    <>
      <GlobalProvider>
        <HeroUIProvider>
          <Outlet />
        </HeroUIProvider>
      </GlobalProvider>

      <Suspense>
        {/* https://tanstack.com/query/latest/docs/framework/react/devtools */}
        <ReactQueryDevtools
          initialIsOpen={false}
          buttonPosition="bottom-left"
        />
        {/* https://tanstack.com/router/latest/docs/framework/react/devtools#using-devtools-in-production */}
        <TanStackRouterDevtools initialIsOpen={false} />
      </Suspense>
    </>
  ),
});
