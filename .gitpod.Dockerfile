FROM gitpod/workspace-full:2023-04-14-07-10-23
# checkov:skip=BC_DKR_2: ADD REASON

USER gitpod

# Install your tools here

RUN brew install \
    tmux \
    bat \
    tldr \
    ripgrep \
    exa \
    fzf \
    gitleaks \
    markdownlint-cli \
    direnv \
    commitizen \
    tree \
    yq \
    yamllint \
    pre-commit
