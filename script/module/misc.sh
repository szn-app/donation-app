misc_() {
    # modify permission
    find ./ -maxdepth 4 -name "script.sh" -exec chmod +x {} \;
}

# Wait for terminating resources to complete
wait_for_terminating_resources.kubernetes#utility() {
    echo "Checking for terminating resources..."
    while kubectl get pods --all-namespaces | grep -q Terminating || \
                kubectl get services --all-namespaces | grep -q Terminating || \
                kubectl get deployments --all-namespaces | grep -q Terminating || \
                kubectl get statefulsets --all-namespaces | grep -q Terminating; do
        echo "Waiting for resources to finish terminating..."
        sleep 2
    done
    echo "All terminating resources have been cleaned up"
}

