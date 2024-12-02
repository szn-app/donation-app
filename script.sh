#!/bin/bash
## Entrypoint for project scripts
# `chmod +x ./script.sh`
# `./script.sh <function-name> [args...]`

# load functions
source ./script/provision_dev.sh
source ./script/dev.sh 

# call function in this script file from commandline argument
{
    fn_name="$1"
    if [[ $# -lt 1 ]]; then
        # This case can be used for executing $`source ./script.sh` to load functions to current shell session.
        echo "source ./script.sh to load functions to current shell session"
        exit 0
    elif ! declare -f "$fn_name" || ! [[ $(type -t "$fn_name") ]]; then # check if defined in current file or sourced declaration 
        echo "Error: function '$fn_name' not declared. "
        exit 1
    else 
        # redirect call to function name provided
        shift
        "$fn_name" "$@"
    fi
}