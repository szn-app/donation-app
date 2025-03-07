/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

import { createFileRoute } from '@tanstack/react-router'

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as CallbackImport } from './routes/callback'

// Create Virtual Routes

const AboutLazyImport = createFileRoute('/about')()
const AppLazyImport = createFileRoute('/_app')()
const AppIndexLazyImport = createFileRoute('/_app/')()
const AppRetailerLazyImport = createFileRoute('/_app/retailer')()
const AppProtectedLazyImport = createFileRoute('/_app/protected')()
const AppMarketLazyImport = createFileRoute('/_app/market')()
const AppLuxuryLazyImport = createFileRoute('/_app/luxury')()
const AppDonationLazyImport = createFileRoute('/_app/donation')()

// Create/Update Routes

const AboutLazyRoute = AboutLazyImport.update({
  id: '/about',
  path: '/about',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/about.lazy').then((d) => d.Route))

const AppLazyRoute = AppLazyImport.update({
  id: '/_app',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/_app.lazy').then((d) => d.Route))

const CallbackRoute = CallbackImport.update({
  id: '/callback',
  path: '/callback',
  getParentRoute: () => rootRoute,
} as any)

const AppIndexLazyRoute = AppIndexLazyImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => AppLazyRoute,
} as any).lazy(() => import('./routes/_app/index.lazy').then((d) => d.Route))

const AppRetailerLazyRoute = AppRetailerLazyImport.update({
  id: '/retailer',
  path: '/retailer',
  getParentRoute: () => AppLazyRoute,
} as any).lazy(() => import('./routes/_app/retailer.lazy').then((d) => d.Route))

const AppProtectedLazyRoute = AppProtectedLazyImport.update({
  id: '/protected',
  path: '/protected',
  getParentRoute: () => AppLazyRoute,
} as any).lazy(() =>
  import('./routes/_app/protected.lazy').then((d) => d.Route),
)

const AppMarketLazyRoute = AppMarketLazyImport.update({
  id: '/market',
  path: '/market',
  getParentRoute: () => AppLazyRoute,
} as any).lazy(() => import('./routes/_app/market.lazy').then((d) => d.Route))

const AppLuxuryLazyRoute = AppLuxuryLazyImport.update({
  id: '/luxury',
  path: '/luxury',
  getParentRoute: () => AppLazyRoute,
} as any).lazy(() => import('./routes/_app/luxury.lazy').then((d) => d.Route))

const AppDonationLazyRoute = AppDonationLazyImport.update({
  id: '/donation',
  path: '/donation',
  getParentRoute: () => AppLazyRoute,
} as any).lazy(() => import('./routes/_app/donation.lazy').then((d) => d.Route))

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/callback': {
      id: '/callback'
      path: '/callback'
      fullPath: '/callback'
      preLoaderRoute: typeof CallbackImport
      parentRoute: typeof rootRoute
    }
    '/_app': {
      id: '/_app'
      path: ''
      fullPath: ''
      preLoaderRoute: typeof AppLazyImport
      parentRoute: typeof rootRoute
    }
    '/about': {
      id: '/about'
      path: '/about'
      fullPath: '/about'
      preLoaderRoute: typeof AboutLazyImport
      parentRoute: typeof rootRoute
    }
    '/_app/donation': {
      id: '/_app/donation'
      path: '/donation'
      fullPath: '/donation'
      preLoaderRoute: typeof AppDonationLazyImport
      parentRoute: typeof AppLazyImport
    }
    '/_app/luxury': {
      id: '/_app/luxury'
      path: '/luxury'
      fullPath: '/luxury'
      preLoaderRoute: typeof AppLuxuryLazyImport
      parentRoute: typeof AppLazyImport
    }
    '/_app/market': {
      id: '/_app/market'
      path: '/market'
      fullPath: '/market'
      preLoaderRoute: typeof AppMarketLazyImport
      parentRoute: typeof AppLazyImport
    }
    '/_app/protected': {
      id: '/_app/protected'
      path: '/protected'
      fullPath: '/protected'
      preLoaderRoute: typeof AppProtectedLazyImport
      parentRoute: typeof AppLazyImport
    }
    '/_app/retailer': {
      id: '/_app/retailer'
      path: '/retailer'
      fullPath: '/retailer'
      preLoaderRoute: typeof AppRetailerLazyImport
      parentRoute: typeof AppLazyImport
    }
    '/_app/': {
      id: '/_app/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof AppIndexLazyImport
      parentRoute: typeof AppLazyImport
    }
  }
}

// Create and export the route tree

interface AppLazyRouteChildren {
  AppDonationLazyRoute: typeof AppDonationLazyRoute
  AppLuxuryLazyRoute: typeof AppLuxuryLazyRoute
  AppMarketLazyRoute: typeof AppMarketLazyRoute
  AppProtectedLazyRoute: typeof AppProtectedLazyRoute
  AppRetailerLazyRoute: typeof AppRetailerLazyRoute
  AppIndexLazyRoute: typeof AppIndexLazyRoute
}

const AppLazyRouteChildren: AppLazyRouteChildren = {
  AppDonationLazyRoute: AppDonationLazyRoute,
  AppLuxuryLazyRoute: AppLuxuryLazyRoute,
  AppMarketLazyRoute: AppMarketLazyRoute,
  AppProtectedLazyRoute: AppProtectedLazyRoute,
  AppRetailerLazyRoute: AppRetailerLazyRoute,
  AppIndexLazyRoute: AppIndexLazyRoute,
}

const AppLazyRouteWithChildren =
  AppLazyRoute._addFileChildren(AppLazyRouteChildren)

export interface FileRoutesByFullPath {
  '/callback': typeof CallbackRoute
  '': typeof AppLazyRouteWithChildren
  '/about': typeof AboutLazyRoute
  '/donation': typeof AppDonationLazyRoute
  '/luxury': typeof AppLuxuryLazyRoute
  '/market': typeof AppMarketLazyRoute
  '/protected': typeof AppProtectedLazyRoute
  '/retailer': typeof AppRetailerLazyRoute
  '/': typeof AppIndexLazyRoute
}

export interface FileRoutesByTo {
  '/callback': typeof CallbackRoute
  '/about': typeof AboutLazyRoute
  '/donation': typeof AppDonationLazyRoute
  '/luxury': typeof AppLuxuryLazyRoute
  '/market': typeof AppMarketLazyRoute
  '/protected': typeof AppProtectedLazyRoute
  '/retailer': typeof AppRetailerLazyRoute
  '/': typeof AppIndexLazyRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/callback': typeof CallbackRoute
  '/_app': typeof AppLazyRouteWithChildren
  '/about': typeof AboutLazyRoute
  '/_app/donation': typeof AppDonationLazyRoute
  '/_app/luxury': typeof AppLuxuryLazyRoute
  '/_app/market': typeof AppMarketLazyRoute
  '/_app/protected': typeof AppProtectedLazyRoute
  '/_app/retailer': typeof AppRetailerLazyRoute
  '/_app/': typeof AppIndexLazyRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths:
    | '/callback'
    | ''
    | '/about'
    | '/donation'
    | '/luxury'
    | '/market'
    | '/protected'
    | '/retailer'
    | '/'
  fileRoutesByTo: FileRoutesByTo
  to:
    | '/callback'
    | '/about'
    | '/donation'
    | '/luxury'
    | '/market'
    | '/protected'
    | '/retailer'
    | '/'
  id:
    | '__root__'
    | '/callback'
    | '/_app'
    | '/about'
    | '/_app/donation'
    | '/_app/luxury'
    | '/_app/market'
    | '/_app/protected'
    | '/_app/retailer'
    | '/_app/'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  CallbackRoute: typeof CallbackRoute
  AppLazyRoute: typeof AppLazyRouteWithChildren
  AboutLazyRoute: typeof AboutLazyRoute
}

const rootRouteChildren: RootRouteChildren = {
  CallbackRoute: CallbackRoute,
  AppLazyRoute: AppLazyRouteWithChildren,
  AboutLazyRoute: AboutLazyRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/callback",
        "/_app",
        "/about"
      ]
    },
    "/callback": {
      "filePath": "callback.tsx"
    },
    "/_app": {
      "filePath": "_app.lazy.tsx",
      "children": [
        "/_app/donation",
        "/_app/luxury",
        "/_app/market",
        "/_app/protected",
        "/_app/retailer",
        "/_app/"
      ]
    },
    "/about": {
      "filePath": "about.lazy.tsx"
    },
    "/_app/donation": {
      "filePath": "_app/donation.lazy.tsx",
      "parent": "/_app"
    },
    "/_app/luxury": {
      "filePath": "_app/luxury.lazy.tsx",
      "parent": "/_app"
    },
    "/_app/market": {
      "filePath": "_app/market.lazy.tsx",
      "parent": "/_app"
    },
    "/_app/protected": {
      "filePath": "_app/protected.lazy.tsx",
      "parent": "/_app"
    },
    "/_app/retailer": {
      "filePath": "_app/retailer.lazy.tsx",
      "parent": "/_app"
    },
    "/_app/": {
      "filePath": "_app/index.lazy.tsx",
      "parent": "/_app"
    }
  }
}
ROUTE_MANIFEST_END */
