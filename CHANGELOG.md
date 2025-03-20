# Changelog

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
