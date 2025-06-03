#!/bin/bash

disbale-volume_protection#provision@infrastructure() {
    if ! hcloud server list &> /dev/null; then
        echo "❌ hcloud CLI is not authenticated. export HCLOUD_TOKEN=..."
        echo "ERROR: Failed to retrieve server list from Hetzner Cloud."
        echo "       Please check your HCLOUD_TOKEN or hcloud CLI configuration."
        return 1
    fi 
 
  # Get a list of all volume IDs
  VOLUME_IDS=$(hcloud volume list -o json | jq -r '.[].id')

  # Loop through each volume ID and disable delete protection
  for VOL_ID in $VOLUME_IDS; do
    echo "Disabling delete protection for volume ID: $VOL_ID"
    hcloud volume disable-protection "$VOL_ID" delete
    if [ $? -eq 0 ]; then
      echo "Successfully disabled protection for $VOL_ID"
    else
      echo "Failed to disable protection for $VOL_ID"
    fi
  done
}

delete.hetzner.cloud#provision#task@infrastructure() {
    echo "NOTE: run 'disbale-volume_protection#provision@infrastructure' first"
    
    pushd infrastructure
      ### [manual] set variables using "terraform.tfvars" or CLI argument or equivalent env variables (with `TF_TOKEN_*` prefix)
      find . -name "*.tfvars"
      set -a && source ".env" && set +a # export TF_TOKEN_app_terraform_io="" 

      printf "Destroying infrastructure...\n"
      terraform init
      terraform destroy -auto-approve
    popd
}

# create snapshots with kube-hetzner binaries (idempotent can be executed in existing project)
# executed one time 
create_snapshot.hetzner.cloud#provision#task@infrastructure() {(
  pushd infrastructure

  tmp_script=$(mktemp)
  curl -sSL -o "${tmp_script}" https://raw.githubusercontent.com/kube-hetzner/terraform-hcloud-kube-hetzner/master/scripts/create.sh
  chmod +x "${tmp_script}" 
  "${tmp_script}"
  rm "${tmp_script}"

  echo "TODO: manulaly next update terraform.tfvars with snapshot ids"

  popd
)}

check_tools_versions() {
  hcloud version && kubectl version && packer --version
  tofu --version && terraform version # either tools should work
  helm version && cilium version
  k9s version && kubectl krew version
  kubectl krew list | grep ctx && kubectl krew list | grep ns 
}

# https://github.com/kube-hetzner/terraform-hcloud-kube-hetzner
hetzner.cloud#provision#task@infrastructure() {
    if ! command -v kubectl-ctx &> /dev/null; then
        echo "kubectl ctx is not installed. Exiting."
        return
    fi

  check_tools_versions

  # remove warnings and logs from coredns
  remove_warnings_logs() { 
    kubectl apply -f - <<'EOF'
apiVersion: v1
kind: ConfigMap
metadata:
  name: coredns-custom
  namespace: kube-system
data:
  log.override: |
    #
  stub.server: |
    #
EOF
    kubectl rollout restart deploy/coredns -n kube-system
  }

    manually_prerequisites() {
      # TODO: automate

      # [manual, then move to ~/.ssh] 
      # Generate an ed25519 SSH key pair - will also be used to log into the machines using ssh
      ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 && chmod 600 ~/.ssh/id_ed25519
      
      create_snapshot.hetzner.cloud#provision#task@infrastructure
    }
    
    ### handle terraform 
    {
      pushd infrastructure

      generate_kubeconfig() {
        find . -name "*.tfvars"
        set -a && source ".env" && set +a # export TF_TOKEN_app_terraform_io="" 

        # create kubeconfig (NOTE: do not version control this credentials file)
        terraform output --raw kubeconfig > "$(realpath ~/.kube)/kubernetes-project-credentials.kubeconfig.yaml"

        # Set default kubeconfig file path
        DEFAULT_KUBECONFIG="$HOME/.kube/config"

        # Create a backup of the existing kubeconfig file
        if [ -f "$DEFAULT_KUBECONFIG" ]; then
            cp "$DEFAULT_KUBECONFIG" "$DEFAULT_KUBECONFIG.bak"
            echo "Backup of existing kubeconfig created at $DEFAULT_KUBECONFIG.bak"
        fi

        # Find all kubeconfig files matching *.kubeconfig.yaml in ~/.kube/
        KUBECONFIG_FILES=$(find "$HOME/.kube" -type f -name "*.kubeconfig.yaml")

        # Check if there are any files to merge
        if [ -z "$KUBECONFIG_FILES" ]; then
            echo "No *.kubeconfig.yaml files found in $HOME/.kube/"
            exit 1
        fi

        # Merge all kubeconfig files into the default kubeconfig
        # KUBECONFIG=$(echo "$KUBECONFIG_FILES" | tr '\n' ':')
        export KUBECONFIG=$(echo "$KUBECONFIG_FILES" | tr '\n' ':')
        kubectl config view --merge --flatten > "$DEFAULT_KUBECONFIG.tmp"

        # Replace the default kubeconfig with the merged file
        mv "$DEFAULT_KUBECONFIG.tmp" "$DEFAULT_KUBECONFIG"
        find $(realpath ~/.kube) -name "*.kubeconfig.yaml" -exec chmod 600 {} + && chmod 600 $(realpath ~/.kube)/config
        echo "Merged kubeconfig files into $DEFAULT_KUBECONFIG"

        # Verify the merge
        kubectl config get-contexts
      }

      # TODO: refactor and document hackish approach 
      patch_init_tf_file() {
        # Install CRDs required by Cilium Gateway API support (required CRDs before Cilium installation) https://docs.cilium.io/en/stable/network/servicemesh/gateway-api/gateway-api/ 
        #     IMPORTANT: Cilium Gateway API controller must be installed BEFORE Cilium installation, otherwise even a restart won't work
        #!/bin/bash
        INIT_TF_PATH=".terraform/modules/kube-hetzner/init.tf"
        # NOTE: contents of match file should be synchronized with the script that is used for minikube to keep development and production environment in sync 'install_gateway_api_crds'
        PATCH_FILE="init.tf.patch"

        # Check if files exist
        if [[ ! -f "$INIT_TF_PATH" ]]; then
            echo "Error: $INIT_TF_PATH does not exist."
            return
        fi

        if [[ ! -f "$PATCH_FILE" ]]; then
            echo "Error: $PATCH_FILE does not exist."
            return
        fi

        # Verify the target line exists in the file
        if ! grep -q 'kubectl apply -k /var/post_install' "$INIT_TF_PATH"; then
            echo "Error: Target line 'kubectl apply -k /var/post_install' not found in $INIT_TF_PATH."
            return
        fi

        # Read the patch content
        PATCH_CONTENT=$(cat "$PATCH_FILE")

        INDENT="        "  # Use 8 spaces for indentation
        PATCH_CONTENT=$(sed "s/^/$INDENT/" "$PATCH_FILE")  # Add indentation to each line in the patch file

        # Read each line from the patch file
        while IFS= read -r line; do sed -i.bak "/kubectl apply -k \/var\/post_install/i\\\t\t\t\t$line" "$INIT_TF_PATH"; done < "$PATCH_FILE"
        echo "Patch applied successfully. A backup of the original file is saved as ${INIT_TF_PATH}.bak."
      }

      # create .env files from default template if doesn't exist
      create_env_files() {
              # Find all *.env.template files
              find . -name "*.env.template" | while IFS= read -r template_file; do
                      # Extract filename without extension
                      filename=$(basename "$template_file" | cut -d '.' -f 1)
                      env_file="$(dirname "$template_file")/$filename.env"

                      # Check if .env file already exists
                      if [ ! -f "$env_file" ]; then
                          # Create a new .env file from the template in the same directory
                          cp "$template_file" "$env_file" 
                          echo "created env file file://$env_file from $template_file"
                      fi
              done

          _related_commands() {
              find . -name '.env.template' 
              sed "s/<username>/your_username/g;s/<password>/your_password/g;s/YOUR_API_KEY/your_actual_api_key/g;s/YOUR_SECRET_KEY/your_actual_secret_key/g" < .env.template > .env
          }

      }

      hcloud context create "k8s-project" || true

      ### [manual] set variables using "terraform.tfvars" or CLI argument or equivalent env variables (with `TF_TOKEN_*` prefix)
      find . -name "*.tfvars"
      create_env_files
      set -a && source ".env" && set +a # export TF_TOKEN_app_terraform_io="" 

      export TF_LOG=DEBUG
      terraform init --upgrade # installed terraform module dependecies
      terraform validate

      patch_init_tf_file
      t_plan="$(mktemp).tfplan" && terraform plan -no-color -out $t_plan
      terraform apply -auto-approve $t_plan

      echo "Successfully setup Hetzner cloud resources."

      {
        read -p "Do you want to continue with infrastructure kubernetes services installations ? (y/n): " yn
        case $yn in
            [Yy]* ) echo "Continuing..."; echo "";;
            [Nn]* ) echo "Exiting script."; return;;
            * ) echo "Please answer yes (y) or no (n).";;
        esac
      }

      install_kubernetes_resources() { 
        generate_kubeconfig
        
        kubectl ctx k3s

        remove_warnings_logs
        sleep 1 
        install_kubernetes_dashboard  
        install_gateway_api_cilium  # [previous implementation] installation_gateway_controller_nginx 
        cert_manager_related  # must be restarted after installation of Gateway Api

        sleep 30
        install_storage_class 

        # TODO: check resource limits and prevent contention when using monitoring tools - check notes in install_monitoring.sh
        # install_monitoring

        popd 

        install_envoy_gateway_class
        # DEPRECATED_install_stackgres_operator
        install_cloudnativepg_operator
        install_minio_operator
        install.kafka-operator#task@infrastructure

        echo "Successfully setup Hetzner cloud + kubernetes core app services."
      }

      install_kubernetes_resources

      verify_installation() {
        echo "export HCLOUD_TOKEN=<...>"

        k9s # https://k9scli.io/topics/commands/
        kubectl get all -A 
        kubectl --kubeconfig $kubeconfig get all -A 
        kubectl get configmap -A
        kubectl get secrets -A
        kubectl api-resources
        kubectl api-versions
        kubectl get gatewayclasses
        hcloud all list
        terraform show
        terraform state list
        terraform state show type_of_resource.label_of_resource

        helm list -A --all-namespaces 
        helm get values --all nginx -n nginx 
        helm get manifest nginx -n nginx 
        
        journalctl -r -n 200

        # load balancer hetzner manager (Hetzner Cloud Controller Manager (CCM))
        kubectl logs -n kube-system -l app=hcloud-cloud-controller-manager

        # volumes: pvc < pv < volumeattachment
        # [NOTE ISSUE - volume stuck on terminating state] usaully have protection finalizers; most likely CSI driver has not released the volume
        # NOTE: could be caused by OOM-Kill - Out Of Memory Kill when processes exceed allocatable capacity for their pods
        kubectl get volume -n longhorn-system
        kubectl get pvc -A
        kubectl get pods -A -o wide
        # CSI storage drivers logs
        kubectl get pods --all-namespaces | grep csi
        kubectl get volumeattachments.storage.k8s.io -o custom-columns="NAME:.metadata.name,ATTACHER:.spec.attacher,PV:.spec.source.persistentVolumeName,NODE:.spec.nodeName,ATTACHED:.status.attached"
        kubectl get csidrivers
        # Longhorn Manager logs - longhorn-engine and volumes
        kubectl get engines -n longhorn-system
        # kubectl describe engine pvc-... -n longhorn-system
        # kubectl get volumes.longhorn.io -n longhorn-system | grep pvc-..
        # kubectl describe volume -n longhorn-system pvc-...
        kubectl get crds | grep longhorn
        kubectl get volumes.longhorn.io -n longhorn-system
        kubectl get engines.longhorn.io -n longhorn-system
        kubectl get replicas.longhorn.io -n longhorn-system
        kubectl get nodes.longhorn.io -n longhorn-system
        kubectl get instancemanagers.longhorn.io -n longhorn-system
        kubectl get backingimages.longhorn.io -n longhorn-system
        kubectl top pod -n longhorn-system

        # check cpu/mem utilization
        kubectl get node && kubectl top nodes && kubectl describe node
        kubectl get pods -o wide -A 
        kubectl get pods -n longhorn-system -l app.kubernetes.io/name=longhorn -o wide 
          # NOTE: memory reporting it seems because of cilium is not reported correctly. (discrepancies found between linux command reported memory and kubectl command one)
        kubectl top pods --containers=true -A --sort-by memory
        kubectl get pods -o wide --all-namespaces | grep k3s-control-plane-wug
        HIGHEST_CPU_NODE=$(kubectl top nodes | awk 'NR>1 {print $1, $2+0}' | sort -k2 -nr | head -n 1 | awk '{print $1}')
        kubectl top pods -A -n $HIGHEST_CPU_NODE --sort-by cpu
        # list all pods of controller nodes: 
        {
          for node in $(kubectl get nodes -l role=control-plane -o jsonpath='{.items[*].metadata.name}'); do
            echo "Pods on node: $node"
            kubectl get pods --all-namespaces -o wide --field-selector spec.nodeName=$node
          done
        }

        hcloud server list
        {
          free | awk '/Mem:/ {printf "Memory Usage: %.2f%%\n", $3/$2 * 100}'
        }

        ### ssh into remove machines
        # echo "" > ~/.ssh/known_hosts # clear known hosts to permit connection for same assigned IP to different server
        ip_address=$(hcloud server list --output json | jq -r '.[0].public_net.ipv4.ip')
        ssh -p 2220 root@$ip_address
        ip_address=$(hcloud server list --output json | jq -r '.[0].public_net.ipv6.ip' | sed 's/\/.*/1/')
        ssh -p 2220 root@$ip_address 
      }

    }
}

delete.longhorn-system@provision-script() { 
  NAMESPACE="longhorn-system"

  echo "Checking if namespace '$NAMESPACE' is stuck in Terminating..."

  # Check if namespace exists and is terminating
  STATUS=$(kubectl get namespace "$NAMESPACE" -o jsonpath='{.status.phase}' 2>/dev/null)

  if [[ "$STATUS" != "Active" && -n "$STATUS" ]]; then
      echo "Namespace '$NAMESPACE' is in status: $STATUS. Proceeding with force delete..."

      echo "Dumping current namespace definition..."
      kubectl get namespace "$NAMESPACE" -o json > ns.json

      echo "Removing finalizers..."
      jq 'del(.spec.finalizers)' ns.json > ns-clean.json

      echo "Sending request to remove finalizers..."
      kubectl replace --raw "/api/v1/namespaces/$NAMESPACE/finalize" -f ns-clean.json

      echo "Namespace '$NAMESPACE' force deleted (finalizers removed)."

      # Cleanup
      rm ns.json ns-clean.json
  else
      echo "Namespace '$NAMESPACE' is not terminating or does not exist. Status: $STATUS"
  fi
}

info.kubernetes@infrastructure() { 
  hcloud server list
  kubectl top pod -A --sort-by='memory'
  kubectl top node --show-capacity=true

  kubectl cluster-info
  cilium status

  kubectl get pvc --all-namespaces
  kubectl get storageclass
  kubectl get nodes -o wide
  kubectl get daemonset -A

  helm list -A

  {
    hcloud server metrics "k3s-worker-small-tty" --type cpu
    
    kubectl drain "k3s-worker-small-ydb" --ignore-daemonsets --delete-emptydir-data
  }
}

storage.info.kubernetes@infrastructure() { 
  if ! hcloud server list &> /dev/null; then
    echo "❌ hcloud CLI is not authenticated. Please check your HCLOUD_TOKEN."
    echo "ERROR: Failed to retrieve server list from Hetzner Cloud."
    echo "       Please check your HCLOUD_TOKEN or hcloud CLI configuration."
    return 1

  fi 

  # Get all server public IPv4 addresses
  # .[] | .public_net.ipv4.ip will extract the IP from each server object in the array
  SERVER_IPS=$(hcloud server list --output json 2>/dev/null | jq -r '.[] | .public_net.ipv4.ip')

  if [ $? -ne 0 ]; then
      echo "ERROR: Failed to retrieve server list from Hetzner Cloud."
      echo "       Please check your HCLOUD_TOKEN or hcloud CLI configuration."
      return 1
  fi

  # Convert string of IPs to an array (readarray is more reliable than IFS+read combo)
  readarray -t ips_array <<< "$SERVER_IPS"

  if [ ${#ips_array[@]} -eq 0 ]; then
      echo "WARNING: No Hetzner Cloud server public IPv4 addresses found."
      return  0
  fi

  echo "Found ${#ips_array[@]} server IP(s)."
  echo "--------------------------------------------------"

  # Loop through each IP and execute the command
  for ip_address in "${ips_array[@]}"; do
      if [ -z "$ip_address" ] || [ "$ip_address" == "null" ]; then
          echo "Skipping empty or null IP address."
          continue
      fi

      echo "Executing command for IP: $ip_address"
      echo "-----------------------------------"

      # ssh -p 2220 root@$ip_address df -h
      
      echo "➤ Calculating total storage usage percentage on $ip_address..."
      ssh -p 2220 root@"$ip_address" \
        "df -B1 | awk '\$1 ~ /^\/dev\// {used+=\$3; total+=\$2} END {if (total > 0) printf(\"Total used: %.2f%%\n\", used/total*100); else print \"No data.\"}'"

      echo ""
      echo "➤ Showing individual volume usage (% used per mounted volume) on $ip_address..."
      ssh -p 2220 root@"$ip_address" \
        "df -h --output=source,pcent,target | grep -E '^/dev/'"

  done; 
}


info.diagnose_cluster_health_issues.kubernetes@infrastructure() {
  kubectl get nodes -o wide
  kubectl get pods -A -o wide

  kubectl events
  hcloud server list
  kubectl get nodes -o wide
  kubectl get pods -A -o wide
  kubectl get httproute -A

  kubectl -n kube-system logs -l app.kubernetes.io/name=cilium-operator
  kubectl -n kube-system get configmap cilium-config -o yaml
  helm get values cilium -n kube-system
}