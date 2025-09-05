# Changelog

## [0.1.9](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.8...api-data-server@v0.1.9) (2025-09-05)


### Documentation

* notes and minor changes ([69081e0](https://github.com/szn-app/donation-app/commit/69081e0cdfd6901ce862138272d3c5e7637935c9))

## [0.1.8](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.7...api-data-server@v0.1.8) (2025-06-17)


### Continuous Integration/Deployment

* apply tag version to staging skaffold using components ([745300e](https://github.com/szn-app/donation-app/commit/745300e4cd533e887c4ea09ebecf62d4fb9b19b3))

## [0.1.7](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.6...api-data-server@v0.1.7) (2025-06-17)


### Bug Fixes

* database docker image build; Trigger build for web-server. ([19b3abb](https://github.com/szn-app/donation-app/commit/19b3abb1e06d163dc6f4d5565bd4528ff5ec6932))

## [0.1.6](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.5...api-data-server@v0.1.6) (2025-06-16)


### Bug Fixes

* **dummy:** trigger release ([85bfd03](https://github.com/szn-app/donation-app/commit/85bfd03bcb1ec3018b99071ce6ba90ef3b5be564))

## [0.1.5](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.4...api-data-server@v0.1.5) (2025-06-16)


### Bug Fixes

* cargo lock file ([e35d38c](https://github.com/szn-app/donation-app/commit/e35d38c51f7d37e8986c16d5ed1250f4dd109e84))

## [0.1.4](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.3...api-data-server@v0.1.4) (2025-06-16)


### Bug Fixes

* graphql operations alignment with server api for create_item ([b7d415b](https://github.com/szn-app/donation-app/commit/b7d415bff292f0499dab3320a88ff51ab70068b9))

## [0.1.3](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.2...api-data-server@v0.1.3) (2025-06-16)


### Bug Fixes

* **api:** update_item operation update of schema and repository ([dbe1ac3](https://github.com/szn-app/donation-app/commit/dbe1ac3a4064e6438e28ede6da5fd4428b20ff59))

## [0.1.2](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.1...api-data-server@v0.1.2) (2025-06-15)


### Bug Fixes

* **dependencies:** update lock files ([6b56706](https://github.com/szn-app/donation-app/commit/6b56706864ebd96f59b298f9fb4489d88f7013cf))
* **dependencies:** update lock files ([60f806a](https://github.com/szn-app/donation-app/commit/60f806a27938574b9f352ac7537f68439071aefb))

## [0.1.1](https://github.com/szn-app/donation-app/compare/api-data-server@v0.1.0...api-data-server@v0.1.1) (2025-06-15)


### Bug Fixes

* update cargo lock files ([b503ab5](https://github.com/szn-app/donation-app/commit/b503ab5aadb481ae73ed582eea6d418f02a55d0e))

## 0.1.0 (2025-06-15)


### Features

* Merge commit '49d668f9fc212636529a9540beef109ce96e2472' ([807407b](https://github.com/szn-app/donation-app/commit/807407bb3197f9c2cd0e66c851b455610b3765ca))

## [0.0.9](https://github.com/szn-app/donation-app/compare/api-data@v0.0.8...api-data@v0.0.9) (2025-06-03)


### Features

* api-data server load environment for username and password for database connection ([0a656dc](https://github.com/szn-app/donation-app/commit/0a656dc0e056de0b0d4d8f3c6ba611414bde571c))

## [0.0.8](https://github.com/szn-app/donation-app/compare/api-data@v0.0.7...api-data@v0.0.8) (2025-05-29)


### Bug Fixes

* **build:** trigger release for fixing builds ([b302b09](https://github.com/szn-app/donation-app/commit/b302b096f68c9de86d0f869cebb3f57f4edf8ed8))

## [0.0.7](https://github.com/szn-app/donation-app/compare/api-data@v0.0.6...api-data@v0.0.7) (2025-05-29)


### Features

* add query sql, models, repository for rest of tables. ([972406c](https://github.com/szn-app/donation-app/commit/972406c88ee4f338971987d977df1bedb6d84737))
* complete CRUD functionality across layers including frontend ([5fe28d9](https://github.com/szn-app/donation-app/commit/5fe28d9729752f83be1295c97b1da081983affd8))
* complete CRUD graphql implementation across layers to datbase and following naming conventions ([95af829](https://github.com/szn-app/donation-app/commit/95af829e9d7966b5d544e603792aa4b2e083cdc8))
* CORS headers implementation on gateway level ([f7067ef](https://github.com/szn-app/donation-app/commit/f7067efe4822ed8b4e2be7b41bbf15df6c480f50))
* graphql api basic entry points for each schema table ([46f7e78](https://github.com/szn-app/donation-app/commit/46f7e78762058520bcd07fbf98529c72f5217797))
* implement CRUD SQL operations for all schemas following consistent LLM instructions ([89367d5](https://github.com/szn-app/donation-app/commit/89367d54e9a3f6427627dda2c732cd4a8e123668))
* implement interaction repository's graphql resolvers ([4db0594](https://github.com/szn-app/donation-app/commit/4db0594fe0d6fc0c9b4817897a23e67a963e85bf))
* implement listing repository's graphql resolvers ([ff8793d](https://github.com/szn-app/donation-app/commit/ff8793dc8b82f6b3a50868c767d0f5165aa0c63a))
* implement user repository's graphql resolvers ([15e4256](https://github.com/szn-app/donation-app/commit/15e4256edcf4d4ddf24e7833301d876dcf848252))
* permission check guard with namespaced subject id ([aa9a91a](https://github.com/szn-app/donation-app/commit/aa9a91aad462833d0d2780d484682433be2eca32))
* preserve consistency across layers : schema, sql ops, rust structs, rust data access ([657f290](https://github.com/szn-app/donation-app/commit/657f290a7861835ec2589c92ca579ee746223666))
* sync frontend and backend with postgresql database schema throught with generate graphql schema ([da91a6f](https://github.com/szn-app/donation-app/commit/da91a6f480c8d9febdb56a538f649d501c57beb7))
* update sql operations as well as Rust models corresponding to db schema changes ([86faaf3](https://github.com/szn-app/donation-app/commit/86faaf3553ccf54a907d7e7476162fc7516200ae))


### Bug Fixes

* autgen script and modify to supported DateTime graphql schema scalar ([4487c71](https://github.com/szn-app/donation-app/commit/4487c71228353f7d6043fa4cb5826b9e56b09c52))
* **authorization:** fix keto authorize for graphql guard ([c799412](https://github.com/szn-app/donation-app/commit/c799412038f0b5f85a34ccad0d815b01b4a97440))
* graphql requests with user id header naming ([8a22372](https://github.com/szn-app/donation-app/commit/8a22372acc77de3329ca8b6b95bf23bc0165ca9e))
* models assign propert postgres field names ([80ac87a](https://github.com/szn-app/donation-app/commit/80ac87aff2780535dd239143e6055e401b5664c6))
* properly pass app-user-id header for all the access-control settings; handle CORS properly for browsers by explicit headers/methods listings ([7749b95](https://github.com/szn-app/donation-app/commit/7749b959253d6f6614765d0872b95651811392e3))
* user id header propagation through gateway & oathkeeper authorizer;  Call keto with read-only grpc functions; ([ffeee99](https://github.com/szn-app/donation-app/commit/ffeee99d3ecc23d2b31623e2ee481e3806294c2f))
* webhook-handler protobuf dependency in distroless image ([be01b8c](https://github.com/szn-app/donation-app/commit/be01b8cb4f496c0f4da62320aec6c1904e47c28b))

## [0.0.6](https://github.com/szn-app/donation-app/compare/api-data@v0.0.5...api-data@v0.0.6) (2025-05-15)


### Features

* add initial sql, access query functions, and models ([c4a4a3e](https://github.com/szn-app/donation-app/commit/c4a4a3e320f156dfac55a973c7d4a47adf55c10d))
* **api-data:** GraphQL implementation with query and mutation resolvers ([cde1e11](https://github.com/szn-app/donation-app/commit/cde1e11ba7db5370627bf5bf52b1192e397d3d04))
* **authorize:** permission check funcationality for Keto service with batch relation tuple checks ([da59993](https://github.com/szn-app/donation-app/commit/da599937051698884c8b03e5715aded44bb91482))
* autogenerate tanstack query type-safe hooks with graphql-request; Support additional custom types; Handle CORS for api ([3f36cd4](https://github.com/szn-app/donation-app/commit/3f36cd481b05dffa9057ad2eb03166079c0330f4))
* GraphQL permission checks for resolvers using async_graphql guards ([85546b3](https://github.com/szn-app/donation-app/commit/85546b32f7a648d1bf140acd633cfb590deea5e3))
* **graphql:** autogenerate graphql SDL from code schema ([d37a433](https://github.com/szn-app/donation-app/commit/d37a433c27cf8c1623ee091ca89a26438f95b653))
* parse request with custom extrator and pass values to graphql resolvers ([eee650f](https://github.com/szn-app/donation-app/commit/eee650f9e80bd808193f5052a06e7bdc5f87d9a1))
* **protobuf:** compile Ory Keto client protobuf services ([5191fff](https://github.com/szn-app/donation-app/commit/5191fff605bbd21ea7aa78a43dd254a602276171))
* refine authentication login/logout flow to involve both operations on Hydra & Kratos for managing user state and cookie session appropriately ([011ed36](https://github.com/szn-app/donation-app/commit/011ed369bfe494c2d33ced1f7dd4c24b51dfdf0d))
* support partial queries with validation/parsing; ([e9d9173](https://github.com/szn-app/donation-app/commit/e9d9173f29c8f773a904795e167850285a614bdb))


### Bug Fixes

* graphql configuration with codegen and react query; + example ([d9ab9b1](https://github.com/szn-app/donation-app/commit/d9ab9b15f25dfdab3a9069cbef3928866c856db9))

## [0.0.5](https://github.com/szn-app/donation-app/compare/api-data@v0.0.4...api-data@v0.0.5) (2025-04-26)


### Features

* broadcast kratos events to kafka ([d214be8](https://github.com/szn-app/donation-app/commit/d214be8c231516fe6d7548a57b59122527c2f5fc))
* customized CNPG image release ([e0bbc64](https://github.com/szn-app/donation-app/commit/e0bbc64ea06aabc2987f324140dfec3cc687ca11))
* implement grpc server for api_data and perform user registration grpc call ([9170733](https://github.com/szn-app/donation-app/commit/9170733c5064a109ab88af729cc18fb0393340c8))
* implement postgres connection pool; Add user to app database in api-data gRPC ([a5d626c](https://github.com/szn-app/donation-app/commit/a5d626c402ca265f56cc311dc20813580aa0ab46))
* implement shared protobuf for api-data with webhook service ([19b0666](https://github.com/szn-app/donation-app/commit/19b066671cb0b3f39359ffb54348ba4ff44ce6c3))
* pass kratos uuid to postgres database; Implement actions retry on failure ([b92e117](https://github.com/szn-app/donation-app/commit/b92e117e9a210f818f1f762df6d68595e4d9dcdd))


### Bug Fixes

* dependency order of kafka-message-queue ([61f3456](https://github.com/szn-app/donation-app/commit/61f34561abb2c27a9ea0d64db0486ca84d68011f))
* expose private methods with pub decoration ([3dd5073](https://github.com/szn-app/donation-app/commit/3dd5073f14fced0c98e5e5f62bf553335812c981))

## [0.0.4](https://github.com/szn-app/donation-app/compare/api-data@v0.0.3...api-data@v0.0.4) (2025-03-20)


### Features

* **auth:** migrate to CNPG HA postgresql setup for auth-ory-stack services; ([3f391aa](https://github.com/szn-app/donation-app/commit/3f391aad82507433bd1fc57729663a3ddc9a93e4))

## [0.0.3](https://github.com/szn-app/donation-app/compare/api-data@v0.0.2...api-data@v0.0.3) (2025-03-16)


### Features

* initial database Postgresql configurations ([7226bff](https://github.com/szn-app/donation-app/commit/7226bffe11af7aaf864e4a37fcf212b9c027487c))
* pass database configuration to api-data server ([fcf1f8c](https://github.com/szn-app/donation-app/commit/fcf1f8cc7012348d80ae93c7901e404eb0773721))

## [0.0.2](https://github.com/szn-app/donation-app/compare/api-data-v0.0.1...api-data@v0.0.2) (2025-03-05)


### Features

* **service:** add api-data service ([7ac82d6](https://github.com/szn-app/donation-app/commit/7ac82d6fd94ead4fd7231ae0142b5f7c6c57f3f4))


### Bug Fixes

* **api-data:** skip test requiring separate server ([fb5494b](https://github.com/szn-app/donation-app/commit/fb5494bb5ad6c663b07d459255bfec39f0d9ec06))
