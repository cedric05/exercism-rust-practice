# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.202.5/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
ARG TOKEN
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

RUN wget https://github.com/exercism/cli/releases/download/v3.0.13/exercism-3.0.13-linux-x86_64.tar.gz && \
    mkdir ~/exercism && \
    tar -zxvf exercism-3.0.13-linux-x86_64.tar.gz  -C ~/exercism/ && \
    sudo cp ~/exercism/exercism /usr/local/bin/exercism && \
    rm -rf ~/exercism/ exercism-3.0.13-linux-x86_64.tar.gz && \
    exercism configure --workspace="$(pwd)"  && \
    exercism configure --token=${TOKEN}
    