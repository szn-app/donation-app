#!/bin/bash
set -e

# https://stackgres.io/doc/latest/install/helm/
install_stackgres_operator() {
    helm repo add stackgres-charts https://stackgres.io/downloads/stackgres-k8s/stackgres/helm/

    # parameter options https://stackgres.io/doc/latest/install/helm/ & https://stackgres.io/doc/latest/install/helm/parameters/ & https://stackgres.io/doc/latest/reference/crd/sgconfig/#sgconfigspec
    helm install --create-namespace --namespace stackgres stackgres-operator stackgres-charts/stackgres-operator \
        --set-string adminui.service.type=ClusterIP
        # --set grafana.autoEmbed=true

    {
        kubectl wait -n stackgres deployment -l group=stackgres.io --for=condition=Available
        kubectl get pods -n stackgres -l group=stackgres.io
    }

    verify() {
        {
            # connect to admin ui 
            # https://stackgres.io/doc/latest/administration/adminui/
            POD_NAME=$(kubectl get pods --namespace stackgres -l "stackgres.io/restapi=true" -o jsonpath="{.items[0].metadata.name}")
            kubectl port-forward ${POD_NAME} --address 0.0.0.0 8080:9443 --namespace stackgres

            kubectl get secret -n stackgres stackgres-restapi-admin --template '{{ printf "username = %s\n" (.data.k8sUsername | base64decode) }}'
            kubectl get secret -n stackgres stackgres-restapi-admin --template '{{ printf "password = %s\n" (.data.clearPassword | base64decode) }}'
        }
    }
}

# docs: https://cloudnative-pg.io/documentation/current/architecture/#postgresql-architecture
# operator image: https://github.com/cloudnative-pg/cloudnative-pg/pkgs/container/cloudnative-pg
# operand image: https://github.com/cloudnative-pg/postgres-containers/pkgs/container/postgresql
# community extension of operand image: https://github.com/cloudnative-pg/postgis-containers/pkgs/container/postgis
install_cloudnativepg_operator() {
    install_pg_admin() {
        helm repo add runix https://helm.runix.net
        helm upgrade --debug --install pgadmin4 runix/pgadmin4
        # TODO: use helm values to set appropriate service and credentials - check file pgadmin4-values.yml

        expose() {
            export POD_NAME=$(kubectl get pods --namespace default -l "app.kubernetes.io/name=pgadmin4,app.kubernetes.io/instance=pgadmin4" -o jsonpath="{.items[0].metadata.name}")
            echo "Visit http://127.0.0.1:8080 to use your application"
            kubectl port-forward $POD_NAME 8080:80
        }
    }

    # https://cloudnative-pg.io/documentation/current/image_catalog/
    add_postgresql_image_list() {

        #PostgreSQL Container Images
        kubectl apply -f https://raw.githubusercontent.com/cloudnative-pg/postgres-containers/main/Debian/ClusterImageCatalog-bookworm.yaml
        # PostGIS Container Images
        kubectl apply -f https://raw.githubusercontent.com/cloudnative-pg/postgis-containers/main/PostGIS/ClusterImageCatalog.yaml

        cat << 'EOF' | kubectl apply -f -
# https://cloudnative-pg.io/documentation/current/image_catalog/
# define images for operators to use (any change to this file will trigger an update to all using resources)
apiVersion: postgresql.cnpg.io/v1
kind: ClusterImageCatalog
metadata:
    name: postgresql-extension-images
    namespace: cnpg-system
spec:
    images:
    # https://github.com/voltade/cnpg-supabase/blob/main/Dockerfile
    # https://github.com/orgs/supabase/discussions/31147
    - major: 17
      image: ghcr.io/voltade/cnpg-supabase:17.4-11 
EOF
    }

    # https://cloudnative-pg.io/documentation/current/installation_upgrade/#installation-on-kubernetes
    kubectl cnpg install generate | kubectl apply --server-side -f -
    kubectl wait --for=condition=Available deployment/cnpg-controller-manager -n cnpg-system --timeout=300s

    add_postgresql_image_list

    # TODO: install monitoring 
    # https://cloudnative-pg.io/documentation/current/quickstart/

    install_pg_admin

    verify() { 
        kubectl get deployment -n cnpg-system cnpg-controller-manager
        kubectl get pods -n cnpg-system
        kubectl get crds | grep cnpg
    }
}