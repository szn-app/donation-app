# Changelog

## [0.0.5](https://github.com/szn-app/donation-app/compare/webhook-handler@v0.0.4...webhook-handler@v0.0.5) (2025-05-29)


### Features

* complete CRUD functionality across layers including frontend ([5fe28d9](https://github.com/szn-app/donation-app/commit/5fe28d9729752f83be1295c97b1da081983affd8))


### Bug Fixes

* models assign propert postgres field names ([80ac87a](https://github.com/szn-app/donation-app/commit/80ac87aff2780535dd239143e6055e401b5664c6))
* webhook-handler protobuf dependency in distroless image ([be01b8c](https://github.com/szn-app/donation-app/commit/be01b8cb4f496c0f4da62320aec6c1904e47c28b))

## [0.0.4](https://github.com/szn-app/donation-app/compare/webhook-handler@v0.0.3...webhook-handler@v0.0.4) (2025-05-15)


### Features

* refine authentication login/logout flow to involve both operations on Hydra & Kratos for managing user state and cookie session appropriately ([011ed36](https://github.com/szn-app/donation-app/commit/011ed369bfe494c2d33ced1f7dd4c24b51dfdf0d))


### Bug Fixes

* protobuf group build of .proto files of same package; ([8ae3f1e](https://github.com/szn-app/donation-app/commit/8ae3f1e8f7f149f9a0d4479ea5cf303bc77de127))

## [0.0.3](https://github.com/szn-app/donation-app/compare/webhook-handler@v0.0.2...webhook-handler@v0.0.3) (2025-04-26)


### Features

* add kafka-ui admin service for debugging ([957a16d](https://github.com/szn-app/donation-app/commit/957a16d90c820ba6e1fea095f068ffe805d8214c))
* broadcast kratos events to kafka ([d214be8](https://github.com/szn-app/donation-app/commit/d214be8c231516fe6d7548a57b59122527c2f5fc))
* customized CNPG image release ([e0bbc64](https://github.com/szn-app/donation-app/commit/e0bbc64ea06aabc2987f324140dfec3cc687ca11))
* expose kafka-ui with access control through the gateway ([47a6eb5](https://github.com/szn-app/donation-app/commit/47a6eb57bdffd8af9a92499a2e2f231d038fa2ec))
* implement grpc server for api_data and perform user registration grpc call ([9170733](https://github.com/szn-app/donation-app/commit/9170733c5064a109ab88af729cc18fb0393340c8))
* implement postgres connection pool; Add user to app database in api-data gRPC ([a5d626c](https://github.com/szn-app/donation-app/commit/a5d626c402ca265f56cc311dc20813580aa0ab46))
* implement shared protobuf for api-data with webhook service ([19b0666](https://github.com/szn-app/donation-app/commit/19b066671cb0b3f39359ffb54348ba4ff44ce6c3))
* **kratos:** configure webhook for registration ([9c14552](https://github.com/szn-app/donation-app/commit/9c14552881c8c55d6a93a372d102ce1d44fe011d))
* pass kratos uuid to postgres database; Implement actions retry on failure ([b92e117](https://github.com/szn-app/donation-app/commit/b92e117e9a210f818f1f762df6d68595e4d9dcdd))


### Bug Fixes

* dependency order of kafka-message-queue ([61f3456](https://github.com/szn-app/donation-app/commit/61f34561abb2c27a9ea0d64db0486ca84d68011f))
* increase retry attempts to grpc connection ([c8acc34](https://github.com/szn-app/donation-app/commit/c8acc34b9d2248dd39fd56a4323f29fbe84bc4d6))

## [0.0.2](https://github.com/szn-app/donation-app/compare/webhook-handler@v0.0.1...webhook-handler@v0.0.2) (2025-04-06)


### Features

* **service:** add webhook-handler service ([5a486b9](https://github.com/szn-app/donation-app/commit/5a486b9840013bf0acdb0aed66b4eb201aa168c9))
