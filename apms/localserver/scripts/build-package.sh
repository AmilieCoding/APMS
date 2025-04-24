#!/bin/bash

# Generic package builder script
PACKAGES_DIR="packages"
BUILD_DIR="build"
REPO_DIR="repo"

build_package() {
    local package_name=$1
    
    echo "[INFO] Building package: ${package_name}"
    
    # Check if package exists
    if [ ! -d "${PACKAGES_DIR}/${package_name}" ]; then
        echo "[ERROR] Package ${package_name} not found in ${PACKAGES_DIR}"
        exit 1
    fi
    
    # Create build directories
    mkdir -p "${BUILD_DIR}"
    mkdir -p "${REPO_DIR}/packages"
    mkdir -p "${REPO_DIR}/downloads"
    
    # Run package-specific build script
    if [ -f "${PACKAGES_DIR}/${package_name}/build.sh" ]; then
        echo "[INFO] Running package build script"
        (cd "${PACKAGES_DIR}/${package_name}" && ./build.sh "${BUILD_DIR}")
    else
        echo "[ERROR] No build script found for ${package_name}"
        exit 1
    fi
    
    # Copy metadata
    if [ -f "${PACKAGES_DIR}/${package_name}/metadata.json" ]; then
        cp "${PACKAGES_DIR}/${package_name}/metadata.json" "${REPO_DIR}/packages/${package_name}.json"
    else
        echo "[ERROR] No metadata.json found for ${package_name}"
        exit 1
    fi
}

# If no package specified, show usage
if [ $# -eq 0 ]; then
    echo "Usage: $0 <package-name>"
    exit 1
fi

build_package $1