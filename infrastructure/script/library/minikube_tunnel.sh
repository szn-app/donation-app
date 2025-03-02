dns_forwarding_dnsmasq_delete() {
    check_issue() {
        dnsmasq_restart() { 
            sudo dnf update
            sudo dnf remove --allowerasing dnsmasq
            sudo dnf install dnsmasq

            sudo systemctl stop dnsmasq.service
            sudo systemctl disable dnsmasq.service
            sudo systemctl mask dnsmasq.service

            systemctl status dnsmasq.service

            sudo systemctl unmask dnsmasq.service
            sudo systemctl enable dnsmasq.service
            sudo systemctl start dnsmasq.service
        }

        systemctl_restart() {
            sudo systemctl stop systemd-resolved
            sudo systemctl disable systemd-resolved
            sudo systemctl mask systemd-resolved

            systemctl status systemd-resolved

            sudo systemctl unmask systemd-resolved
            sudo systemctl enable systemd-resolved
            sudo systemctl start systemd-resolved
        }

        NetworkManager_restart() { 
            sudo systemctl stop NetworkManager.service
            sudo systemctl disable NetworkManager.service
            sudo systemctl mask NetworkManager.service

            systemctl status NetworkManager.service

            sudo systemctl unmask NetworkManager.service
            sudo systemctl enable NetworkManager.service
            sudo systemctl start NetworkManager.service

            sudo systemctl restart NetworkManager.service
        }

        # sudo kill -9 $(ps aux | grep '[d]nsmasq' | awk '{print $2}')
        systemctl list-units --type=service | grep dnsmasq
        sudo netstat -tulnp | grep :53 # check port conflicts
        journalctl -b0 -u dnsmasq.service
        journalctl -u dnsmasq -n 100 --no-hostname --no-pager
        sudo ss -lp "sport = :domain"
    }

    sudo sed -i '/\.test/d' /etc/dnsmasq.conf
    sudo systemctl restart dnsmasq
}

systemd_resolved_conf() {
    action=${1:-"install"}

    if [ "$action" == "delete" ]; then
        sudo rm -f /etc/systemd/resolved.conf
        sudo systemctl restart systemd-resolved
        return
    fi

    CONFIG_FILE="/etc/systemd/resolved.conf"
    sudo tee "$CONFIG_FILE" > /dev/null <<EOF
[Resolve]
DNS=127.0.0.1
Domains=~test
DNSSEC=no
Cache=false
DNSStubListener=no
EOF

    sudo systemctl restart systemd-resolved
}

networkmanager_config() {
    action=${1:-"install"}

    if [ "$action" == "delete" ]; then
        sudo rm -f /etc/NetworkManager/conf.d/dnsmasq.conf
        sudo rm -f /etc/NetworkManager/dnsmasq.d/test-domains.conf
        sudo systemctl restart NetworkManager
        return
    fi

    CONFIG_FILE="/etc/NetworkManager/conf.d/dnsmasq.conf"
    echo -e "[main]\ndns=dnsmasq" | sudo tee "$CONFIG_FILE" > /dev/null
    echo "address=/test/127.0.0.1" | sudo tee /etc/NetworkManager/dnsmasq.d/test-domains.conf
    sudo systemctl restart NetworkManager
}

dns_forwarding() {
    local loadbalancer_ip="$1"

    dns_forwarding_hosts() {
        # remove previous entries
        sudo sed -i '/\.test/d' /etc/hosts
        # add new entries
        echo "$loadbalancer_ip donation-app.test auth.donation-app.test api.donation-app.test test.donation-app.test *.donation-app.test" | sudo tee -a /etc/hosts
    }

    dnsmasq_conf() {
        sudo sed -i '/\.test/d' /etc/dnsmasq.conf
        # echo "address=/.donation-app.test/$loadbalancer_ip" | sudo tee -a /etc/dnsmasq.conf
        echo "address=/.test/$loadbalancer_ip" | sudo tee -a /etc/dnsmasq.conf
        if ! grep -q "strict-order" /etc/dnsmasq.conf; then
            echo "strict-order" | sudo tee -a /etc/dnsmasq.conf
        fi
        
        sudo systemctl restart dnsmasq
    }

    dns_forwarding_dnsmasq() {
        action=${1:-"install"}

        if [ "$action" == "delete" ]; then
            networkmanager_config delete
            systemd_resolved_conf delete
            return
        fi

        sudo systemctl enable dnsmasq
        sudo systemctl start dnsmasq

        networkmanager_config delete
        dnsmasq_conf
        systemd_resolved_conf install
        networkmanager_config install # makes dnsmasq be controlled by NetworkManager

        {
            sleep 2
            nslookup donation-app.test 127.0.0.1
            time nslookup donation-app.test 127.0.0.1
        }

        verify() { 
            systemctl status dnsmasq
            dig donation-app.test @127.0.0.1
        }
    }

    # dns_forwarding_hosts
    dns_forwarding_dnsmasq install
}

terminate_background_jobs() {
    jobs -p | xargs -r kill -9
    pkill -f "minikube tunnel"

    dns_forwarding_dnsmasq_delete
    networkmanager_config delete
    systemd_resolved_conf delete
}

tunnel_minikube() {
    terminate_background_jobs

    sudo echo "" # switch to sudo explicitely
    minikube tunnel & 
    sleep 5 
    
    while ! kubectl get svc nginx-gateway -n nginx-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}' &> /dev/null; do
        echo "Waiting for load balancer IP..."
        sleep 5
    done
    loadbalancer_ip=$(kubectl get svc nginx-gateway -n nginx-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
    curl -k -i --header "Host: donation-app.test" $loadbalancer_ip
    
    dns_forwarding $loadbalancer_ip

    curl -k -i --resolve donation-app.test:443:$loadbalancer_ip https://donation-app.test
    curl -k -i https://donation-app.test
}
