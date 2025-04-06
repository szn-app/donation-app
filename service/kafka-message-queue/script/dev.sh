#!/bin/bash
set -e

skaffold#task@kafka-message-queue() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"
    
    skaffold dev --module kafka-message-queue-generic --profile development --port-forward --tail

    popd
}

render.skaffold#task@kafka-message-queue() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --profile development > "$temp_file" && echo "$temp_file"
    popd
}

delete.skaffold@kafka-message-queue() {
    skaffold delete --profile development
}

manual_delete@kafka-message-queue() {
    kubectl -n kafka-message-queue delete $(kubectl get strimzi -o name -n kafka-message-queue)
    kubectl delete pvc -l strimzi.io/name=my-cluster-kafka -n kafka-message-queue # new kafka instances may fail trying to use the same pvc
}

manual.verify_functionality#example@kafka-message-queue() {
    local bootstrap_cluster_name="kafka-message-queue-cluster-kafka-bootstrap"
    local namespace="kafka-message-queue"

    # Extract JAAS credentials
    credentials=$(kubectl get secret kafka-user -n "$namespace" -o jsonpath="{.data.sasl\.jaas\.config}" | base64 -d)
    # formatted as Java Authentication and Authorization Service
    echo "$credentials" # sasl.jaas.config: org.apache.kafka.common.security.scram.ScramLoginModule required username="kafka-user" password="....";  

    # Create a temporary config file
    config_file=$(mktemp)
    cat <<EOF > "$config_file"
security.protocol=SASL_PLAINTEXT
sasl.mechanism=SCRAM-SHA-512
sasl.jaas.config=$credentials
EOF

    echo "Kafka client config:"
    cat "$config_file"

    # send message to kafka topic
    { # terminal A 
        kubectl -n "$namespace" run kafka-producer --image=quay.io/strimzi/kafka:0.45.0-kafka-3.9.0 --restart=Never --command -- sleep 3600
        kubectl wait --for=condition=Ready pod/kafka-producer -n "$namespace" --timeout=60s
        
        kubectl cp "$config_file" "$namespace/kafka-producer:/tmp/client.properties"

        kubectl -n "$namespace" exec -ti kafka-producer -- bin/kafka-console-producer.sh --bootstrap-server $bootstrap_cluster_name:9092 --topic my-topic --producer.config /tmp/client.properties

        # write message
    }

    # consume message from kafka topic at `kafka-message-queue-cluster-kafka-bootstrap:9092`
    { # terminal B
        kubectl -n "$namespace" run kafka-consumer --image=quay.io/strimzi/kafka:0.45.0-kafka-3.9.0 --restart=Never --command -- sleep 3600
        kubectl wait --for=condition=Ready pod/kafka-consumer -n "$namespace" --timeout=60s
        
        kubectl cp "$config_file" "$namespace/kafka-consumer:/tmp/client.properties"

        kubectl -n "$namespace" exec -ti kafka-consumer -- bin/kafka-console-consumer.sh --bootstrap-server "$bootstrap_cluster_name:9092" --topic my-topic --from-beginning --consumer.config /tmp/client.properties

        # verify that the message is received
    }

    # Cleanup
    kubectl -n "$namespace" delete pod kafka-consumer --force
    kubectl -n "$namespace" delete pod kafka-producer --force
}

verify_running#example@kafka-message-queue() {
    # NOTE: cluster must be deployed to same namespace as the strimzi operator
    
    local cluster_name="kafka-message-queue-cluster"
    local namespace="kafka-message-queue"

    kubectl get kafka,kafkanodepool -n kafka-message-queue
    kubectl get clusterrolebinding -l app=strimzi
    # check kubernetes RBAC permission
    kubectl auth can-i create statefulset --as=system:serviceaccount:$namespace:strimzi-cluster-operator -n $namespace
}