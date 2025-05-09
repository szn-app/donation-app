misc@web-server() {
        cargo create-tauri-app
}

dev@web-server() { 
    pnpm run dev
    pnpm run lint -- --debug
    pnpm run lint -- --fix

    pnpm run generate-graphql
}

skaffold#task@web-server() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    # skaffold dev --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false
    skaffold dev --module web-server --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=true
    popd
}

delete.skaffold#task@web-server() {
    skaffold delete --module web-server --profile development
}

bootstrap@web_server() { 
    # https://typescript-eslint.io/getting-started/
    # https://react-v9.holt.courses/lessons/tools/linting
    install_eslint() {
        pnpm add --save-dev eslint @eslint/js typescript typescript-eslint eslint-config-prettier
        pnpm run eslint --init
    }

    # https://react-v9.holt.courses/lessons/tools/code-formatting
    install_prettier() { 
        pnpm install --save-dev prettier
        # create prettier.config.js
    }

    install_shadcn() { 
        pnpm dlx shadcn@latest init
    }

    install_tanstack_router() {
        # https://tanstack.com/router/latest/docs/framework/react/quick-start#using-file-based-route-generation
        echo ''
     }

     install_codegen_tools() {
        # https://github.com/dotansimha/graphql-code-generator
        # https://the-guild.dev/graphql/codegen/docs/guides/react-query

        pnpm add graphql
        pnpm add -D typescript @graphql-codegen/cli

        pnpm add --save-dev @graphql-codegen/cli @parcel/watcher
        pnpm add --save-dev @graphql-codegen/schema-ast
        pnpm add --save-dev @0no-co/graphqlsp

        # manual setting of cofig files
        pnpm install
     }
}

add_shadcn_components@web-server() { 
    pnpm dlx shadcn@latest add "component-name" 
}

## IMPORTANT! used in .github/workflows/*
build_react_spa@web-server() {
    pushd ./service/web-server

    pnpm install --frozen-lockfile
    pnpm run build
    
    popd
}

develop_tauri_desktop_with_workaround_black_screen@web-server() { 
    cd ./service/web-server
    WEBKIT_DISABLE_COMPOSITING_MODE=1 cargo tauri dev
}

develop_tauri_android@web-server() { 
    ./script.sh setup_android_sdk_variables

    cargo tauri android init
    cargo tauri android dev
}

develop_pnpm_react@web-server() { 
    cd web-server
    pnpm install
    # run application development
    WEBKIT_DISABLE_COMPOSITING_MODE=1 cargo tauri dev
    # or 
    pnpm run dev
}

build_app@web-server() {
    pnpm install
    NO_STRIP=true cargo tauri build 
    # run application
    WEBKIT_DISABLE_COMPOSITING_MODE=1 ./src-tauri/target/release/bundle/appimage/*.AppImage
}

generate_graphql_codegen#task@web-server() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 

    npx graphql-codegen --config codegen.ts
    
    watch_mode() { 
        npx graphql-codegen --config codegen.ts --watch
    }

    popd
}
