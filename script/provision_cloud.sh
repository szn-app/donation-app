#!/bin/bash

# [old] hcloud version && docker-credential-pass version && aws --version
# [manual] setup object storage in Hetzner cloud to allow for kOps operations

# https://github.com/kube-hetzner/terraform-hcloud-kube-hetzner
hetzner() { 
    hcloud version && kubectl version && packer --version
    tofu --version && terraform version # either tools should work
    helm version

    # [manual, then move to ~/.ssh] 
    # will also be used to log into the machines using ssh
    ssh-keygen -t ed25519

    {
        tmp_script=$(mktemp)
        curl -sSL -o "${tmp_script}" https://raw.githubusercontent.com/kube-hetzner/terraform-hcloud-kube-hetzner/master/scripts/create.sh
        chmod +x "${tmp_script}" 
        "${tmp_script}"
        rm "${tmp_script}"
    }
    hcloud context create "k8s-project"
    

    ### handle terraform 
    {
        pushd infrastructure

        ### set variables using "terraform.tfvars" or CLI argument or env variables
        # export TF_VAR_hcloud_token=""
        # export TF_VAR_ssh_private_key=""
        # export TF_VAR_ssh_public_key=""

        export TF_TOKEN_app_terraform_io=""  
        terraform init --upgrade # installed terraform module dependecies
        terraform validate

        terraform plan -no-color -out kube.tfplan > output_plan.txt.tmp
        terraform apply kube.tfplan
        t=$(mktemp) && terraform output --raw kubeconfig > "$t" && post_cluster_install "$t"
        # terraform destroy # when completely redploying

        # create kubeconfig (NOTE: do not version control)
        terraform output --raw kubeconfig > ~/.ssh/k8s-project-credentials.kubeconfig.yaml && chmod 600 ~/.ssh/k8s-project-credentials.kubeconfig.yaml

        ### verify: 
        kubectl --kubeconfig ~/.ssh/k8s-project-credentials.kubeconfig.yaml get all -A 
        kubectl --kubeconfig ~/.ssh/k8s-project-credentials.kubeconfig.yaml get configmap -A
        kubectl --kubeconfig ~/.ssh/k8s-project-credentials.kubeconfig.yaml api-resources
        kubectl --kubeconfig ~/.ssh/k8s-project-credentials.kubeconfig.yaml api-versions
        hcloud all list
        terraform show
        terraform state list
        terraform state show type_of_resource.label_of_resource

        helm list -A --all-namespaces --kubeconfig ~/.ssh/k8s-project-credentials.kubeconfig.yaml
        helm get values --all nginx -n nginx --kubeconfig ~/.ssh/k8s-project-credentials.kubeconfig.yaml
        helm get manifest nginx -n nginx --kubeconfig ~/.ssh/k8s-project-credentials.kubeconfig.yaml

        ### ssh into remove machines
        # echo "" > ~/.ssh/known_hosts # clear known hosts to permit connection for same assigned IP to different server
        ip_address=$(hcloud server list --output json | jq -r '.[0].public_net.ipv4.ip')
        ssh -p 2220 root@$ip_address
        ip_address=$(hcloud server list --output json | jq -r '.[0].public_net.ipv6.ip' | sed 's/\/.*/1/')
        ssh -p 2220 root@$ip_address 

        popd
    }
    
}

post_cluster_install() { 
    [ -z "$1" ] && { echo "Error: No arguments provided."; exit 1; } || kubeconfig="$1" 

    # Gateway API CRD installation - https://gateway-api.sigs.k8s.io/guides/#installing-a-gateway-controller
    kubectl apply --kubeconfig $kubeconfig -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.2.0/standard-install.yaml   

    # Gateway controller instlalation - https://gateway-api.sigs.k8s.io/implementations/ & https://docs.nginx.com/nginx-gateway-fabric/installation/ 
    kubectl apply --kubeconfig $kubeconfig -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.5.1/deploy/crds.yaml
    kubectl apply --kubeconfig $kubeconfig -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.5.1/deploy/default/deploy.yaml
    kubectl --kubeconfig $kubeconfig get pods -n nginx-gateway
}