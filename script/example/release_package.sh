manual_release_package#example() {
    manual_service_tag_version() { 
        local service="${1:-web-server}" 
        local version="${2:-0.1.0}" 

        # bump package.json version
        set_version() {
            local new_version="$1"

            jq --arg new_version "$new_version" '.version = $new_version' package.json > package.json.tmp
            mv package.json.tmp package.json

            echo "Version set to $new_version"
        }

        pushd ./service/$service
        set_version "$version"

        git add package.json
        git commit -m "$service $version version bump"

        popd
    }

    local service="${1:-web-server}" 
    local version="${2:-0.1.0}" 

    if ! git symbolic-ref --short HEAD | grep -q '^main$'; then
        echo "error: not on main branch."
        exit 1;
    fi

    if [[ -z "$(git diff --cached --name-only)" ]]; then
        echo "No staged files found. Proceeding..."
        if [[ -n "$(git status --porcelain)" ]]; then
            git stash 
        fi 
    else 
        echo "There are staged files. Please commit or stash them before proceeding."
        exit 1
    fi

    if [[ $# -gt 1 ]]; then
        manual_service_tag_version $service $version
    fi

    git push origin 

    git tag $service-v$version
    git push --tags

    git stash pop > /dev/null 2>&1
}