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
