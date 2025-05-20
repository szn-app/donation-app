
# Development 

Development PRs can be automerged (`gh merge ... --auto`), while release PRs should aggregate releases and merged manually.

- vscode, gitbutler

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