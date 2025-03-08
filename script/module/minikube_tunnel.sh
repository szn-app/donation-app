# Wait until index.docker.io is resolvable
wait_until_internet_resolvable() {
    local verbose=false

    while getopts "v" opt; do
        case $opt in
            v) verbose=true ;;
            *) ;;
        esac
    done

    log() {
        if [ "$verbose" = true ]; then
            echo "$@"
        fi
    }

    local max_attempts=30
    local attempt=0
    while ! nslookup index.docker.io > /dev/null 2>&1; do
        attempt=$((attempt+1))
        if [ $attempt -ge $max_attempts ]; then
            log "Error: connectivity check - Docker registry not resolvable after $max_attempts attempts"
            return 1
        fi
        log "Attempt $attempt - internet connectivity: Docker registry not resolvable yet, retrying in 5s..."
        sleep 5
    done
    log "Internet is now resolvable"
}

check_donation_app_connectivity() {
    log "Waiting for donation-app.test to be resolvable..."
    local max_attempts=30
    local attempt=0
    while ! nslookup donation-app.test 127.0.0.1 &>/dev/null; do
        attempt=$((attempt+1))
        if [ $attempt -ge $max_attempts ]; then
            log "Error: donation-app.test not resolvable after $max_attempts attempts"
            return 1
        fi
        log "Attempt $attempt - DNS resolution: waiting for donation-app.test to be resolvable, retrying in 2s..."
        sleep 2
    done
    log "Success: donation-app.test is now resolvable"
    time nslookup donation-app.test 127.0.0.1
}


# add_config_section "custom_dns" "server=8.8.8.8" "server=1.1.1.1" "address=/.test/10.96.135.68"
# remove_config_section "custom_dns"
add_config_section() {
    local conf_file="$1"  # First argument is the config file
    local section_name="$2"
    shift 2  # Shift arguments to process the remaining as content

    # Define section markers
    local section_start="# BEGIN $section_name"
    local section_end="# END $section_name"

    # Remove existing section if it exists
    remove_config_section "$conf_file" "$section_name"

    # Append the new section at the end of the file
    sudo bash -c "{
        echo -e '\n$section_start'
        cat
        echo -e '$section_end'
    } >> '$conf_file'"
    
    echo "Added section '$section_name' to $conf_file"
}

remove_config_section() {
    local conf_file="$1"
    local section_name="$2"
    local section_start="# BEGIN $section_name"
    local section_end="# END $section_name"

    # Use sed to remove the section
    sudo sed -i "/$section_start/,/$section_end/d" "$conf_file"

    echo "Removed section '$section_name' from $conf_file"
}

DNSMASQ_CONF="/etc/dnsmasq.conf"
SYSTEMD_RESOLVED_CONFIG="/etc/systemd/resolved.conf"
NETWORK_MANAGER_CONFIG_1="/etc/NetworkManager/conf.d/dnsmasq.conf"
NETWORK_MANAGER_CONFIG_2="/etc/NetworkManager/dnsmasq.d/test-domains.conf"

##################################################

hosts() {
    # remove previous entries
    sudo sed -i '/\.test/d' /etc/hosts
    # add new entries
    echo "$loadbalancer_ip donation-app.test auth.donation-app.test api.donation-app.test test.donation-app.test *.donation-app.test" | sudo tee -a /etc/hosts
}

dns_forwarding() {
    {
        local loadbalancer_ip="$1"
        local verbose=false

        while getopts "v" opt; do
            case $opt in
                v) verbose=true ;;
                *) ;;
            esac
        done

        log() {
            if [ "$verbose" = true ]; then
                echo "$@"
            fi
        }
    }

    {
        add_config_section $DNSMASQ_CONF "custom_dns" <<EOF
# echo "address=/.donation-app.test/$loadbalancer_ip" | sudo tee -a /etc/dnsmasq.conf
# all-servers
# resolv-file=/etc/resolv.conf

# Listen on localhost
listen-address=127.0.0.1

# Do not listen on external interfaces
bind-interfaces

# Do not read /etc/resolv.conf for upstream servers
no-resolv

# Use specific upstream DNS servers (e.g., Google DNS)
server=8.8.8.8
server=8.8.4.4
# server=1.1.1.1

# Enable DNS caching
cache-size=1000

address=/.test/$loadbalancer_ip

EOF
    }
    sudo systemctl enable dnsmasq
    sudo systemctl start dnsmasq
    sudo systemctl restart dnsmasq

    {
        sudo tee "$SYSTEMD_RESOLVED_CONFIG" > /dev/null <<EOF
[Resolve]
DNS=127.0.0.1
Domains=~test
DNSSEC=no
EOF
    }
    echo "modified $SYSTEMD_RESOLVED_CONFIG" 
    sudo systemctl restart systemd-resolved

    # {
    #     # makes dnsmasq be controlled by NetworkManager
    #     echo -e "[main]\ndns=dnsmasq" | sudo tee "$NETWORK_MANAGER_CONFIG_1" > /dev/null
    #     echo "address=/test/127.0.0.1" | sudo tee "$NETWORK_MANAGER_CONFIG_2"
    #     echo "modified $NETWORK_MANAGER_CONFIG_1"
    #     echo "modified $NETWORK_MANAGER_CONFIG_2"
    # }
    # sudo systemctl restart NetworkManager

    check_donation_app_connectivity

    verify() {
        systemctl status dnsmasq
        dig donation-app.test @127.0.0.1
        code /etc/dnsmasq.conf
        code /etc/resolv.conf
    }
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
}

tunnel_minikube() {
    local verbose=false
    local background=false

    while getopts "v" opt; do
        case $opt in
            v) verbose=true ;;
            b) background=true ;;
            *) ;;
        esac
    done

    # Register exit handler for proper cleanup
    tunnel_minikube_delete() {
        jobs -p | xargs -r kill -9
        pkill -f "minikube tunnel"

        {
            remove_config_section $DNSMASQ_CONF "custom_dns"
            sudo systemctl restart systemd-resolved
        }
        {
            sudo rm -f /etc/NetworkManager/conf.d/dnsmasq.conf
            sudo rm -f /etc/NetworkManager/dnsmasq.d/test-domains.conf
            sudo systemctl restart NetworkManager    
        }
        {
            sudo rm -f /etc/systemd/resolved.conf
            sudo systemctl restart systemd-resolved
        }

        wait_until_internet_resolvable
    }
    cleanup_on_exit() {
        echo "Caught exit signal. Cleaning up... Stopping minikube tunnel and cleaning up DNS configuration..."
        tunnel_minikube_delete
        exit 0
    }

    # Trap various exit signals
    trap cleanup_on_exit EXIT SIGTERM SIGINT SIGQUIT

    log() {
        if [ "$verbose" = true ]; then
            echo "$@"
        fi
    }

    sudo echo "" # switch to sudo explicitely      

    minikube tunnel &

    read -t 5 -p "Configure DNS resolver for .test domains? (y/n): [Auto 'y' in 5s] " dns_config || dns_config="y"
    if [[ "${dns_config,,}" =~ ^y ]]; then
        log "Configuring DNS resolver for .test domains"
    else
        log "DNS resolver configuration skipped"
        return
    fi

    while ! kubectl get svc nginx-gateway -n nginx-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}' &> /dev/null; do
        log "Waiting for load balancer IP..."
        sleep 5
    done
    loadbalancer_ip=$(kubectl get svc nginx-gateway -n nginx-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')

    wait_until_internet_resolvable
    
    # curl -k -i --header "Host: donation-app.test" $loadbalancer_ip
    if nslookup test.donation-app.test > /dev/null 2>&1; then
        log "test.donation-app.test resolvable"
    fi
    
    dns_forwarding $loadbalancer_ip
    wait_until_internet_resolvable

    # Try curl commands until domain is resolvable
    local max_attempts=30
    local attempt=0
    while ! curl -k -i --resolve test.donation-app.test:443:$loadbalancer_ip https://test.donation-app.test/allow --connect-timeout 5 -o /dev/null -s; do
        attempt=$((attempt+1))
        if [ $attempt -ge $max_attempts ]; then
            log "Error: test.donation-app.test not accessible after $max_attempts attempts"
            break
        fi
        log "Attempt $attempt - Web access: waiting for test.donation-app.test to be accessible, retrying in 2s..."
        sleep 2
    done
    
    if [ $attempt -lt $max_attempts ]; then
        log "Success: test.donation-app.test is now accessible"
        curl -k -i --resolve test.donation-app.test:443:$loadbalancer_ip https://test.donation-app.test/allow
        curl -k -i https://test.donation-app.test/allow
    fi

    # Ask user if they want to end the minikube tunnel
    echo "Minikube tunnel is running. Press Ctrl+C to stop the tunnel."
    # Set up trap to catch Ctrl+C
    trap 'echo ""; echo "Stopping minikube tunnel and cleaning up DNS configuration..."; tunnel_minikube_delete; exit 0' INT

    echo "ctrl+C to cleanup"
    sleep 10000000
}