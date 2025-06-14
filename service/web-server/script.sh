#!/bin/bash
## Entrypoint for project scripts
# `chmod +x ./script.sh` && ./script.sh <function-name> [args...]`

source ../../script/module/script_discovery_execution.sh

load_scripts_recursive.util "./script/" 
load_scripts_recursive.util "./k8s/" 
find_and_register_rust_functions.util "./script"

call_function "$@"
