# Changelog

## [0.0.88](https://github.com/szn-app/donation-app/compare/web-server@v0.0.87...web-server@v0.0.88) (2025-06-17)


### Bug Fixes

* database docker image build; Trigger build for web-server. ([19b3abb](https://github.com/szn-app/donation-app/commit/19b3abb1e06d163dc6f4d5565bd4528ff5ec6932))

## [0.0.87](https://github.com/szn-app/donation-app/compare/web-server@v0.0.86...web-server@v0.0.87) (2025-06-16)


### Bug Fixes

* graphql operations alignment with server api for create_item ([b7d415b](https://github.com/szn-app/donation-app/commit/b7d415bff292f0499dab3320a88ff51ab70068b9))

## [0.0.86](https://github.com/szn-app/donation-app/compare/web-server@v0.0.85...web-server@v0.0.86) (2025-06-16)


### Bug Fixes

* **api:** update_item operation update of schema and repository ([dbe1ac3](https://github.com/szn-app/donation-app/commit/dbe1ac3a4064e6438e28ede6da5fd4428b20ff59))

## [0.0.85](https://github.com/szn-app/donation-app/compare/web-server@v0.0.84...web-server@v0.0.85) (2025-06-15)


### Bug Fixes

* missing production .env file in git ([80382ea](https://github.com/szn-app/donation-app/commit/80382ea69fa324ae3a562c5a9cf1666e077ea1a8))

## [0.0.84](https://github.com/szn-app/donation-app/compare/web-server@v0.0.83...web-server@v0.0.84) (2025-06-15)


### Features

* Merge commit '49d668f9fc212636529a9540beef109ce96e2472' ([807407b](https://github.com/szn-app/donation-app/commit/807407bb3197f9c2cd0e66c851b455610b3765ca))

## [0.0.83](https://github.com/szn-app/donation-app/compare/web-server@v0.0.82...web-server@v0.0.83) (2025-06-03)


### Bug Fixes

* production deployment configurations and scripts ([b369710](https://github.com/szn-app/donation-app/commit/b369710f4f78ee76eafc3c813a67bb4856db6cc1))

## [0.0.82](https://github.com/szn-app/donation-app/compare/web-server@v0.0.81...web-server@v0.0.82) (2025-05-29)


### Bug Fixes

* **build:** web server build ([32bc467](https://github.com/szn-app/donation-app/commit/32bc467fe0e41b10e403922577f587656a2ee1aa))

## [0.0.81](https://github.com/szn-app/donation-app/compare/web-server@v0.0.80...web-server@v0.0.81) (2025-05-29)


### Bug Fixes

* **build:** trigger release for fixing builds ([b302b09](https://github.com/szn-app/donation-app/commit/b302b096f68c9de86d0f869cebb3f57f4edf8ed8))

## [0.0.80](https://github.com/szn-app/donation-app/compare/web-server@v0.0.79...web-server@v0.0.80) (2025-05-29)


### Features

* add graphql basic operations (query/mutation) ([7305dcc](https://github.com/szn-app/donation-app/commit/7305dccc0081fdfae4ae69cd7f084c8ee57240c6))
* auth header react hook ([8fd7f6f](https://github.com/szn-app/donation-app/commit/8fd7f6f99a6d27af402cbf2bcc3f870ab8d7d8e3))
* complete CRUD functionality across layers including frontend ([5fe28d9](https://github.com/szn-app/donation-app/commit/5fe28d9729752f83be1295c97b1da081983affd8))
* CORS headers implementation on gateway level ([f7067ef](https://github.com/szn-app/donation-app/commit/f7067efe4822ed8b4e2be7b41bbf15df6c480f50))
* implement CRUD SQL operations for all schemas following consistent LLM instructions ([89367d5](https://github.com/szn-app/donation-app/commit/89367d54e9a3f6427627dda2c732cd4a8e123668))
* mock data for graphql endpoint testing ([75765b0](https://github.com/szn-app/donation-app/commit/75765b0e38b63f4a43fcaca568de3bc5fb5f16f1))
* mock display page for database generated data ([5bc743c](https://github.com/szn-app/donation-app/commit/5bc743caa50ffed8121996f79d5967a68c8c2ffd))
* sync frontend and backend with postgresql database schema throught with generate graphql schema ([da91a6f](https://github.com/szn-app/donation-app/commit/da91a6f480c8d9febdb56a538f649d501c57beb7))


### Bug Fixes

* autgen script and modify to supported DateTime graphql schema scalar ([4487c71](https://github.com/szn-app/donation-app/commit/4487c71228353f7d6043fa4cb5826b9e56b09c52))
* authorization headers for tanstack query calls ([7b03100](https://github.com/szn-app/donation-app/commit/7b03100e1ab7eefc03514e3a9de1beb273487f76))

## [0.0.79](https://github.com/szn-app/donation-app/compare/web-server@v0.0.78...web-server@v0.0.79) (2025-05-15)


### Features

* **authorize:** permission check funcationality for Keto service with batch relation tuple checks ([da59993](https://github.com/szn-app/donation-app/commit/da599937051698884c8b03e5715aded44bb91482))
* autogenerate tanstack query type-safe hooks with graphql-request; Support additional custom types; Handle CORS for api ([3f36cd4](https://github.com/szn-app/donation-app/commit/3f36cd481b05dffa9057ad2eb03166079c0330f4))
* **frontend:** integrate GraphQL Codegen with Tanstack React Query ([75a09fc](https://github.com/szn-app/donation-app/commit/75a09fcf4e603c5d8ca6fcca42f18dead4a109d9))
* parse & validate json data from external API using codegen zod tool ([e4ff6bb](https://github.com/szn-app/donation-app/commit/e4ff6bb92dcbde830a87464d16e27d7bb0df2335))
* refine authentication login/logout flow to involve both operations on Hydra & Kratos for managing user state and cookie session appropriately ([011ed36](https://github.com/szn-app/donation-app/commit/011ed369bfe494c2d33ced1f7dd4c24b51dfdf0d))
* support partial queries with validation/parsing; ([e9d9173](https://github.com/szn-app/donation-app/commit/e9d9173f29c8f773a904795e167850285a614bdb))


### Bug Fixes

* graphql configuration with codegen and react query; + example ([d9ab9b1](https://github.com/szn-app/donation-app/commit/d9ab9b15f25dfdab3a9069cbef3928866c856db9))
* zod validation/parsing of Date fields ([a1a0e41](https://github.com/szn-app/donation-app/commit/a1a0e415a111cdfa7948e238d72e859a188e0a52))

## [0.0.78](https://github.com/szn-app/donation-app/compare/web-server@v0.0.77...web-server@v0.0.78) (2025-04-26)


### Bug Fixes

* **dummy:** release please trigger ([ef6b635](https://github.com/szn-app/donation-app/commit/ef6b635c0a18db0a2d129321acd6eae8e64dd348))

## [0.0.77](https://github.com/szn-app/donation-app/compare/web-server@v0.0.76...web-server@v0.0.77) (2025-03-20)


### Features

* **auth:** migrate to CNPG HA postgresql setup for auth-ory-stack services; ([3f391aa](https://github.com/szn-app/donation-app/commit/3f391aad82507433bd1fc57729663a3ddc9a93e4))

## [0.0.76](https://github.com/szn-app/donation-app/compare/web-server@v0.0.75...web-server@v0.0.76) (2025-03-18)


### Bug Fixes

* **script:** match function names after refactoring ([b87a9e7](https://github.com/szn-app/donation-app/commit/b87a9e7af13f7be5dff75324a6f9df90a8aeb6c6))

## [0.0.75](https://github.com/szn-app/donation-app/compare/web-server@v0.0.74...web-server@v0.0.75) (2025-03-16)


### Bug Fixes

* callback redirect in case of already signed-in user ([85d7c97](https://github.com/szn-app/donation-app/commit/85d7c972181c8eada59154545364d94fbc2815b5))

## [0.0.74](https://github.com/szn-app/donation-app/compare/web-server@v0.0.73...web-server@v0.0.74) (2025-03-05)


### Features

* configure Keto policies and retlations with oathkeeper. ([df0d9f9](https://github.com/szn-app/donation-app/commit/df0d9f912cc2dace4d11771e8b7aeacee1fc5e0b))
* **service:** add api-data service ([7ac82d6](https://github.com/szn-app/donation-app/commit/7ac82d6fd94ead4fd7231ae0142b5f7c6c57f3f4))

## [0.0.73](https://github.com/szn-app/donation-app/compare/web-server@v0.0.72...web-server@v0.0.73) (2025-02-21)


### Features

* implement backend token exchange with react-oidc-context and hydra response ([bbc4884](https://github.com/szn-app/donation-app/commit/bbc488429ad7a6b53b367313b40e7c26e4ed5ccc))
* persist sessions between reloads and update react-oidc-context configs ([fb89d35](https://github.com/szn-app/donation-app/commit/fb89d35bc7911eae48449af92f72d886d86db156))
* redirection to user original url and protected route example ([6d3bdb1](https://github.com/szn-app/donation-app/commit/6d3bdb158ee0e3b794ebf305c198753fc4b20015))
* support oidc and referesh tokens with silent refresh ([c6d127b](https://github.com/szn-app/donation-app/commit/c6d127b94aecc63f214e3a55100c41948e5e6015))

## [0.0.72](https://github.com/szn-app/donation-app/compare/web-server@v0.0.71...web-server@v0.0.72) (2025-02-11)


### Features

* scaffold oauth2 backend token exchange ([b5f30ca](https://github.com/szn-app/donation-app/commit/b5f30cae485488258b1ed3fdd7184d2a11a6680e))

## [0.0.71](https://github.com/szn-app/donation-app/compare/web-server@v0.0.70...web-server@v0.0.71) (2025-02-08)


### Bug Fixes

* **dummy:** update cargo package to trigger release ([c9f0ef4](https://github.com/szn-app/donation-app/commit/c9f0ef4847d599b7064a3db43d5b834edbb18f8c))

## [0.0.70](https://github.com/szn-app/donation-app/compare/web-server@v0.0.69...web-server@v0.0.70) (2025-02-07)


### Features

* context for user info when logged in ([4fd1cba](https://github.com/szn-app/donation-app/commit/4fd1cbac5b701217fc3c16310151d078e5cf261b))
* frontend OIDC client ([6e54bbe](https://github.com/szn-app/donation-app/commit/6e54bbe0770472038a03cc60a0d51b8e5ca67b79))
* index redirect and sidebar toggle ([d452ec1](https://github.com/szn-app/donation-app/commit/d452ec1e38f2b08990479698cc2fca09c6a13359))
* setup backend auth-token-exchange ([840b78e](https://github.com/szn-app/donation-app/commit/840b78e6ada4153db568aa11434de83e14e11c07))
* user sign in button ([0da425b](https://github.com/szn-app/donation-app/commit/0da425ba92172166663987857d139177ff64f131))

## [0.0.69](https://github.com/szn-app/donation-app/compare/web-server@v0.0.68...web-server@v0.0.69) (2025-02-06)


### Bug Fixes

* **dummy:** for testing release-please updater ([3c5ae93](https://github.com/szn-app/donation-app/commit/3c5ae934a99b5b1d23edae1f5e7aead84cdb97a4))

## [0.0.68](https://github.com/szn-app/donation-app/compare/web-server@v0.0.67...web-server@v0.0.68) (2025-02-06)


### Features

* setup heroui ([43a9fb8](https://github.com/szn-app/donation-app/commit/43a9fb86c2d57d1ed59b3952bd97817e4bfb487a))

## [0.0.67](https://github.com/szn-app/donation-app/compare/web-server@v0.0.66...web-server@v0.0.67) (2025-02-06)


### Features

* **dummy:** update packages ([9f8a4fd](https://github.com/szn-app/donation-app/commit/9f8a4fd7c9afd2f487cb2a54eb72b8e654b13c3a))


### Bug Fixes

* remove example tests and files ([236eaca](https://github.com/szn-app/donation-app/commit/236eaca4fe0ace4009e49df7b005e9e94ffa4814))

## [0.0.66](https://github.com/szn-app/donation-app/compare/web-server@v0.0.65...web-server@v0.0.66) (2025-02-06)


### Bug Fixes

* types for imported components and remove old test ([b088987](https://github.com/szn-app/donation-app/commit/b088987d4145c14caec86e1692e5ceaf24d4e4a8))

## [0.0.65](https://github.com/szn-app/donation-app/compare/web-server@v0.0.64...web-server@v0.0.65) (2025-02-06)


### Features

* about & not found pages ([4763a47](https://github.com/szn-app/donation-app/commit/4763a47abfe26c226336cc6f0c8fc89d4206a56f))
* add React tools, dev tools, and examples ([d8dbf61](https://github.com/szn-app/donation-app/commit/d8dbf61549de76586f0b5fb31e6996b1da699d9e))
* app initial layout ([06d9172](https://github.com/szn-app/donation-app/commit/06d9172f5d9033c24392a45c00601dd41dff9a03))
* app sections and primary pages ([b49f53d](https://github.com/szn-app/donation-app/commit/b49f53da8f7c13c15fc153c4041cd6f9f4a2c981))
* associate navigation with app sections; Create sample pages for each section ([711af22](https://github.com/szn-app/donation-app/commit/711af224c3c3df65a17959ccd38c0386a53afe49))
* breadcrumb navigation ([bb4f26a](https://github.com/szn-app/donation-app/commit/bb4f26ad9752d27faeba9113dd6062d73967fe0f))
* setup app scaffold and example pages ([bf0a32e](https://github.com/szn-app/donation-app/commit/bf0a32e52cbf72ee33c5f4c58bb80ae0d19106ff))

## [0.0.64](https://github.com/szn-app/donation-app/compare/web-server@v0.0.63...web-server@v0.0.64) (2025-01-13)


### Bug Fixes

* jsx typescript type matching ([5147787](https://github.com/szn-app/donation-app/commit/51477879d37e194796e8bfec3d676f97b573456d))

## [0.0.63](https://github.com/szn-app/donation-app/compare/web-server@v0.0.62...web-server@v0.0.63) (2025-01-13)


### Features

* add frontend packages and configs ([3fa6cbf](https://github.com/szn-app/donation-app/commit/3fa6cbf723e071dce3788fb4ebd949fcd969edf9))
* auth-ui package registration and release-please workflow to publish container ([ffa5642](https://github.com/szn-app/donation-app/commit/ffa5642950319bab93cb4ae261cda519bc07b8d4))

## [0.0.62](https://github.com/szn-app/donation-app/compare/web-server@v0.0.61...web-server@v0.0.62) (2025-01-09)


### Bug Fixes

* Tauri build fix by removing unused imports ([fedb4c9](https://github.com/szn-app/donation-app/commit/fedb4c936994bbb2e5a12d161b58d7c086035242))

## [0.0.61](https://github.com/szn-app/donation-app/compare/web-server@v0.0.60...web-server@v0.0.61) (2025-01-09)


### Features

* integrate Tailwind css and Prettier ([62e20fd](https://github.com/szn-app/donation-app/commit/62e20fd40587d7e46fe8ef8986a7974ee25efa9e))
* layout example shadcn components ([6345a65](https://github.com/szn-app/donation-app/commit/6345a65a310ab56fb1608867592028039c83eebc))

## [0.0.60](https://github.com/szn-app/donation-app/compare/web-server@v0.0.59...web-server@v0.0.60) (2024-12-18)


### Features

* **dummy:** title change ([417c0c7](https://github.com/szn-app/donation-app/commit/417c0c78d1c908b91acc2c99bc3c5403163508d4))

## [0.0.59](https://github.com/szn-app/donation-app/compare/web-server@v0.0.58...web-server@v0.0.59) (2024-12-18)


### Features

* **dummy:** title change ([414d346](https://github.com/szn-app/donation-app/commit/414d346523b7e8b895be479e3c2cab12aec806b6))

## [0.0.58](https://github.com/szn-app/donation-app/compare/web-server@v0.0.57...web-server@v0.0.58) (2024-12-18)


### Features

* **dummy:** title change ([6cf55b8](https://github.com/szn-app/donation-app/commit/6cf55b8e9700ac8970f63ef15618f6e827095507))

## [0.0.57](https://github.com/szn-app/donation-app/compare/web-server@v0.0.56...web-server@v0.0.57) (2024-12-18)


### Features

* **dummy:** title change ([f737155](https://github.com/szn-app/donation-app/commit/f7371556a76ecab865d38eb819534f37acd902a8))

## [0.0.56](https://github.com/szn-app/donation-app/compare/web-server@v0.0.55...web-server@v0.0.56) (2024-12-18)


### Features

* **dummy:** title change ([1458bc6](https://github.com/szn-app/donation-app/commit/1458bc68d34f3ad769ef521ccfd912dff18d46b7))

## [0.0.55](https://github.com/szn-app/donation-app/compare/web-server@v0.0.54...web-server@v0.0.55) (2024-12-18)


### Features

* **dummy:** title change ([3b1a841](https://github.com/szn-app/donation-app/commit/3b1a841736adf111fcfe93a6c7cd9490d2853df6))

## [0.0.54](https://github.com/szn-app/donation-app/compare/web-server@v0.0.53...web-server@v0.0.54) (2024-12-18)


### Features

* **dummy:** title change ([050d752](https://github.com/szn-app/donation-app/commit/050d75269f4f2e8a3c098c19039624336a087520))

## [0.0.53](https://github.com/szn-app/donation-app/compare/web-server@v0.0.52...web-server@v0.0.53) (2024-12-18)


### Features

* **dummy:** change title ([de4eb3c](https://github.com/szn-app/donation-app/commit/de4eb3c43975c57bd373344f3736d67feeef4cc3))

## [0.0.52](https://github.com/szn-app/donation-app/compare/web-server@v0.0.51...web-server@v0.0.52) (2024-12-18)


### Features

* **dummy:** change title ([192c3d9](https://github.com/szn-app/donation-app/commit/192c3d918d7d7366396cb85b17e2966aecf4df7c))
* **dummy:** empty commit ([0626ab7](https://github.com/szn-app/donation-app/commit/0626ab740912a5c90a098e25ae24df8503e00772))

## [0.0.51](https://github.com/szn-app/donation-app/compare/web-server-v0.0.50...web-server@v0.0.51) (2024-12-18)


### Features

* dummy feature ([c3418d5](https://github.com/szn-app/donation-app/commit/c3418d574e61799c3f536020f72f8e2fa16318af))
* **dummy:** change title ([d0b88fa](https://github.com/szn-app/donation-app/commit/d0b88fa8873f375549daf2779713f6006cb7e17a))
* **dummy:** change title ([31283fd](https://github.com/szn-app/donation-app/commit/31283fd342f938534ed9e3d6d5315ba653076c67))
* **dummy:** change title ([2584d0b](https://github.com/szn-app/donation-app/commit/2584d0b43ffe005e6cc0dab5c4a715232d34014e))
* **dummy:** title for entry page ([9b088bc](https://github.com/szn-app/donation-app/commit/9b088bc018462dcfa7061158604fe597802a9da5))
* **dummy:** title for entry page ([1a4fc1a](https://github.com/szn-app/donation-app/commit/1a4fc1a8c2a64b17e6aa451d3aaacd5912ad565a))
* **dummy:** title for entry page ([badc712](https://github.com/szn-app/donation-app/commit/badc7129ac5ec4d35695c0dbb67fd3f0bb952046))
* new dummy feature ([183ebfd](https://github.com/szn-app/donation-app/commit/183ebfdb8e3be36903c945d63ab3ba08c135d89a))
* title change ([c9a3031](https://github.com/szn-app/donation-app/commit/c9a3031e5e06c2025e4c2ef2e5265c7eb8a3849d))
