## setup repo guide for local development:

- `./script.sh provision_tauri
- install dependencies for Tauri mobile targets: https://v2.tauri.app/start/prerequisites/#configure-for-mobile-targets

- update squash message settings for pull request on github to "pull request title & commit": https://github.blog/changelog/2022-05-11-default-to-pr-titles-for-squash-merge-commit-messages/
- toggle General > "allow automerge" option in repo settings.
- setup automerge using Mergeable Github App; setup "enforce-all-checks" as required setrule for the repo's release branch.
- Use PAT to trigger events from Github API (bots/actions/apps): in github workflows events from bots do not trigger futher events unless using a PAT (rather than relying on Github_Token). https://github.com/orgs/community/discussions/57484
- repo setting > Rulesets > Require status checks to pass > add job "enforce-all-checks"
- repo setting > Rulesets > Require branches to be up to date before merging.
- Organization github setting > Default package settings > Inherit access from source repository. + > package creation > public
- setup Firefox exception for local _.dev domains avoiding SSL certificate warning in the browser. Add new string in about:config > security.exception.siteList = _.dev
- go to about:preferences#privacy and view certificate and add to exception button.
- For github packages turn on "Inherit access from source repository (recommended)" from pacakge settings.
- Manual Cloudflare infrastructure setup required -  dash.cloudflare.com to update kubernetes deployment node loadbalancer ips from Hetzner
- Manual Google OIDC client registration.
- Browser dev tools: React Developer Tools extension
- repository: settings > secrets and variables > actions; add repository secret for ACTIONS_STEP_DEBUG = true and ACTIONS_RUNNER_DEBUG = true
- change docker data location to avoid storage limitations: modify /etc/docker/daemon.json with "data-root" field.
