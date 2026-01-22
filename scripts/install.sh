#!/bin/bash
# Briefcase Installer for Linux/macOS
set -e

echo "Briefcase Backup Tool Installer"
echo "==============================="

# Detect architecture and OS
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

if [ "$OS" = "linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        BINARY="briefcase-linux-x64.tar.gz"
    elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
        BINARY="briefcase-linux-arm64.tar.gz"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
elif [ "$OS" = "darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        BINARY="briefcase-macos-x64.tar.gz"
    elif [ "$ARCH" = "arm64" ]; then
        BINARY="briefcase-macos-arm64.tar.gz"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# Download URL (replace with actual GitHub release URL)
DOWNLOAD_URL="https://github.com/br8km/briefcase/releases/latest/download/$BINARY"

echo "Downloading $BINARY for $OS $ARCH..."
curl -L -o "$BINARY" "$DOWNLOAD_URL"

echo "Extracting..."
if [[ "$BINARY" == *.tar.gz ]]; then
    tar -xzf "$BINARY"
else
    gunzip "$BINARY"
fi

echo "Installing to /usr/local/bin..."
sudo mv briefcase /usr/local/bin/
sudo chmod +x /usr/local/bin/briefcase

echo "Cleaning up..."
rm -f "$BINARY"

echo "Installation complete!"
echo "Run 'briefcase --help' to get started."