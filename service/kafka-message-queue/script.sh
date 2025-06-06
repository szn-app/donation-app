#!/bin/bash
## Entrypoint for project scripts
# `chmod +x ./script.sh` && ./script.sh <function-name> [args...]`

source ../../script/module/utility.sh

load_scripts_recursive.util "./script/" 
find_and_register_rust_functions.util "./script"

call_function "$@"
