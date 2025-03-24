func#predeploy-skaffold-hook@pgadmin4() {
    local environment=$1

    helm repo add runix https://helm.runix.net
}

func#postdeploy-skaffold-hook@pgadmin4() {
    local environment=$1
}