#!/bin/bash

## IMPORTANT! used in .github/workflows/*
build_container#package_hook@auth-ui() {
    pushd kratos-selfservice-ui-node
    
    docker build . -t auth-ui:latest

    popd
}
