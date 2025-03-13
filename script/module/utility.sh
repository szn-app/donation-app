filter_functions_by_args() {
    local keywords=("$@")
    local matches=()
    for fn in $(declare -F | awk '{print $3}'); do
        for kw in "${keywords[@]}"; do
            if [[ "$fn" == *"$kw"* ]]; then
                matches+=("$fn")
                break
            fi
        done
    done
    echo "${matches[@]}"
}

execute() {         
    # Execute each function directly from filter_functions_by_args output
    for func in $(filter_functions_by_args "$@"); do
        echo "Executing $func..."
        $func
        local exit_code=$?
        if [[ $exit_code -ne 0 ]]; then
            echo "Error: $func exited with code $exit_code"
        fi
    done
}

filter_script_files_by_directory() {
    local directory="$1"
    local script_files=()
    for file in $(find "$directory" -type f -name "*.sh"); do
        script_files+=("$file")
    done
    echo "${script_files[@]}"
}

filter_script_files_by_args() {
    local root_directory="$1"
    shift
    local keywords=("$@")
    local script_files=()
    
    # If no keywords provided, return all script files
    if [[ ${#keywords[@]} -eq 0 ]]; then
        echo $(filter_script_files_by_directory "$root_directory")
        return
    fi
    
    for file in $(filter_script_files_by_directory "$root_directory"); do
        for kw in "${keywords[@]}"; do
            if [[ "$file" == *"$kw"* ]]; then
                script_files+=("$file")
                break
            fi
        done
    done
    
    echo "${script_files[@]}"
}

load_scripts_recursive() {
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
    # filter_script_files_by_args already handles the case when no keywords are provided
    for script in $(filter_script_files_by_args "$root_directory" "${keywords[@]}"); do
        echo "Sourcing $script..."
        source "$script"
    done
}
