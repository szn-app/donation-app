#!/bin/bash
set -e

# https://min.io/docs/minio/kubernetes/upstream/operations/install-deploy-manage/deploy-operator-helm.html
# https://min.io/docs/minio/kubernetes/upstream/reference/operator-chart-values.html#minio-operator-chart-values
install_minio_operator() {
    helm repo add minio-operator https://operator.min.io
    helm install operator minio-operator/operator --namespace minio-operator --create-namespace

    verify() {
        helm search repo minio-operator
        kubectl get all -n minio-operator
    }
}
