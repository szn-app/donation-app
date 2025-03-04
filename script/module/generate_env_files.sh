env_files() {
    local environment="$1"

    _related_commands() {
        find . -name '.env.template' 
        sed "s/<username>/your_username/g;s/<password>/your_password/g;s/YOUR_API_KEY/your_actual_api_key/g;s/YOUR_SECRET_KEY/your_actual_secret_key/g" < .env.template > .env
    }

    # create .env files from default template if doesn't exist
    create_env_files() {
            # Find all *.env.template files
            find . -name "*.env.template" | while IFS= read -r template_file; do
                    # Extract filename without extension
                    filename=$(basename "$template_file" | cut -d '.' -f 1)
                    env_file="$(dirname "$template_file")/$filename.env"

                    # Check if .env file already exists
                    if [ ! -f "$env_file" ]; then
                        # Create a new .env file from the template in the same directory
                        cp "$template_file" "$env_file" 
                        echo "created env file file://$env_file from $template_file"
                    fi
            done
    }

    generate_database_kratos_credentials
    generate_default_username_kratos
    generate_database_hydra_credentials
    generate_database_keto_credentials
    generate_secret_auth_ui $environment
    create_env_files
}
