
# Development 

Development PRs can be automerged (`gh merge ... --auto`), while release PRs should aggregate releases and merged manually.

- vscode, gitbutler

## Skaffold composable configurations
_Project uses skaffold profiles to override/extend base configurations. Profile Structure (Layered by Concern)_: 

| Profile Name      | Purpose                                       | Includes/Overrides                                                                                        |
| ----------------- | --------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `dev-env`         | Development environment settings (e.g., envs) | Sets environment variables, overrides labels, might tweak port forwarding                                 |
| `staging-env`     | Enables **staging deployment**                | Enables Helm/Kustomize with staging values (based-on production) and local cluster context                |
| `prod-env`        | Enables **production deployment**             | Enables Helm/Kustomize with production values, secrets, including real domains and remote cluster context |
| `watch`           | Enables **file watching** + auto rebuild      | `build.local.push = false`, `build.artifacts[].sync`, `portForward`, etc.                                 |
| `rebuild-debug`   | Forces **rebuild from scratch** for dev       | Clears cache, disables sync, forces `docker` build, targets debug container                               |
| `rebuild-release` | Forces **rebuild from scratch** for prod      | Clears cache, disables sync, forces `docker` build, targets release container                             |
| `prebuilt`        | Use remote **prebuilt images** from registry  | Overrides artifacts with `image:tag`, disables build section                                              |
| `prebuilt`        | Use remote **prebuilt images** from registry  | Overrides artifacts with `image:tag`, disables build section                                              |

### Combinations: 
| Use Case                              | Composite Profile Name | Profiles Used                    | Description                                                             |
| ------------------------------------- | ---------------------- | -------------------------------- | ----------------------------------------------------------------------- |
| Local development with watch          | `dev-watch`            | `dev-env`, `watch`               | Active development with file watching and live reload                   |
| Local development with image rebuild  | `dev-rebuild`          | `dev-env`, `rebuild-debug`       | Clean rebuild of containers without watching                            |
| Staging deployment with image rebuild | `staging-rebuild`      | `staging-env`, `rebuild-release` | Simulates production locally using prebuilt images                      |
| Staging deployment                    | `staging-prebuilt`     | `staging-env`, `prebuilt`        | Simulates production locally using prebuilt images                      |
| Production deployment                 | `prod`                 | `prod-env`, `prebuilt`           | Cloud deployment using prebuilt production containers with real domains |

notes: 
- `skaffold build` will rebuild if image is not present and if changes are detected.
- `--load_restrictor=none` allows to reference files external to overlay directory tree.
- 

## commits (following [Conventional Commit message](https://www.conventionalcommits.org/) and SemVer)
- `feat: ...` → minor release; `fix: ...` → patch release; or with scopes `feat(frontend):`
- `feat!: ...` or `fix!: ...` or any other type → major release (BREAKING CHANGE).
- `docs:`, `test:`, `refactor:`, `chore:`, `build:`, `ci:`, `style:`, `build:`, etc.

## branch convention (optional): 
`feature/*` and `hotfix/*`

## tests: 
- direct `main` branch commits skips testing. 
- PR requests trigger tests (either on direct commits or merges from other branch).

## develop github acitons workflow 
- https://nektosact.com/ locally test workflows

# Database design: 
- spin ChartDB container
- load ChartDB JSON diagram
- modify design
- export to JSON; export to generic SQL; 
- convert generic to postgres-dialect SQL using LLM; 
- modify permissions commands and insert into `init.sql` migration script after fine-tunning for Postgresql

# API endpoints: 
- write SQL ⟶ Resolvers -(manually using a script)⟶ DSL ⟶ Client call wrappers
  - Implement common limit, filter, sort and pagination arguments for resolvers with SQL modification (otherwise, the default arguments supported by the server tool will be handled in Rust instead of optimized DB queries).
- [NOTE] CORS blocked requests in browser can be caused because of an invalid security certificate which should be trusted for each of the subdomains. (manually achieved by visiting and accepting risk button)
