#!/usr/bin/env bash

# Deploy SystemWeaver to remote server for testing
# This script:
# 1. Creates a clean zip of the repository (excluding build artifacts)
# 2. Transfers it to the remote server via SCP
# 3. Removes the old version on remote
# 4. Extracts the new version
# 5. Builds the project on remote

set -euo pipefail  # Exit on error, undefined variables, and pipe failures
IFS=$'\n\t'        # Set safe Internal Field Separator

# Configuration
readonly REMOTE_USER="${REMOTE_USER:-dominik}"
readonly REMOTE_HOST="${REMOTE_HOST:-aspiremx}"
readonly REMOTE_PATH="${REMOTE_PATH:-./Development/SystemWeaver}"
readonly SSH_KEY="${SSH_KEY:-$HOME/.ssh/raspberrypi3}"
readonly PROJECT_NAME="systemweaver"
readonly ARCHIVE_NAME="${PROJECT_NAME}_$(date +%Y%m%d_%H%M%S).zip"

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

# Cleanup function
cleanup() {
    if [[ -f "${ARCHIVE_NAME}" ]]; then
        log_info "Cleaning up local archive..."
        rm -f "${ARCHIVE_NAME}"
    fi
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    local missing_tools=()
    
    for tool in zip ssh scp; do
        if ! command -v "$tool" &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done
    
    if [[ ${#missing_tools[@]} -gt 0 ]]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        exit 1
    fi
    
    if [[ ! -f "${SSH_KEY}" ]]; then
        log_error "SSH key not found: ${SSH_KEY}"
        exit 1
    fi
    
    log_info "All prerequisites met"
}

# Create archive excluding build artifacts and git files
create_archive() {
    log_info "Creating archive: ${ARCHIVE_NAME}"
    
    zip -r "${ARCHIVE_NAME}" . \
        -x "*.git*" \
        -x "*target/*" \
        -x "*Cargo.lock" \
        -x "*.DS_Store" \
        -x "*deploy_test.sh" \
        -x "*.zip" \
        > /dev/null
    
    if [[ ! -f "${ARCHIVE_NAME}" ]]; then
        log_error "Failed to create archive"
        exit 1
    fi
    
    local size
    size=$(du -h "${ARCHIVE_NAME}" | cut -f1)
    log_info "Archive created successfully (${size})"
}

# Transfer archive to remote server
transfer_archive() {
    log_info "Transferring archive to ${REMOTE_USER}@${REMOTE_HOST}..."
    
    if ! scp -i "${SSH_KEY}" "${ARCHIVE_NAME}" "${REMOTE_USER}@${REMOTE_HOST}:/tmp/${ARCHIVE_NAME}"; then
        log_error "Failed to transfer archive"
        exit 1
    fi
    
    log_info "Transfer completed"
}

# Deploy on remote server
deploy_on_remote() {
    log_info "Deploying on remote server..."
    
    # shellcheck disable=SC2087
    ssh -i "${SSH_KEY}" "${REMOTE_USER}@${REMOTE_HOST}" "REMOTE_PATH='${REMOTE_PATH}' ARCHIVE_NAME='${ARCHIVE_NAME}'" bash << 'EOF'
        set -eo pipefail
        
        # Source environment files to load cargo/rustup
        # Temporarily disable unbound variable check for sourcing rc files
        if [[ -f "$HOME/.cargo/env" ]]; then
            source "$HOME/.cargo/env"
        fi
        
        if [[ -f "$HOME/.bashrc" ]]; then
            set +u
            source "$HOME/.bashrc" 2>/dev/null || true
            set -u
        fi
        
        if [[ -f "$HOME/.profile" ]]; then
            set +u
            source "$HOME/.profile" 2>/dev/null || true
            set -u
        fi
        
        # Re-enable strict mode for our commands
        set -u
        
        echo "[Remote] Preserving build artifacts..."
        if [[ -d "${REMOTE_PATH}" ]]; then
            # Backup Cargo.lock and target directory if they exist
            [[ -f "${REMOTE_PATH}/Cargo.lock" ]] && cp "${REMOTE_PATH}/Cargo.lock" "/tmp/Cargo.lock.backup" || true
            [[ -d "${REMOTE_PATH}/target" ]] && mv "${REMOTE_PATH}/target" "/tmp/target.backup" || true
        fi
        
        echo "[Remote] Removing old version..."
        rm -rf "${REMOTE_PATH}"
        
        echo "[Remote] Creating directory..."
        mkdir -p "${REMOTE_PATH}"
        
        echo "[Remote] Extracting archive..."
        unzip -q "/tmp/${ARCHIVE_NAME}" -d "${REMOTE_PATH}"
        
        echo "[Remote] Restoring build artifacts..."
        # Restore Cargo.lock and target directory
        [[ -f "/tmp/Cargo.lock.backup" ]] && mv "/tmp/Cargo.lock.backup" "${REMOTE_PATH}/Cargo.lock" || true
        [[ -d "/tmp/target.backup" ]] && mv "/tmp/target.backup" "${REMOTE_PATH}/target" || true
        
        echo "[Remote] Cleaning up archive..."
        rm -f "/tmp/${ARCHIVE_NAME}"
        
        echo "[Remote] Building project..."
        cd "${REMOTE_PATH}"
        
        if command -v cargo &> /dev/null; then
            cargo build --release
            echo "[Remote] Build completed successfully"
        else
            echo "[Remote] WARNING: cargo not found, skipping build"
            echo "[Remote] PATH: $PATH"
            exit 1
        fi
EOF
    
    if [[ $? -eq 0 ]]; then
        log_info "Deployment completed successfully"
    else
        log_error "Deployment failed"
        exit 1
    fi
}

# Main execution
main() {
    log_info "Starting deployment of ${PROJECT_NAME}..."
    
    check_prerequisites
    create_archive
    transfer_archive
    deploy_on_remote
    
    log_info "Deployment finished successfully!"
    log_info "Remote path: ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}"
}

# Run main function
main "$@"
