# Changelog

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
