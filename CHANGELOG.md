# Changelog

## [0.4.36](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.35...donation-app@v0.4.36) (2025-06-16)


### Bug Fixes

* k8s path ([9f3e201](https://github.com/szn-app/donation-app/commit/9f3e201a17f73c3579f02f305dcf68b2f2f6ddc4))


### Documentation

* todo terraform cloudflare integration ([9005238](https://github.com/szn-app/donation-app/commit/9005238121f32016d8b4466c2546e9edfb1cffd2))

## [0.4.35](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.34...donation-app@v0.4.35) (2025-06-16)


### Bug Fixes

* deployment configs domain overwrites ([9c10d7d](https://github.com/szn-app/donation-app/commit/9c10d7d2c851c049cbd9c175472d19e412ac1d78))
* ory-kratos kustomize deployment for each overlay ([c39ac05](https://github.com/szn-app/donation-app/commit/c39ac05a58bfa4f40893aab32a2a8460bd7f5976))


### Continuous Integration/Deployment

* **oathkeeper:** fix domain substitution in production mode for access control configs ([6410053](https://github.com/szn-app/donation-app/commit/6410053474bf7a762f1b979e64d9976e1850aacf))
* **ory-stack:** move uninstall scripts to corresponding service ([1c7ebde](https://github.com/szn-app/donation-app/commit/1c7ebde0f3fdf51d2726169ebaf3987872528199))

## [0.4.34](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.33...donation-app@v0.4.34) (2025-06-15)


### Bug Fixes

* missing production .env file in git ([80382ea](https://github.com/szn-app/donation-app/commit/80382ea69fa324ae3a562c5a9cf1666e077ea1a8))

## [0.4.33](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.32...donation-app@v0.4.33) (2025-06-15)


### Bug Fixes

* update cargo lock files ([b503ab5](https://github.com/szn-app/donation-app/commit/b503ab5aadb481ae73ed582eea6d418f02a55d0e))

## [0.4.32](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.31...donation-app@v0.4.32) (2025-06-15)


### Features

* Merge commit '49d668f9fc212636529a9540beef109ce96e2472' ([807407b](https://github.com/szn-app/donation-app/commit/807407bb3197f9c2cd0e66c851b455610b3765ca))

## [0.4.31](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.30...donation-app@v0.4.31) (2025-06-03)


### Bug Fixes

* production deployment configurations and scripts ([b369710](https://github.com/szn-app/donation-app/commit/b369710f4f78ee76eafc3c813a67bb4856db6cc1))
* scripts and configs for production deployment ([a0b2b8c](https://github.com/szn-app/donation-app/commit/a0b2b8c4f5043a44196e9453d88d498d672a4ee4))

## [0.4.30](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.29...donation-app@v0.4.30) (2025-05-29)


### Features

* complete CRUD functionality across layers including frontend ([5fe28d9](https://github.com/szn-app/donation-app/commit/5fe28d9729752f83be1295c97b1da081983affd8))
* complete CRUD graphql implementation across layers to datbase and following naming conventions ([95af829](https://github.com/szn-app/donation-app/commit/95af829e9d7966b5d544e603792aa4b2e083cdc8))
* CORS headers implementation on gateway level ([f7067ef](https://github.com/szn-app/donation-app/commit/f7067efe4822ed8b4e2be7b41bbf15df6c480f50))
* preserve consistency across layers : schema, sql ops, rust structs, rust data access ([657f290](https://github.com/szn-app/donation-app/commit/657f290a7861835ec2589c92ca579ee746223666))
* update sql operations as well as Rust models corresponding to db schema changes ([86faaf3](https://github.com/szn-app/donation-app/commit/86faaf3553ccf54a907d7e7476162fc7516200ae))


### Bug Fixes

* autgen script and modify to supported DateTime graphql schema scalar ([4487c71](https://github.com/szn-app/donation-app/commit/4487c71228353f7d6043fa4cb5826b9e56b09c52))
* **authorization:** fix keto authorize for graphql guard ([c799412](https://github.com/szn-app/donation-app/commit/c799412038f0b5f85a34ccad0d815b01b4a97440))
* **dummy:** trigger release ([5563934](https://github.com/szn-app/donation-app/commit/5563934b2de180a97b0671c9eec720c3011e44c4))
* graphql requests with user id header naming ([8a22372](https://github.com/szn-app/donation-app/commit/8a22372acc77de3329ca8b6b95bf23bc0165ca9e))
* models assign propert postgres field names ([80ac87a](https://github.com/szn-app/donation-app/commit/80ac87aff2780535dd239143e6055e401b5664c6))
* properly pass app-user-id header for all the access-control settings; handle CORS properly for browsers by explicit headers/methods listings ([7749b95](https://github.com/szn-app/donation-app/commit/7749b959253d6f6614765d0872b95651811392e3))
* user id header propagation through gateway & oathkeeper authorizer;  Call keto with read-only grpc functions; ([ffeee99](https://github.com/szn-app/donation-app/commit/ffeee99d3ecc23d2b31623e2ee481e3806294c2f))
* webhook-handler protobuf dependency in distroless image ([be01b8c](https://github.com/szn-app/donation-app/commit/be01b8cb4f496c0f4da62320aec6c1904e47c28b))

## [0.4.29](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.28...donation-app@v0.4.29) (2025-05-15)


### Features

* add initial sql, access query functions, and models ([c4a4a3e](https://github.com/szn-app/donation-app/commit/c4a4a3e320f156dfac55a973c7d4a47adf55c10d))
* **api-data:** GraphQL implementation with query and mutation resolvers ([cde1e11](https://github.com/szn-app/donation-app/commit/cde1e11ba7db5370627bf5bf52b1192e397d3d04))
* **authorize:** permission check funcationality for Keto service with batch relation tuple checks ([da59993](https://github.com/szn-app/donation-app/commit/da599937051698884c8b03e5715aded44bb91482))
* autogenerate tanstack query type-safe hooks with graphql-request; Support additional custom types; Handle CORS for api ([3f36cd4](https://github.com/szn-app/donation-app/commit/3f36cd481b05dffa9057ad2eb03166079c0330f4))
* **frontend:** integrate GraphQL Codegen with Tanstack React Query ([75a09fc](https://github.com/szn-app/donation-app/commit/75a09fcf4e603c5d8ca6fcca42f18dead4a109d9))
* **graphql:** autogenerate graphql SDL from code schema ([d37a433](https://github.com/szn-app/donation-app/commit/d37a433c27cf8c1623ee091ca89a26438f95b653))
* parse & validate json data from external API using codegen zod tool ([e4ff6bb](https://github.com/szn-app/donation-app/commit/e4ff6bb92dcbde830a87464d16e27d7bb0df2335))
* parse request with custom extrator and pass values to graphql resolvers ([eee650f](https://github.com/szn-app/donation-app/commit/eee650f9e80bd808193f5052a06e7bdc5f87d9a1))
* **protobuf:** compile Ory Keto client protobuf services ([5191fff](https://github.com/szn-app/donation-app/commit/5191fff605bbd21ea7aa78a43dd254a602276171))
* refine authentication login/logout flow to involve both operations on Hydra & Kratos for managing user state and cookie session appropriately ([011ed36](https://github.com/szn-app/donation-app/commit/011ed369bfe494c2d33ced1f7dd4c24b51dfdf0d))
* support partial queries with validation/parsing; ([e9d9173](https://github.com/szn-app/donation-app/commit/e9d9173f29c8f773a904795e167850285a614bdb))


### Bug Fixes

* graphql configuration with codegen and react query; + example ([d9ab9b1](https://github.com/szn-app/donation-app/commit/d9ab9b15f25dfdab3a9069cbef3928866c856db9))
* protobuf group build of .proto files of same package; ([8ae3f1e](https://github.com/szn-app/donation-app/commit/8ae3f1e8f7f149f9a0d4479ea5cf303bc77de127))

## [0.4.28](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.27...donation-app@v0.4.28) (2025-04-26)


### Bug Fixes

* **dummy:** release please trigger ([ef6b635](https://github.com/szn-app/donation-app/commit/ef6b635c0a18db0a2d129321acd6eae8e64dd348))

## [0.4.27](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.26...donation-app@v0.4.27) (2025-04-26)


### Bug Fixes

* container release name ([32dc455](https://github.com/szn-app/donation-app/commit/32dc455668161d12fb670fe4b4b0730968fcb740))

## [0.4.26](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.25...donation-app@v0.4.26) (2025-04-26)


### Features

* setup api-data-database container for CNPG customized deployment ([49f2dec](https://github.com/szn-app/donation-app/commit/49f2deced6e7c422384541ada94abc32ff039a4d))

## [0.4.25](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.24...donation-app@v0.4.25) (2025-04-26)


### Bug Fixes

* release-please autogenerated workflow update ([348adaf](https://github.com/szn-app/donation-app/commit/348adafba4d3cd2f4b082d2fff846c6bef09a18c))

## [0.4.24](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.23...donation-app@v0.4.24) (2025-04-26)


### Bug Fixes

* api-data-database container build step ([0bf8b2b](https://github.com/szn-app/donation-app/commit/0bf8b2b896ff204300e8601ee0c226d47c98b6fd))

## [0.4.23](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.22...donation-app@v0.4.23) (2025-04-26)


### Bug Fixes

* release-please config naming ([8c961ff](https://github.com/szn-app/donation-app/commit/8c961ff536afa2511d2c45a61ebeff4ff7bacd03))

## [0.4.22](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.21...donation-app@v0.4.22) (2025-04-26)


### Features

* add kafka-ui admin service for debugging ([957a16d](https://github.com/szn-app/donation-app/commit/957a16d90c820ba6e1fea095f068ffe805d8214c))
* broadcast kratos events to kafka ([d214be8](https://github.com/szn-app/donation-app/commit/d214be8c231516fe6d7548a57b59122527c2f5fc))
* comprehensive database model design for user, listing, interaction domains; ([749e4a9](https://github.com/szn-app/donation-app/commit/749e4a99aa13416462241e4d69fe6ff83ccdffcb))
* customized CNPG image release ([e0bbc64](https://github.com/szn-app/donation-app/commit/e0bbc64ea06aabc2987f324140dfec3cc687ca11))
* **db:** design profiles for solo, community groups, & organizations ([24ca523](https://github.com/szn-app/donation-app/commit/24ca5234c6601f730802d5dfbbe0de0257a389f4))
* expose kafka-ui with access control through the gateway ([47a6eb5](https://github.com/szn-app/donation-app/commit/47a6eb57bdffd8af9a92499a2e2f231d038fa2ec))
* implement grpc server for api_data and perform user registration grpc call ([9170733](https://github.com/szn-app/donation-app/commit/9170733c5064a109ab88af729cc18fb0393340c8))
* implement postgres connection pool; Add user to app database in api-data gRPC ([a5d626c](https://github.com/szn-app/donation-app/commit/a5d626c402ca265f56cc311dc20813580aa0ab46))
* implement shared protobuf for api-data with webhook service ([19b0666](https://github.com/szn-app/donation-app/commit/19b066671cb0b3f39359ffb54348ba4ff44ce6c3))
* **kratos:** configure webhook for registration ([9c14552](https://github.com/szn-app/donation-app/commit/9c14552881c8c55d6a93a372d102ce1d44fe011d))
* pass kratos uuid to postgres database; Implement actions retry on failure ([b92e117](https://github.com/szn-app/donation-app/commit/b92e117e9a210f818f1f762df6d68595e4d9dcdd))
* refine SQL migration definitions, match postgresql SQL, and set limits. ([7c367dc](https://github.com/szn-app/donation-app/commit/7c367dc337310a2cd6bf091d7bc1b465377a3bbb))


### Bug Fixes

* add kafka-ui required dependency in skaffold ([32370ad](https://github.com/szn-app/donation-app/commit/32370adb2fad7d76f170d8592ac5a99fb1dd53cd))
* api-data's database secret name ([cb82970](https://github.com/szn-app/donation-app/commit/cb82970c7c43e7f3ab4c4ae4b58e9f0ef377cd0d))
* dependency order of kafka-message-queue ([61f3456](https://github.com/szn-app/donation-app/commit/61f34561abb2c27a9ea0d64db0486ca84d68011f))
* increase retry attempts to grpc connection ([c8acc34](https://github.com/szn-app/donation-app/commit/c8acc34b9d2248dd39fd56a4323f29fbe84bc4d6))
* kafka-ui dependencies ([8ff7dbe](https://github.com/szn-app/donation-app/commit/8ff7dbe00b042be1c3d43539c1f7ce3909b5751a))

## [0.4.21](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.20...donation-app@v0.4.21) (2025-03-20)


### Features

* **auth:** migrate to CNPG HA postgresql setup for auth-ory-stack services; ([3f391aa](https://github.com/szn-app/donation-app/commit/3f391aad82507433bd1fc57729663a3ddc9a93e4))


### Bug Fixes

* namespace correction of object-store; ([fc3d8a1](https://github.com/szn-app/donation-app/commit/fc3d8a121dcbdf9081c4866bc6c3ec35b28b79d8))

## [0.4.20](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.19...donation-app@v0.4.20) (2025-03-18)


### Bug Fixes

* **script:** match function names after refactoring ([b87a9e7](https://github.com/szn-app/donation-app/commit/b87a9e7af13f7be5dff75324a6f9df90a8aeb6c6))

## [0.4.19](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.18...donation-app@v0.4.19) (2025-03-16)


### Bug Fixes

* **ci:** move workflow to required folder ([2d0f9c8](https://github.com/szn-app/donation-app/commit/2d0f9c81b5435c0c8a379a38bda986eff3e42086))
* use # instead of @ to permit special character naming ([99a777b](https://github.com/szn-app/donation-app/commit/99a777b6e8753dc19bc7b0fca5707258ed7536ac))

## [0.4.18](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.17...donation-app@v0.4.18) (2025-03-16)


### Features

* exopse minio console ([f1e9d2e](https://github.com/szn-app/donation-app/commit/f1e9d2e6a369842941e8bfde7fcc6269ba61ce64))
* initial database Postgresql configurations ([7226bff](https://github.com/szn-app/donation-app/commit/7226bffe11af7aaf864e4a37fcf212b9c027487c))
* pass database configuration to api-data server ([fcf1f8c](https://github.com/szn-app/donation-app/commit/fcf1f8cc7012348d80ae93c7901e404eb0773721))
* setup CNPG database deployment ([fb7f0e2](https://github.com/szn-app/donation-app/commit/fb7f0e2b9188f7c60975e3327874c340498e3a9d))
* setup MinIO object store with network volume ([3fa9bbf](https://github.com/szn-app/donation-app/commit/3fa9bbf645c04cf6631bc4d3ac3ebe61abb510fb))


### Bug Fixes

* callback redirect in case of already signed-in user ([85d7c97](https://github.com/szn-app/donation-app/commit/85d7c972181c8eada59154545364d94fbc2815b5))

## [0.4.17](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.16...donation-app@v0.4.17) (2025-03-05)


### Features

* access rules to protect admin dashboards ([048e2b6](https://github.com/szn-app/donation-app/commit/048e2b61251f2bafe4008a8b09508a19649d49e6))
* **auth:** permit cookie session authenticator for protected api registered through internal gateway ([08d6ded](https://github.com/szn-app/donation-app/commit/08d6dedbed8babd4e43aa425acf95bc6c4923bdc))
* configure Keto policies and retlations with oathkeeper. ([df0d9f9](https://github.com/szn-app/donation-app/commit/df0d9f912cc2dace4d11771e8b7aeacee1fc5e0b))
* **service:** add api-data service ([7ac82d6](https://github.com/szn-app/donation-app/commit/7ac82d6fd94ead4fd7231ae0142b5f7c6c57f3f4))

## [0.4.16](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.15...donation-app@v0.4.16) (2025-02-21)


### Features

* implement backend token exchange with react-oidc-context and hydra response ([bbc4884](https://github.com/szn-app/donation-app/commit/bbc488429ad7a6b53b367313b40e7c26e4ed5ccc))
* implement revoke endpoint ([fe17d5f](https://github.com/szn-app/donation-app/commit/fe17d5fb0d041eb71ef08a8361e39e48e43f340a))
* persist sessions between reloads and update react-oidc-context configs ([fb89d35](https://github.com/szn-app/donation-app/commit/fb89d35bc7911eae48449af92f72d886d86db156))
* support oidc and referesh tokens with silent refresh ([c6d127b](https://github.com/szn-app/donation-app/commit/c6d127b94aecc63f214e3a55100c41948e5e6015))

## [0.4.15](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.14...donation-app@v0.4.15) (2025-02-11)


### Features

* scaffold oauth2 backend token exchange ([b5f30ca](https://github.com/szn-app/donation-app/commit/b5f30cae485488258b1ed3fdd7184d2a11a6680e))

## [0.4.14](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.13...donation-app@v0.4.14) (2025-02-11)


### Features

* auth-token-exchange fallback api implementation ([674b4ff](https://github.com/szn-app/donation-app/commit/674b4ffa4e81aa697f3ec343eb83ab3f20e966b3))

## [0.4.13](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.12...donation-app@v0.4.13) (2025-02-08)


### Features

* accept requests from non localhost origins ([85a2d3a](https://github.com/szn-app/donation-app/commit/85a2d3af5071b0d880ab2fd5c4fef3c453103cdd))

## [0.4.12](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.11...donation-app@v0.4.12) (2025-02-08)


### Bug Fixes

* **docker:** copy configuration step ([24a8af6](https://github.com/szn-app/donation-app/commit/24a8af6c7d5fcc96f5e41109d53d59ad1ecf5245))

## [0.4.11](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.10...donation-app@v0.4.11) (2025-02-07)


### Bug Fixes

* docker config in Rust subproject ([286dbbe](https://github.com/szn-app/donation-app/commit/286dbbe4d3ac6663fcaff25720e8c8a67173029a))

## [0.4.10](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.9...donation-app@v0.4.10) (2025-02-07)


### Features

* frontend OIDC client ([6e54bbe](https://github.com/szn-app/donation-app/commit/6e54bbe0770472038a03cc60a0d51b8e5ca67b79))
* setup backend auth-token-exchange ([840b78e](https://github.com/szn-app/donation-app/commit/840b78e6ada4153db568aa11434de83e14e11c07))

## [0.4.9](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.8...donation-app@v0.4.9) (2025-02-06)


### Bug Fixes

* **dummy:** for testing release-please updater ([3c5ae93](https://github.com/szn-app/donation-app/commit/3c5ae934a99b5b1d23edae1f5e7aead84cdb97a4))

## [0.4.8](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.7...donation-app@v0.4.8) (2025-02-06)


### Bug Fixes

* remove example tests and files ([236eaca](https://github.com/szn-app/donation-app/commit/236eaca4fe0ace4009e49df7b005e9e94ffa4814))

## [0.4.7](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.6...donation-app@v0.4.7) (2025-02-06)


### Bug Fixes

* types for imported components and remove old test ([b088987](https://github.com/szn-app/donation-app/commit/b088987d4145c14caec86e1692e5ceaf24d4e4a8))

## [0.4.6](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.5...donation-app@v0.4.6) (2025-02-06)


### Features

* add React tools, dev tools, and examples ([d8dbf61](https://github.com/szn-app/donation-app/commit/d8dbf61549de76586f0b5fb31e6996b1da699d9e))
* **auth:** Google OIDC provider ([e6f48e5](https://github.com/szn-app/donation-app/commit/e6f48e599990cac3da1e5471a45c7757a073bcc6))
* complete Oauth2.0 + OIDC integration ([9aba583](https://github.com/szn-app/donation-app/commit/9aba5835b1e191e0acf4b23ddd0d53d60280b966))


### Bug Fixes

* **gateway:** properly configure gateway routes and expose Hubble UI ([6f659d1](https://github.com/szn-app/donation-app/commit/6f659d169035f0593034b57db9f4091926534243))
* rollback cert-manager version to avoid issue ([ac66fb8](https://github.com/szn-app/donation-app/commit/ac66fb8e68bb4fc3da7c6a801c8655b21cbdd8e0))

## [0.4.5](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.4...donation-app@v0.4.5) (2025-01-14)


### Bug Fixes

* secret parsing using secretGenerator kustomize property ([4a3b93e](https://github.com/szn-app/donation-app/commit/4a3b93ee508d391059fdecbc6496314547c8e06e))

## [0.4.4](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.3...donation-app@v0.4.4) (2025-01-13)


### Features

* add frontend packages and configs ([3fa6cbf](https://github.com/szn-app/donation-app/commit/3fa6cbf723e071dce3788fb4ebd949fcd969edf9))
* add submodule kratos ui ([d250342](https://github.com/szn-app/donation-app/commit/d2503425b85ad0cf14bce81e5e17ec6aa6cc33c6))
* auth-ui package registration and release-please workflow to publish container ([ffa5642](https://github.com/szn-app/donation-app/commit/ffa5642950319bab93cb4ae261cda519bc07b8d4))
* **gateway:** expose internal management interfaces ([3f88cc5](https://github.com/szn-app/donation-app/commit/3f88cc510cd4e695fce2951d802b6fbb0ce1f568))
* user ui for auth services ([132c8c4](https://github.com/szn-app/donation-app/commit/132c8c4c9c28c2009fd63198dcf5c236f68857f6))

## [0.4.3](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.2...donation-app@v0.4.3) (2025-01-09)


### Features

* integrate Tailwind css and Prettier ([62e20fd](https://github.com/szn-app/donation-app/commit/62e20fd40587d7e46fe8ef8986a7974ee25efa9e))
* layout example shadcn components ([6345a65](https://github.com/szn-app/donation-app/commit/6345a65a310ab56fb1608867592028039c83eebc))

## [0.4.2](https://github.com/szn-app/donation-app/compare/donation-app@v0.4.1...donation-app@v0.4.2) (2025-01-04)


### Features

* **routing:** setup routing for api subdomain and root with https redirect ([e10765e](https://github.com/szn-app/donation-app/commit/e10765e7a92dfc74fc72c8c81698b84b13d50822))
* **tls:** generate certificates using cert-manager ([e10765e](https://github.com/szn-app/donation-app/commit/e10765e7a92dfc74fc72c8c81698b84b13d50822))


### Bug Fixes

* web server service as ClusterIP instead of LoadBalancer ([53bab67](https://github.com/szn-app/donation-app/commit/53bab67953fd69474a5c93362303b89782a6bdc8))

## [0.4.1](https://github.com/szn-app/donation-app/compare/donation-app-v0.4.0...donation-app@v0.4.1) (2024-12-18)


### Features

* dummy feature rust ([cc9564f](https://github.com/szn-app/donation-app/commit/cc9564fafc8537c6dba7c65f2d5d50ecdee743a5))
* dummy feature rust ([28b0285](https://github.com/szn-app/donation-app/commit/28b02853b021ef2f993eee3b7cb64932bd818bfc))
