#!/bin/bash

skaffold#task@auth-ory-stack() {
    skaffold dev -p local-production --port-forward
}

delete.skaffold#task@auth-ory-stack() {
    skaffold delete -p local-production
}
