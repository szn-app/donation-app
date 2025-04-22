
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
- convert generic to postgres SQL using LLM; 
- modify permissions commands and insert into `init.sql` migration script