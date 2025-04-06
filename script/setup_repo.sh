#!/bin/bash
set -e

config#aggregate_setup#task@monorepo() {
    execute.util '#setup' '#task' '@monorepo'
}

git_submodule#setup@monorepo() {
    clone_with_submodules() { 
        git clone --recursive https://github.com/szn-app/donation-app
    }

    onetime_intialization() {
        git submodule add https://github.com/szn-app/fork-kratos-selfservice-ui-node.git service/auth-ui/kratos-selfservice-ui-node
        git submodule add https://github.com/szn-app/ai-generated dependency/ai-generated
    }

    example_remove() { 
        git submodule deinit -f service/auth-ui/kratos-selfservice-ui-node
        git rm --cached service/auth-ui/kratos-selfservice-ui-node
        rm -r .git/modules/kratos-selfservice-ui-node
        # [manual] remove section from .git/config
    }

    git submodule init && git submodule update
}

docker_github_container_registry#setup@monorepo() {
    CR_PAT='token'
    echo $CR_PAT | docker login ghcr.io -u 'username' --password-stdin # using PAT token    
}

generate_initial_release_please_config#setup@monorepo() {
    pnpm install release-please -g

    # using `release-please-config.json` file to bootstrap release-please 
    release-please bootstrap --token=$GITHUB_TOKEN --repo-url=szn-app/donation-app --dry-run

    # for debug development: 
    release-please release-pr --token=$GITHUB_TOKEN --repo-url=szn-app/donation-app --dry-run
    release-please github-release --token=$GITHUB_TOKEN --repo-url=szn-app/donation-app
}
