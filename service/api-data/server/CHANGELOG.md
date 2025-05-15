# Changelog

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
