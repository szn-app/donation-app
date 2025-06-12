#!/bin/bash
set -e  

# call function in this script file from commandline argument
call_function() {
    fn_name="$1"
    if [[ $# -lt 1 ]]; then
        # This case can be used for executing $`source ./script.sh` to load functions to current shell session.
        echo "use 'source ./script.sh' to load functions to current shell session"
    elif ! declare -f "$fn_name" || ! [[ $(type -t "$fn_name") ]]; then
        echo "Error: function '$fn_name' not declared."
        exit 1
    else
        # redirect call to function name provided
        shift
        "$fn_name" "$@"
    fi
}

filter_functions_by_args.util() {
    local keywords=("$@")
    local matches=()
    for fn in $(declare -F | awk '{print $3}'); do
        declare -i matchAll=1
        for kw in "${keywords[@]}"; do
            if [[ "$fn" != *"$kw"* ]]; then
                matchAll=0
                break
            fi
        done
        if [[ $matchAll -eq 1 ]]; then
            matches+=("$fn")
        fi
    done
    echo "${matches[@]}"
}

execute.util() {         
    # execute each function directly from filter_functions_by_args.util output
    for func in $(filter_functions_by_args.util "$@"); do
        echo "Executing $func..."
        $func
        local exit_code=$?
        if [[ $exit_code -ne 0 ]]; then
            echo "Error: $func exited with code $exit_code"
        fi
    done
}

filter_script_files_by_directory.util() {
    local directory="$1"
    local script_files=()
    for file in $(find "$directory" -type f -name "*.sh"); do
        script_files+=("$file")
    done
    echo "${script_files[@]}"
}

filter_script_files_by_args.util() {
    local root_directory="$1"
    shift
    local keywords=("$@")
    local script_files=()
    
    # If no keywords provided, return all script files
    if [[ ${#keywords[@]} -eq 0 ]]; then
        echo $(filter_script_files_by_directory.util "$root_directory")
        return
    fi
    
    for file in $(filter_script_files_by_directory.util "$root_directory"); do
        for kw in "${keywords[@]}"; do
            if [[ "$file" == *"$kw"* ]]; then
                script_files+=("$file")
                break
            fi
        done
    done
    
    echo "${script_files[@]}"
}

load_scripts_recursive.util() {
    local root_directory="$1"  # Get directory from argument
    shift  # Remove first argument (directory) to pass remaining args as keywords
    local keywords=("$@")

    # Validate input
    if [[ -z "$root_directory" ]]; then
        echo "Usage: $0 <script_directory> [keywords...]"
        return 1
    elif [[ ! -d "$root_directory" ]]; then
        echo "Error: '$root_directory' is not a valid directory."
        return 1
    fi

    # Find and source all matching .sh scripts recursively
    # filter_script_files_by_args.util already handles the case when no keywords are provided
    for script in $(filter_script_files_by_args.util "$root_directory" "${keywords[@]}"); do
        echo "Sourcing $script..."
        source "$script"
    done
}

# Find all executable .script.rs files and declare them as shell functions
find_and_register_rust_functions.util() {
    local root_directory="$1"  
    local rust_scripts=$(find $root_directory -path "*/script/*" -name "*.script.rs" -type f -executable)
    
    for script in $rust_scripts; do
        local name=$(basename "$script")
        local dir=$(dirname "$script")

        # Define a shell function that will execute the Rust script
        eval "${name}() {
            \"$script\" \"\$@\"
        }"
        
        echo "Registered Rust script as function: $name from $script"
    done
}
