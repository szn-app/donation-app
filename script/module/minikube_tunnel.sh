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

# Function to check if donation-app.local is resolvable
check_gateway_external_ip() {   
    # Return status based on whether we found an IP
    local external_ip
    external_ip=$(kubectl get svc nginx-gateway -n nginx-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null)
    
    if [[ -z "$external_ip" ]]; then
        echo "1"
        return 1
    else
        echo "0"
        return 0
    fi
}

check_donation_app_connectivity() {
    log "Waiting for donation-app.local to be resolvable..."
    local max_attempts=30
    local attempt=0
    while ! nslookup donation-app.local 127.0.0.1 &>/dev/null; do
        attempt=$((attempt+1))
        if [ $attempt -ge $max_attempts ]; then
            log "Error: donation-app.local not resolvable after $max_attempts attempts"
            return 1
        fi
        log "Attempt $attempt - DNS resolution: waiting for donation-app.local to be resolvable, retrying in 2s..."
        sleep 2
    done
    log "Success: donation-app.local is now resolvable"
    time nslookup donation-app.local 127.0.0.1
}


# Function to gracefully terminate all background jobs
cleanup_jobs() {
    echo "Cleaning up background jobs..."
    local job_list=$(jobs -p)
    if [[ -n "$job_list" ]]; then
        echo "Terminating jobs: $job_list"
        jobs -p | xargs -r kill -15  # SIGTERM first for graceful shutdown
        sleep 1
        # Force kill any remaining jobs
        job_list=$(jobs -p)
        if [[ -n "$job_list" ]]; then
            echo "Force killing remaining jobs: $job_list"
            jobs -p | xargs -r kill -9  # SIGKILL for stubborn processes
        fi
    else
        echo "No background jobs to clean up"
    fi
}

# add_config_section "custom_dns" "server=8.8.8.8" "server=1.1.1.1" "address=/.local/10.96.135.68"
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
    # Remove empty lines at the end of the file
    sudo sed -i -e :a -e '/^\n*$/{$d;N;ba' -e '}' $conf_file

    echo "Removed section '$section_name' from $conf_file"
}

DNSMASQ_CONF="/etc/dnsmasq.conf"
SYSTEMD_RESOLVED_CONFIG="/etc/systemd/resolved.conf"
NETWORK_MANAGER_CONFIG_1="/etc/NetworkManager/conf.d/dnsmasq.conf"
NETWORK_MANAGER_CONFIG_2="/etc/NetworkManager/dnsmasq.d/test-domains.conf"

##################################################

hosts_example() {
    # remove previous entries
    sudo sed -i '/\.local/d' /etc/hosts
    # add new entries
    echo "$loadbalancer_ip donation-app.local auth.donation-app.local api.donation-app.local test.donation-app.local *.donation-app.local" | sudo tee -a /etc/hosts
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
# echo "address=/.donation-app.local/$loadbalancer_ip" | sudo tee -a /etc/dnsmasq.conf
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

address=/.local/$loadbalancer_ip

EOF
    }
    sudo systemctl enable dnsmasq
    sudo systemctl start dnsmasq
    sudo systemctl restart dnsmasq

    {
        # ⚠️ Conflict with mDNS (Bonjour/Avahi): If Avahi (Linux/macOS) or Bonjour (Windows/macOS) is running, .local names will not resolve through dnsmasq. Avoid using .local unless mDNS is disabled.
        sudo tee "$SYSTEMD_RESOLVED_CONFIG" > /dev/null <<EOF
[Resolve]
DNS=127.0.0.1
Domains=~local
MulticastDNS=no
DNSSEC=no
Cache=no
EOF
# FallbackDNS=1.1.1.1 8.8.8.8
    }
    echo "modified $SYSTEMD_RESOLVED_CONFIG" 
    sudo systemctl restart systemd-resolved

    # {
    #     # makes dnsmasq be controlled by NetworkManager
    #     echo -e "[main]\ndns=dnsmasq" | sudo tee "$NETWORK_MANAGER_CONFIG_1" > /dev/null
    #     echo "address=/local/127.0.0.1" | sudo tee "$NETWORK_MANAGER_CONFIG_2"
    #     echo "modified $NETWORK_MANAGER_CONFIG_1"
    #     echo "modified $NETWORK_MANAGER_CONFIG_2"
    # }
    # sudo systemctl restart NetworkManager

    check_donation_app_connectivity

    verify() {
        systemctl status dnsmasq
        dig donation-app.local @127.0.0.1
        code /etc/dnsmasq.conf
        code /etc/resolv.conf
    }
    check_issue() {
        sudo chmod 644 /etc/systemd/resolved.conf.d/*
        sudo chown root:root /etc/systemd/resolved.conf.d/*

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

remove_dns_forwarding() {
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

delete.tunnel.minikube#task@monorepo() {
    # Gracefully terminate minikube tunnel processes
    echo "Gracefully stopping minikube tunnel processes..."
    if pgrep -f "minikube tunnel" > /dev/null 2>&1; then
        kill -2 "$(pgrep -f "minikube tunnel")" >/dev/null 2>&1
    fi

    remove_dns_forwarding
}

start.tunnel.minikube#task@monorepo() {
    local verbose=false
    local background=false

    while getopts "v" opt; do
        case $opt in
            v) verbose=true ;;
            b) background=true ;;
            *) ;;
        esac
    done

    # Check domain accessibility before proceeding
    if [ "$(check_gateway_external_ip)" = "0" ]; then
        echo "minikube tunnel already running"
        echo "refreshing minikube tunnel..."
        delete.tunnel.minikube#task@monorepo
        # read -t 5 -p "Do you want to refresh minikube tunnel? [y/n] " answer
        # answer=${answer:-y}
        # if [[ "$answer" =~ ^[Yy]$ ]]; then
        #     delete.tunnel.minikube#task@monorepo
        # else
        #     return
        # fi
    fi

    # Register exit handler for proper cleanup
    cleanup_on_exit() {
        echo "Caught exit signal. Cleaning up... Stopping minikube tunnel and cleaning up DNS configuration..."
        delete.tunnel.minikube#task@monorepo
        exit 0
    }

    # Trap various exit signals
    # trap cleanup_on_exit EXIT SIGTERM SIGINT SIGQUIT

    log() {
        if [ "$verbose" = true ]; then
            echo "$@"
        fi
    }

    sudo echo "" # switch to sudo explicitely      
    
    minikube tunnel --cleanup=true &
    sleep 2

    loadbalancer_ip=$(kubectl get svc nginx-gateway -n nginx-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
    while [ -z "$loadbalancer_ip" ]; do
        log "Waiting for load balancer IP..."
        sleep 5
        loadbalancer_ip=$(kubectl get svc nginx-gateway -n nginx-gateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
    done

    echo "$loadbalancer_ip"
    dns_forwarding $loadbalancer_ip
    wait_until_internet_resolvable
    
    # curl -k -i --header "Host: donation-app.local" $loadbalancer_ip
    if nslookup test.donation-app.local > /dev/null 2>&1; then
        log "test.donation-app.local resolvable"
    fi

    # Try curl commands until domain is resolvable
    local max_attempts=30
    local attempt=0
    log "Attempt $attempt - Web access: waiting for test.donation-app.local to be accessible, retrying in 2s..."
    while ! curl -k -i --resolve test.donation-app.local:443:$loadbalancer_ip https://test.donation-app.local/allow --connect-timeout 5 -o /dev/null -s; do
        attempt=$((attempt+1))
        if [ $attempt -ge $max_attempts ]; then
            log "Error: test.donation-app.local not accessible after $max_attempts attempts"
            break
        fi
        printf "\rAttempt %d/%d - Web access: waiting for test.donation-app.local to be accessible..." $attempt $max_attempts
        sleep 2
    done
    
    if [ $attempt -lt $max_attempts ]; then
        log "Success: test.donation-app.local is now accessible"
        curl -k -i --resolve test.donation-app.local:443:$loadbalancer_ip https://test.donation-app.local/allow
        curl -k -i https://test.donation-app.local/allow
    fi

    # Ask user if they want to end the minikube tunnel
    echo "Minikube tunnel is running."
    # Set up trap to catch Ctrl+C
    trap 'echo ""; echo "Stopping minikube tunnel and cleaning up DNS configuration..."; delete.tunnel.minikube#task@monorepo; exit 0' INT

    echo "Minikube tunnel is running. Press Ctrl+C to stop."
    # Wait indefinitely
    while true; do
        sleep 86400 &  # Sleep for a day
        wait $!  # Wait for the sleep to finish
    done
}