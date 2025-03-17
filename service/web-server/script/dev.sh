misc@web-server() {
        cargo create-tauri-app
}

dev@web-server() { 
    pnpm run dev
    pnpm run lint -- --debug
    pnpm run lint -- --fix
}

dev.skaffold@web-server() {     
    skaffold dev --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false
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
