#!/bin/bash

deploy#task@auth-ory-stack() {
    skaffold dev -p local-production --port-forward
}

delete_deploy#task@auth-ory-stack() {
    skaffold delete -p local-production
}
