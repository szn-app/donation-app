# Strimzi Kafka operator https://strimzi.io/quickstarts/
install.kafka-operator#task@infrastructure() {
    local operator_namespace="kafka-operator"
    local strimzi_deployment_name="strimzi-cluster-operator"
    
    # Create the namespace for the Kafka Operator
    kubectl create namespace "$operator_namespace" 2>/dev/null || true    

    # Check if the Strimzi Cluster Operator deployment exists
    if kubectl get deployment -n "$operator_namespace" "$strimzi_deployment_name" &> /dev/null; then
        echo "Strimzi Cluster Operator '${strimzi_deployment_name}' already exists in namespace '${operator_namespace}'. Skipping installation."
        return 0 # Exit the function if already installed
    else
        echo "Strimzi Cluster Operator '${strimzi_deployment_name}' not found. Proceeding with installation."
    fi

    # Patch operator installation to watch all namespaces - https://strimzi.io/docs/operators/latest/full/deploying.html#deploying-cluster-operator-to-watch-whole-cluster-str
    tmp_dir=$(mktemp -d)
    pushd "$tmp_dir"
    {
        # Download Strimzi installation files into the temporary directory
        # TODO: update to 0.46.0 to support kafka
        local strimzi_version="0.45.0"
        curl -L "https://github.com/strimzi/strimzi-kafka-operator/releases/download/$strimzi_version/strimzi-$strimzi_version.zip" -o strimzi-$strimzi_version.zip && unzip strimzi-$strimzi_version.zip
        pushd "strimzi-$strimzi_version"
        {
            # modify Strimzi operator to watch all namespaces
            sed -i "s/namespace: .*/namespace: $operator_namespace/" install/cluster-operator/*RoleBinding*.yaml

            # Set STRIMZI_NAMESPACE to watch all namespaces (set it to value: "*")
            # NOTE: this automation is not perfect and may require modifications in future versions
            {
                # Replace valueFrom with value: "*" if it exists
                sed -i '/name: STRIMZI_NAMESPACE/{n;s/valueFrom:.*/value: "*"/;}' install/cluster-operator/060-Deployment-strimzi-cluster-operator.yaml
                # Remove fieldRef and fieldPath blocks if they exist
                sed -i '/name: STRIMZI_NAMESPACE/,/- name\|]/{/fieldRef:/d; /fieldPath:/d;}' install/cluster-operator/060-Deployment-strimzi-cluster-operator.yaml
                # Ensure value is "*" (this handles the case where valueFrom doesn't exist)
                sed -i '/name: STRIMZI_NAMESPACE/{n;s/value: .*/value: "*"/;}' install/cluster-operator/060-Deployment-strimzi-cluster-operator.yaml
            }
        
            # install customized operator
            kubectl apply -f install/cluster-operator -n "$operator_namespace"
        
            # Create ClusterRoleBindings for cluster-wide access
            kubectl create clusterrolebinding strimzi-cluster-operator-namespaced --clusterrole=strimzi-cluster-operator-namespaced --serviceaccount $operator_namespace:strimzi-cluster-operator
            kubectl create clusterrolebinding strimzi-cluster-operator-watched --clusterrole=strimzi-cluster-operator-watched --serviceaccount $operator_namespace:strimzi-cluster-operator
            kubectl create clusterrolebinding strimzi-cluster-operator-entity-operator-delegation --clusterrole=strimzi-entity-operator --serviceaccount $operator_namespace:strimzi-cluster-operator
        }
        popd
    }
    popd

    # Wait for the operator to be ready
    kubectl wait --for=condition=Ready pod -l name=strimzi-cluster-operator -n $operator_namespace --timeout=300s

    echo "Kafka Operator installed successfully in the $operator_namespace namespace, watching all namespaces for Kafka Strimzi CRDs."
}

delete.kafka-operator@infrastructure() {
    local operator_namespace="kafka-operator"

    kubectl -n $operator_namespace delete -f "https://strimzi.io/install/latest?namespace=$operator_namespace"
    kubectl delete namespace $operator_namespace

    kubectl delete clusterrolebinding strimzi-cluster-operator-namespaced
    kubectl delete clusterrolebinding strimzi-cluster-operator-watched
    kubectl delete clusterrolebinding strimzi-cluster-operator-entity-operator-delegation
}

verify.kafka-operator@infrastructure() {
    kubectl get pod -n $operator_namespace --watch

    # follow the operator’s log:
    kubectl logs deployment/strimzi-cluster-operator -n $operator_namespace -f
    
    get_strimzi_version() {
        kubectl get deployment strimzi-cluster-operator -n $operator_namespace -o yaml | grep image: | awk '{print $2}'
    }
}
