#!/usr/bin/env bash
set -e

REPO="armedev/smart-skills"
INSTALL_DIR="${HOME}/.local/bin"

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                echo "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

show_help() {
    cat <<EOF
Usage: $0 [OPTIONS]

Install smart-skills binary to your system.

OPTIONS:
    --dir DIR     Install to DIR (default: ~/.local/bin)
    --help        Show this help message

EXAMPLES:
    $0                          # Install to ~/.local/bin
    $0 --dir /usr/local/bin     # Install to /usr/local/bin

After installation:
    1. Create global skills directory:
       mkdir -p ~/.config/smart-skills/skills

    2. Add your skills to ~/.config/smart-skills/skills/

    3. Initialize a project:
       cd your-project
       smart-skills init
EOF
}

detect_os() {
    case "$(uname -s)" in
        Darwin*) echo "darwin" ;;
        Linux*) echo "linux" ;;
        *) echo "unknown" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *) echo "x86_64" ;;
    esac
}

get_asset_name() {
    local os="$1"
    local arch="$2"

    case "$os" in
        darwin)
            case "$arch" in
                x86_64) echo "smart-skills-x86_64-apple-darwin.tar.gz" ;;
                aarch64) echo "smart-skills-aarch64-apple-darwin.tar.gz" ;;
            esac
            ;;
        linux)
            echo "smart-skills-x86_64-unknown-linux-gnu.tar.gz"
            ;;
    esac
}

get_binary_name() {
    local os="$1"

    case "$os" in
        darwin|linux) echo "smart-skills" ;;
    esac
}

download_and_install() {
    local os="$1"
    local arch="$2"
    local install_dir="$3"

    local asset_name
    asset_name=$(get_asset_name "$os" "$arch")

    local binary_name
    binary_name=$(get_binary_name "$os")

    local download_url="https://github.com/${REPO}/releases/latest/download/${asset_name}"

    echo "Downloading smart-skills..."
    echo "  URL: ${download_url}"
    echo "  Install dir: ${install_dir}"

    mkdir -p "${install_dir}"

    if command -v curl &> /dev/null; then
        curl -sL "${download_url}" -o "/tmp/${asset_name}"
    elif command -v wget &> /dev/null; then
        wget -q "${download_url}" -O "/tmp/${asset_name}"
    else
        echo "Error: curl or wget is required to download smart-skills"
        exit 1
    fi

    echo "Extracting..."
    tar -xzf "/tmp/${asset_name}" -C "/tmp"

    echo "Installing to ${install_dir}..."
    mv "/tmp/${binary_name}" "${install_dir}/"
    chmod +x "${install_dir}/${binary_name}"

    rm -f "/tmp/${asset_name}"

    echo ""
    echo "✓ smart-skills installed successfully!"
    echo ""
    echo "Next steps:"
    echo "  1. Add ${install_dir} to your PATH if not already added"
    echo "  2. Create global skills directory:"
    echo "       mkdir -p ~/.config/smart-skills/skills"
    echo "  3. Add your skills to ~/.config/smart-skills/skills/"
    echo "  4. Initialize a project:"
    echo "       cd your-project"
    echo "       smart-skills init"
}

main() {
    parse_args "$@"

    local os
    os=$(detect_os)

    if [[ "$os" == "unknown" ]]; then
        echo "Error: Unsupported operating system"
        exit 1
    fi

    local arch
    arch=$(detect_arch)

    download_and_install "$os" "$arch" "$INSTALL_DIR"
}

main "$@"
