# for feature branches and hotfixes.
feature_pull_request() {  
    if [[ $# -lt 1 ]]; then
        exit 1; 
    fi

    local feature_branch="${1:-feature/example}"
    git push origin $feature_branch
    
    # PR to trigger CI test
    gh pr create --head $feature_branch --base main --title "feat(frontend): new implementation feature" --fill-verbose
    # or merges but without triggering CI test
    {
        git checkout main
        git merge --squash $feature_branch -m "feat(frontend): new implementation feature"
    }

    # NOTE: automerge is applied only on PRs from branches that are prefix with "feature/*" or "hotfix/*".
}

workflow_with_release_please_manually_triggered() { 
    create_feature_pr() {
        feature_branch=$1
        git checkout -b $feature_branch
        git commit --allow-empty -m "commit 1" 
        git commit --allow-empty -m "commit 2" 
        git commit --allow-empty -m "commit 3"
        git push --set-upstream origin $feature_branch
        gh pr create --base main --head $feature_branch --title "feat: adding feacture x to component A" --fill-verbose
    }

    merge_last_pr() { 
        local feature_branch=$1
        git fetch origin 
        git checkout main 
        last_pr_number=$(gh pr list --state open --json number | jq -r '.[0].number') 
        default_branch=$(git remote show origin | grep "HEAD branch:" | awk '{print $3}')
        pr_title=$(gh pr view "$last_pr_number" --json title | jq -r '.title')
        gh pr merge $last_pr_number --squash -t "$pr_title"
        git pull && git push origin main 
    }

    local release_please_workflow=release.yml

    local feature_branch=branch_feature_x
    create_feature_pr $feature_branch

    # create release pr
    merge_last_pr $feature_branch # merge feature pr
    gh workflow run $release_please_workflow # -> release pr is created. 
    gh run list --workflow=$release_please_workflow 

    {
        # new features can be added and will be appended to the existing release PR managed by `release-please``.
        local feature_branch=branch_feature_y
        create_feature_pr $feature_branch
        
        # create release pr
        merge_last_pr $feature_branch # merge feature pr
        gh workflow run $release_please_workflow # -> release pr is created. 
        gh run list --workflow=$release_please_workflow 

        merge_last_pr $feature_branch # merge release pr
    }

    # create a package release on Github
    merge_last_pr $feature_branch # merge release pr
    gh workflow run $release_please_workflow # -> package a github release
}

delete_tag() { 
    tag=${1:-web-server-v0.1.1}
    git push origin :$tag
    git tag -d $tag
}

# NOTE: Dockerfile labels should associate package release to github repo (otherwise a manual web interface association is required)
github_container_registry_deploy() {
    TAG=web-server:latest
    docker tag $TAG ghcr.io/szn-app/donation-app/$TAG
    docker push ghcr.io/szn-app/donation-app/$TAG
}


