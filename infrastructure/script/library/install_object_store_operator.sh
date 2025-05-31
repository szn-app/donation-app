#!/bin/bash
set -e

# https://min.io/docs/minio/kubernetes/upstream/operations/install-deploy-manage/deploy-operator-helm.html
# https://min.io/docs/minio/kubernetes/upstream/reference/operator-chart-values.html#minio-operator-chart-values
install_minio_operator() {
    RELEASE_NAME="operator"
    NAMESPACE="minio-operator"
    HELM_REPO_NAME="minio-operator"
    HELM_REPO_URL="https://operator.min.io"
    VALUES_FILE="infrastructure/helm_values/minio-operator-values.yaml"

    echo "Checking if MinIO Operator Helm release '${RELEASE_NAME}' exists in namespace '${NAMESPACE}'..."

    # Check if the helm release exists
    if helm list -n "${NAMESPACE}" | grep -q "${RELEASE_NAME}"; then
        echo "MinIO Operator Helm release '${RELEASE_NAME}' already exists in namespace '${NAMESPACE}'. Skipping installation."
    else
        echo "MinIO Operator Helm release '${RELEASE_NAME}' not found. Proceeding with installation."

        helm repo add "${HELM_REPO_NAME}" "${HELM_REPO_URL}"
        helm install "${RELEASE_NAME}" "${HELM_REPO_NAME}/operator" --namespace "${NAMESPACE}" --create-namespace --values "${VALUES_FILE}"
        echo "MinIO Operator installed."
    fi
    
    verify() {
        helm search repo minio-operator
        kubectl get all -n minio-operator
    }
}