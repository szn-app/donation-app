#!/bin/bash

misc@api-data-server() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yaml workflow
production.build_container#package_hook@api-data-server() {
    docker build . --target release -t api-data-server:latest # --build-arg ENV=production
}

build_container#package_hook@api-data-server() {
    docker build . --target debug -t api-data-server:latest # --build-arg ENV=development
}

# IMPORTANT! used by docker image build; & github workflow
install.system-dependency@api-data-server() {
    # Rust protobuf compiler dependency
    apt update && apt upgrade -y && apt install -y protobuf-compiler libprotobuf-dev
}

run_container@api-data-server() {
    docker run -d -p 80:80 api-data-server
}

copy_secret_default_admin_user_credentials@api-data-server() {
  local secret_name=default-admin-user-credentials
  local source_ns=auth
  local target_ns=api-data

  echo "Copying secret '$secret_name' from namespace '$source_ns' to '$target_ns' (will overwrite if exists)..."

  kubectl get secret "$secret_name" -n "$source_ns" -o json | \
    jq 'del(.metadata.namespace, .metadata.resourceVersion, .metadata.uid, .metadata.creationTimestamp, .metadata.managedFields)' > /tmp/secret-copy.json

  # Set the correct namespace in the JSON
  jq --arg ns "$target_ns" '.metadata.namespace = $ns' /tmp/secret-copy.json > /tmp/secret-copy-patched.json

  # Force replace (delete and create)
  kubectl replace --force -f /tmp/secret-copy-patched.json

  local status=$?
  rm -f /tmp/secret-copy.json /tmp/secret-copy-patched.json

  if [[ $status -eq 0 ]]; then
    echo "Secret successfully copied and replaced."
  else
    echo "Failed to copy the secret."
    return 1
  fi
}

func#predeploy-skaffold-hook@api-data-server() {(
    local environment=$1

    generate_config@api-data-server

    # TODO: synchronize admin user to app database in initialization process (currently it is manually updated if required after deployment)
    # copy_secret_default_admin_user_credentials@api-data-server
)}

func#postdeploy-skaffold-hook@api-data-server() {(
    local environment=$1
)}