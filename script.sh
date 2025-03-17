#!/bin/bash
# set -e
## Entrypoint for project scripts
# `chmod +x ./script.sh && ./script.sh <function-name> [args...]`

source ./script/module/utility.sh

# load functions
load_scripts_recursive.util "./script/" 
load_scripts_recursive.util "./infrastructure/" 'script/'
load_scripts_recursive.util "./service/" 'script/'
load_scripts_recursive.util "./service/auth-ory-stack"

call_function "$@"
