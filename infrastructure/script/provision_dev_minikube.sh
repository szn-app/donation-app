#!/bin/bash

# add hosts DNS resolution in Fedora: resolve *.local to $(minikube ip)
install_domain_dns_systemd_resolved_for_dev_domains() {
    # add minikube dns to linux as a dns server https://minikube.sigs.k8s.io/docs/handbook/addons/ingress-dns/#Linux
    sudo mkdir -p /etc/systemd/resolved.conf.d
    sudo tee /etc/systemd/resolved.conf.d/minikube.conf << EOF
[Resolve]
# DNS=1.1.1.1 $(minikube ip) 8.8.8.8
DNS=$(minikube ip) 8.8.8.8
Domains=dev
# use the System's Existing DNS: makes systemd-resolved use the default DNS as a fallback
# DNSStubListener=yes  # Important! Enables listening on 53 for stub queries
EOF
    sudo systemctl restart systemd-resolved
}

install-resources.minikube@infrastructure() {
    kubectl label node minikube role=worker # used for some affinity/selector configurations in the app manifest/helm files
    kubectl ctx minikube
    kubectl config get-contexts

    {
        minikube addons enable metrics-server
        minikube addons enable dashboard
        launch_dashboard_proxy() {
            # lanch minikube dashboard proxy
            minikube dashboard --url &
        }
    }
    {
        install_envoy_gateway_class
    }
    {
        # Ingress k8s resource controllers
        minikube addons enable ingress # NGINX Ingress controller
        minikube addons enable ingress-dns 

        installation_gateway_controller_nginx

        kubectl get pods -n ingress-nginx # verify Ingress controller running
    }
    {
        minikube_install_cert_manager
        cert_manager_related
    }

    {
        minikube_mock_storage_classes
    }
    {
        install_gateway_api_crds
    }
    {
        # DEPRECATED_install_stackgres_operator
        install_cloudnativepg_operator
        install_minio_operator
        install_kafka_operator
    }
}

delete.minikube#provision#task@infrastructure() {
    # Confirm deletion
    echo -n "Delete Minikube cluster? (y/n): " && read -r answer
    if [[ "${answer,,}" != "y" ]]; then
        echo "Aborted."
        return 1
    fi
    echo "Deleting Minikube..."

    minikube delete
}

install.minikube#provision#task@infrastructure() {
    action=${1:-"install"}

    if ! command -v kubectl-ctx &> /dev/null; then
        echo "kubectl ctx is not installed. Exiting."
        return
    fi

    docker context ls
    docker context use default
    
    minikube config set driver docker
    fix_issue_docker() {
        # fix issue with docker cli plugins where symlinks are corrupted and invalid
        rm ~/.docker/cli-plugins/*
    }
    minikube start --cpus=max --memory=30251MiB --disk-size=100g --driver=docker
    # minikube start --driver=docker

    install-resources.minikube@infrastructure

    minikube status

    # TODO: use host solution instead to avoid issues
    # install_domain_dns_systemd_resolved_for_dev_domains
    # NOTE: careful of minikube dns caching and limitations, if dns name is not resolved after a change, an entire restart of minikube and probably disable/enable addons is required. 

    verify() { 
        minikube kubectl -- get po -A # for a separate version kubectl install
        # or 
        kubetcl ctx minikube
        kubectl cluster-info 
        kubectl get nodes
    }

}

# NOTE: ABANDONED DUE TO ISSUES WITH NONE DRIVER and installation of required dependencies
ABANDONED_bootstrap_minikube_baremetal() {
    action=${1:-"install"}

    if ! command -v kubectl-ctx &> /dev/null; then
        echo "kubectl ctx is not installed. Exiting."
        return
    fi

    if [ "$action" == "delete" ]; then
        minikube delete
        return 
    fi

    # https://hs-note.tistory.com/28
    # https://github.com/kubernetes/minikube/issues/16783
    # https://nieldw.medium.com/running-minikube-with-vm-driver-none-47de91eab84c
    install_minikube_driver_none_dependencies() { 
        sudo yum -y install conntrack crictl

        # https://github.com/Mirantis/cri-dockerd/releases
        sudo dnf install -y https://github.com/Mirantis/cri-dockerd/releases/download/v0.3.16/cri-dockerd-0.3.16-3.fc36.x86_64.rpm

        # https://minikube.sigs.k8s.io/docs/faq/#how-do-i-install-containernetworking-plugins-for-none-driver
        {
            pushd /tmp
            CNI_PLUGIN_VERSION="v1.6.2"
            CNI_PLUGIN_TAR="cni-plugins-linux-amd64-$CNI_PLUGIN_VERSION.tgz" # change arch if not on amd64
            CNI_PLUGIN_INSTALL_DIR="/opt/cni/bin"

            curl -LO "https://github.com/containernetworking/plugins/releases/download/$CNI_PLUGIN_VERSION/$CNI_PLUGIN_TAR"
            sudo mkdir -p "$CNI_PLUGIN_INSTALL_DIR"
            sudo tar -xf "$CNI_PLUGIN_TAR" -C "$CNI_PLUGIN_INSTALL_DIR"
            rm "$CNI_PLUGIN_TAR"
            popd
        }

        sudo yum install -y kubelet kubeadm kubectl

        sudo chown -R $USER $HOME/.minikube
        sudo chgrp -R $USER $HOME/.minikube

        sudo mkdir -p /etc/cni/net.d

        kubeadm init
    }

    # Verify required commands exist
    for cmd in docker conntrack minikube kubectl; do command -v $cmd &>/dev/null || { echo "Error: $cmd not found"; return 1; }; done

    # minikube config set driver none
    minikube start --driver none
    sudo minikube start --driver=none --apiserver-ips 127.0.0.1 --apiserver-name localhost

    install-resources.minikube@infrastructure

    minikube status
}