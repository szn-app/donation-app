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
    kubectl -n kafka delete $(kubectl get strimzi -o name -n kafka)
    kubectl delete pvc -l strimzi.io/name=my-cluster-kafka -n kafka # new kafka instances may fail trying to use the same pvc
}

verify_functionality#example@kafka-message-queue() {
    local bootstrap_cluster_name="kafka-message-queue-cluster-kafka-bootstrap"
    local namespace="kafka-message-queue"

    # consume message from kafka topic at `kafka-message-queue-cluster-kafka-bootstrap:9092`
    { # terminal B
        kubectl -n $namespace run kafka-consumer -ti --image=quay.io/strimzi/kafka:0.45.0-kafka-3.9.0 --rm=true --restart=Never -- bin/kafka-console-consumer.sh --bootstrap-server $bootstrap_cluster_name:9092 --topic my-topic --from-beginning
        # verify that the message is received
    }
    # send message to kafka topic
    { # terminal A 
        kubectl -n $namespace run kafka-producer -ti --image=quay.io/strimzi/kafka:0.45.0-kafka-3.9.0 --rm=true --restart=Never -- bin/kafka-console-producer.sh --bootstrap-server $bootstrap_cluster_name:9092 --topic my-topic
        # write message
    }
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