#!/bin/sh
set -e

REPO="oneortwo/soundcharts-cli"
BINARY="sc"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    darwin) OS="apple-darwin" ;;
    linux) OS="unknown-linux-gnu" ;;
    *) echo "error: Unsupported OS: $OS" && exit 1 ;;
esac

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "error: Unsupported architecture: $ARCH" && exit 1 ;;
esac

TARGET="${ARCH}-${OS}"
LATEST=$(curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST" ]; then
    echo "error: Could not determine latest release"
    exit 1
fi

URL="https://github.com/${REPO}/releases/download/${LATEST}/${BINARY}-${TARGET}.tar.gz"
INSTALL_DIR="${HOME}/.local/bin"

echo "Downloading sc ${LATEST} for ${TARGET}..."
mkdir -p "$INSTALL_DIR"
curl -sSL "$URL" | tar xz -C "$INSTALL_DIR"
chmod +x "${INSTALL_DIR}/${BINARY}"

echo "Installed sc to ${INSTALL_DIR}/${BINARY}"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "Add ${INSTALL_DIR} to your PATH to use sc globally."
fi
