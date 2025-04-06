#!/bin/bash
set -e

rust_script#setup#task@monorepo() {(
    # Find all .script.rs files recursively from current directory
    local rust_script_files=()
    mapfile -t rust_script_files < <(find . -type f -name "*.script.rs")

    # Make each found file executable
    for script in "${rust_script_files[@]}"; do
        chmod +x "$script"
    done

    echo "All rust scripts are now executable"
)}