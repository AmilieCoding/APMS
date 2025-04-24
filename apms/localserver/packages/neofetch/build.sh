#!/bin/bash

BUILD_DIR="${1:-build}"
PACKAGE_NAME="neofetch"
VERSION="7.1.0"
PACKAGE_DIR="${BUILD_DIR}/${PACKAGE_NAME}-${VERSION}"

echo "[INFO] Building neofetch package..."

# Create package directory structure
mkdir -p "${PACKAGE_DIR}/bin"

# Download neofetch
echo "[INFO] Downloading neofetch..."
curl -L "https://raw.githubusercontent.com/dylanaraps/neofetch/master/neofetch" -o "${PACKAGE_DIR}/bin/neofetch"

# Make executable
chmod +x "${PACKAGE_DIR}/bin/neofetch"

# Create installation script
cat > "${PACKAGE_DIR}/install.sh" << 'EOF'
#!/bin/bash
set -x  # Enable debug output

# Make sure the binary is executable
chmod +x bin/neofetch

# Create symlink
mkdir -p /usr/local/bin
ln -svf "$(pwd)/bin/neofetch" /usr/local/bin/neofetch

# Create config directory
mkdir -p /etc/neofetch
mkdir -p ~/.config/neofetch
EOF

chmod +x "${PACKAGE_DIR}/install.sh"

# Create package
echo "[INFO] Creating package archive..."
(cd "${BUILD_DIR}" && tar czf "${PACKAGE_NAME}-${VERSION}.tar.gz" "${PACKAGE_NAME}-${VERSION}")

# Move to repo downloads directory
mkdir -p "../../repo/downloads"
mv "${BUILD_DIR}/${PACKAGE_NAME}-${VERSION}.tar.gz" "../../repo/downloads/"

echo "[INFO] Package built successfully"