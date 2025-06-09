## IMPORTANT! used in .github/workflows/*
func#predeploy-skaffold-hook@web-server() {
    local environment=${1:-dev}
}

func#postdeploy-skaffold-hook@web-server() {
    local environment=${1:-dev}
}