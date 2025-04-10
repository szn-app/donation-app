#!/bin/bash
set -e

### contains some scripts used for tools intallaions
# NOTE: these are not used anywhere in the code, they are merely for documentation records

install_tailwindcss() { 
    pnpm install -D tailwindcss postcss autoprefixer
    pnpm dlx tailwindcss init -p
    
    pnpm install -D prettier prettier-plugin-tailwindcss

    example() { 
        pnpm tailwindcss -i ./src/input.css -o ./src/output.css --watch
    }
}

install_shadcn_for_vite() { 
    install_tailwindcss

    # follow https://ui.shadcn.com/docs/installation/vite
    # [manual] setup configs and paths resolution

    pnpm dlx shadcn@latest init
    pnpm dlx shadcn@latest add button
}   

install_nextui() {
    # https://nextui.org/docs/guide/installation#global-installation
    pnpm add @nextui-org/react framer-motion
    echo "public-hoist-pattern[]=*@nextui-org/*" > .npmrc && pnpm install

    pnpm add @nextui-org/button
    # [manual] add component styles to tailwind.config.js
}

install_cilium_cli() { 
    CILIUM_CLI_VERSION=$(curl -s https://raw.githubusercontent.com/cilium/cilium-cli/main/stable.txt)
    CLI_ARCH=amd64
    if [ "$(uname -m)" = "aarch64" ]; then CLI_ARCH=arm64; fi
    curl -L --fail --remote-name-all https://github.com/cilium/cilium-cli/releases/download/${CILIUM_CLI_VERSION}/cilium-linux-${CLI_ARCH}.tar.gz{,.sha256sum}
    sha256sum --check cilium-linux-${CLI_ARCH}.tar.gz.sha256sum
    sudo tar xzvfC cilium-linux-${CLI_ARCH}.tar.gz /usr/local/bin
    rm cilium-linux-${CLI_ARCH}.tar.gz{,.sha256sum}
}

install_kubectl_krew_plugin_manager() { 
    (
    set -x; cd "$(mktemp -d)" &&
    OS="$(uname | tr '[:upper:]' '[:lower:]')" &&
    ARCH="$(uname -m | sed -e 's/x86_64/amd64/' -e 's/\(arm\)\(64\)\?.*/\1\2/' -e 's/aarch64$/arm64/')" &&
    KREW="krew-${OS}_${ARCH}" &&
    curl -fsSLO "https://github.com/kubernetes-sigs/krew/releases/latest/download/${KREW}.tar.gz" &&
    tar zxvf "${KREW}.tar.gz" &&
    ./"${KREW}" install krew
    )

    export PATH="${KREW_ROOT:-$HOME/.krew}/bin:$PATH"

    kubectl krew 
}

install_k8s_tools() { 
    # optional installation for more command options compared to `kubectl kustomize <...>` (kubectly kustomize preinstalled plugin)
    install_kustomize() { 
        TMP=$(mktemp -d)
        pushd $TMP
            curl -s "https://raw.githubusercontent.com/kubernetes-sigs/kustomize/master/hack/install_kustomize.sh"  | bash

            sudo mv kustomize /usr/local/bin/kustomize
        popd
        rm -r $TMP
        kustomize version
    }

    install_kustomize
    brew install derailed/k9s/k9s
    kubectl krew install ctx
    kubectl krew install ns
    brew install fzf

    {
        # NOTE: this works on specific images such as alpine linux
        # [install] apt-get update && apt-get install -y tcpdump
        # apt update && apt install tcpdump -y && \
        #   rm /tmp/static-tcpdump && \
        #   ln /bin/tcpdump /tmp/static-tcpdump
        kubectl krew install sniff # view all connection of a pod 
        kubectl sniff setup-pod-keto -n auth -f "port 80" -o - | tshark -r -

        manual_alternative() {
            tcpdump -i any -U -w - 
        }
    }

}

# https://www.ory.sh/docs/kratos/install#linux
install_ory_cli() {
    bash <(curl https://raw.githubusercontent.com/ory/meta/master/install.sh) -d -b . kratos v1.3.1 && chmod +x ./kratos && sudo mv kratos /usr/bin
    kratos help
}

install_skaffold() { 
    # https://skaffold.dev/docs/install/
    curl -Lo skaffold https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64 && sudo install skaffold /usr/local/bin/ && rm skaffold

}

install_psql() {
    # must install correct version that matches the postgresql server version

    # add repo
    sudo dnf install -y https://download.postgresql.org/pub/repos/yum/reporpms/F-$(rpm -E %fedora)-x86_64/pgdg-fedora-repo-latest.noarch.rpm
    
    sudo dnf install postgresql17
}

kubectl_cnpg_installation() { 
    brew install kubectl-cnpg && brew upgrade kubectl-cnpg

    {
        cat > kubectl_complete-cnpg <<EOF
#!/usr/bin/env sh

# Call the __complete command passing it all arguments
kubectl cnpg __complete "\$@"
EOF

        chmod +x kubectl_complete-cnpg

        # Important: the following command may require superuser permission
        sudo mv kubectl_complete-cnpg /usr/local/bin
    }

    verify() {
        kubectl cnpg install generate --help
    }
}

install_yq( ){ 
    sudo wget https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -O /usr/bin/yq && sudo chmod +x /usr/bin/yq
}

install_mc() { 
    curl https://dl.min.io/client/mc/release/linux-amd64/mc --create-dirs -o $HOME/minio-binaries/mc

    chmod +x $HOME/minio-binaries/mc
    export PATH=$PATH:$HOME/minio-binaries/

    mc --version
}

install_binary_tools#rust@monorepo() {
    cargo install rust-script
    cargo install cargo-binstall
    # cargo binstall cargo-watch
    cargo install cargo-watch --locked
}

install_darling#obsolete() { # runs .dmg MacOS applicaitons on Linux
    sudo dnf install make cmake clang bison dbus-devel flex glibc-devel.i686 fuse-devel \
        systemd-devel elfutils-libelf-devel cairo-devel freetype-devel.{x86_64,i686} \
        libjpeg-turbo-devel.{x86_64,i686} fontconfig-devel.{x86_64,i686} libglvnd-devel.{x86_64,i686} \
        mesa-libGL-devel.{x86_64,i686} mesa-libEGL-devel.{x86_64,i686} mesa-libGLU-devel.{x86_64,i686} \
        libtiff-devel.{x86_64,i686} libxml2-devel libbsd-devel git git-lfs libXcursor-devel \
        libXrandr-devel giflib-devel pulseaudio-libs-devel libxkbfile-devel \
        openssl-devel llvm libcap-devel libavcodec-free-devel libavformat-free-devel

# follow instructions: 
    # https://docs.darlinghq.org/build-instructions.html
    # https://docs.darlinghq.org/installing-software.html

    {
        darling shell
        {
            ls -l /
            uname
            sw_vers

            ls -al /Volumes/SystemRoot/home # mounted host system to the emulated system
            
        }
    }
}

postgres_local_install_for_testing#obsolete() {
    install_supabase_cli() {
        brew install supabase/tap/supabase
    }

    # https://www.postgresql.org/download/linux/redhat/
    sudo dnf install postgresql-server 

    # todo install pgadmin or cli tools on local not kubernetes
}

install_osx_hackintosh() { 
    # TODO:    https://github.com/foxlet/macOS-Simple-KVM
    
    install_kvm() {
        # https://docs.fedoraproject.org/en-US/quick-docs/virtualization-getting-started/
        sudo dnf install @virtualization
        sudo systemctl start libvirtd
        sudo systemctl enable libvirtd
        lsmod | grep kvm
    }
}

install_grpc_dependency() {
    # Rust `tonic` dependnecy to compile protobuf definitions
    sudo dnf install protobuf-compiler protobuf-devel -y

    # install dev tools
    curl -sSL "https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz" | sudo tar -xz -C /usr/local/bin
}