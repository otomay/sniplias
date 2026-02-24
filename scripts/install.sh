#!/bin/sh
set -e

# Check if output is a TTY for color support
if [ -t 1 ]; then
    # Colors for output (matching app theme: purple/cyan accent, modern terminal aesthetic)
    PURPLE='\033[0;35m'       # Purple (primary accent)
    BRIGHT_PURPLE='\033[1;35m' # Bold purple
    CYAN='\033[0;36m'         # Cyan (secondary accent)
    BRIGHT_CYAN='\033[1;36m'  # Bold cyan
    RED='\033[0;31m'          # Red (for errors)
    BRIGHT_RED='\033[1;31m'   # Bright red
    GREEN='\033[0;32m'        # Green (for success)
    BRIGHT_GREEN='\033[1;32m'  # Bright green
    YELLOW='\033[1;33m'       # Yellow (for warnings)
    NC='\033[0m'              # No Color
else
    # No colors if not a TTY
    PURPLE=''
    BRIGHT_PURPLE=''
    CYAN=''
    BRIGHT_CYAN=''
    RED=''
    BRIGHT_RED=''
    GREEN=''
    BRIGHT_GREEN=''
    YELLOW=''
    NC=''
fi

ASCII_LOGO="
                                            
                                            
██████ ▄▄  ▄▄ ▄▄ ▄▄▄▄  ▄▄    ▄▄  ▄▄▄   ▄▄▄▄ 
▀▀▀▄▄▄ ███▄██ ██ ██▄█▀ ██    ██ ██▀██ ███▄▄ 
█████▀ ██ ▀██ ██ ██    ██▄▄▄ ██ ██▀██ ▄▄██▀ 
                                            
"

REPO="otomay/sniplias"
BINARY_NAME="sniplias"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Print ASCII art header with purple color
printf "${BRIGHT_PURPLE}${ASCII_LOGO}${NC}\n\n"
printf "${BRIGHT_GREEN}Installing ${BINARY_NAME}...${NC}\n\n"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) printf "${BRIGHT_RED}Unsupported architecture: ${ARCH}${NC}\n"; exit 1 ;;
esac

# Check for supported OS
case "$OS" in
    darwin|linux) ;;
    *) printf "${BRIGHT_RED}Unsupported operating system: ${OS}${NC}\n"; exit 1 ;;
esac

printf "${CYAN}Detected: ${OS}/${ARCH}${NC}\n\n"

# Get the latest release tag FIRST (needed for version comparison)
printf "${CYAN}Fetching latest release...${NC}\n"
LATEST=$(curl -sL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)

if [ -z "$LATEST" ]; then
    printf "${BRIGHT_RED}Failed to fetch latest release${NC}\n"
    exit 1
fi

printf "${CYAN}Latest version: ${LATEST}${NC}\n\n"

# Check for existing installations
check_existing_installation() {
    EXISTING_INSTALLS=""
    NEEDS_UPGRADE=""
    
    # Check yay (AUR packages)
    if command -v yay >/dev/null 2>&1; then
        if yay -Qs "^${BINARY_NAME}$" >/dev/null 2>&1; then
            EXISTING_INSTALLS="${EXISTING_INSTALLS}\n  - yay/AUR"
        fi
    fi
    
    # Check cargo
    if command -v cargo >/dev/null 2>&1; then
        if cargo install --list 2>/dev/null | grep -q "^${BINARY_NAME} "; then
            CARGO_VERSION=$(cargo install --list 2>/dev/null | grep "^${BINARY_NAME} " | awk '{print $2}')
            EXISTING_INSTALLS="${EXISTING_INSTALLS}\n  - cargo: ${CARGO_VERSION}"
        fi
    fi
    
    # Check Homebrew
    if command -v brew >/dev/null 2>&1; then
        if brew list --formula 2>/dev/null | grep -q "^${BINARY_NAME}$"; then
            BREW_VERSION=$(brew list --formula ${BINARY_NAME} 2>/dev/null | head -1)
            EXISTING_INSTALLS="${EXISTING_INSTALLS}\n  - Homebrew: ${BREW_VERSION}"
        fi
    fi
    
    # Check if binary already exists in common locations (LOCAL)
    for dir in /usr/local/bin /usr/bin ~/.local/bin; do
        EXPANDED_DIR=$(eval echo "$dir")
        if [ -f "$EXPANDED_DIR/${BINARY_NAME}" ]; then
            # Try to get version from existing binary
            INSTALLED_VERSION=$($EXPANDED_DIR/${BINARY_NAME} --version 2>/dev/null | head -1)
            if [ -n "$INSTALLED_VERSION" ]; then
                # Extract version number (handle various formats)
                INSTALLED_VER=$(echo "$INSTALLED_VERSION" | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
                REMOTE_VER=$(echo "$LATEST" | sed 's/^v//')
                
                if [ "$INSTALLED_VER" = "$REMOTE_VER" ]; then
                    printf "${GREEN}✓ ${BINARY_NAME} ${INSTALLED_VER} is already installed (latest)${NC}\n"
                    exit 0
                else
                    EXISTING_INSTALLS="${EXISTING_INSTALLS}\n  - ${dir}: ${INSTALLED_VER} → ${REMOTE_VER}"
                    NEEDS_UPGRADE="yes"
                fi
            else
                EXISTING_INSTALLS="${EXISTING_INSTALLS}\n  - ${dir} (manual)"
            fi
        fi
    done
    
    if [ -n "$EXISTING_INSTALLS" ]; then
        printf "${YELLOW}Found existing installation(s):${NC}${EXISTING_INSTALLS}\n"
        
        if [ -n "$NEEDS_UPGRADE" ]; then
            printf "\n${CYAN}Upgrade to ${LATEST}? [Y/n]: ${NC}"
            DEFAULT_ANSWER="y"
        else
            printf "\n${YELLOW}This script will install to ${INSTALL_DIR}, which may conflict.${NC}\n"
            printf "${CYAN}Continue anyway? [y/N]: ${NC}"
            DEFAULT_ANSWER="n"
        fi
        
        # Read answer (handle non-interactive mode)
        if [ -t 0 ]; then
            read -r answer || answer="$DEFAULT_ANSWER"
            # If empty, use default
            if [ -z "$answer" ]; then
                answer="$DEFAULT_ANSWER"
            fi
        else
            answer="y"
        fi
        
        case "$answer" in
            y|Y) 
                printf "${CYAN}Proceeding...${NC}\n\n"
                ;;
            *)
                printf "${YELLOW}Installation cancelled.${NC}\n"
                exit 0
                ;;
        esac
    fi
}

# Run existing installation check
check_existing_installation

# Construct download URL based on OS
case "$OS" in
    darwin)
        URL="https://github.com/$REPO/releases/download/$LATEST/${BINARY_NAME}-${OS}-${ARCH}"
        ;;
    linux)
        URL="https://github.com/$REPO/releases/download/$LATEST/${BINARY_NAME}-${OS}-${ARCH}"
        ;;
esac

# Download the binary
printf "${CYAN}Downloading ${BINARY_NAME} ${LATEST} for ${OS}/${ARCH}...${NC}\n"
if ! curl -sL "$URL" -o "$BINARY_NAME"; then
    printf "${BRIGHT_RED}Failed to download binary${NC}\n"
    exit 1
fi

chmod +x "$BINARY_NAME"

# Determine install location and install
printf "${CYAN}Installing to ${INSTALL_DIR}...${NC}\n"

if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
else
    # Try with sudo if we don't have write access
    if command -v sudo >/dev/null 2>&1; then
        sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    else
        # Fallback to user's local bin
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
        mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
        printf "${YELLOW}Installed to ${INSTALL_DIR} (no sudo available)${NC}\n"
    fi
fi

# Verify installation
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    # Check if the binary is in PATH
    if ! command -v "$BINARY_NAME" >/dev/null 2>&1; then
        printf "\n${YELLOW}Warning: ${BINARY_NAME} may not be in your PATH.${NC}\n"
        printf "${YELLOW}Add ${INSTALL_DIR} to your PATH if needed:${NC}\n"
        printf "${CYAN}  export PATH=\"\$PATH:${INSTALL_DIR}\"${NC}\n"
    fi

    printf "\n${BRIGHT_GREEN}✓ ${BINARY_NAME} ${LATEST} installed successfully!${NC}\n"
    printf "${BRIGHT_GREEN}Run '${BINARY_NAME}' to start managing your aliases and snippets.${NC}\n"
else
    printf "${BRIGHT_RED}Installation failed${NC}\n"
    exit 1
fi
