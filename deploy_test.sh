#!/usr/bin/env bash

# Deploy Weaver Desktop to remote server for testing
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
readonly REMOTE_PATH="${REMOTE_PATH:-./Development/WeaverDesktop}"
readonly SSH_KEY="${SSH_KEY:-}"

# Build SSH options - only add identity file if explicitly specified
# Note: Using ${SSH_OPTS[@]+"${SSH_OPTS[@]}"} pattern to handle empty array with set -u
SSH_OPTS=()
if [[ -n "${SSH_KEY}" ]]; then
    SSH_OPTS+=(-i "${SSH_KEY}")
fi
readonly PROJECT_NAME="WeaverDesktop"
readonly ARCHIVE_PATTERN="${PROJECT_NAME}_*.tar.gz"
ARCHIVE_NAME=""

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

# Track if transfer succeeded for cleanup decision
TRANSFER_SUCCEEDED=false

# Cleanup function - only remove archive after successful transfer
cleanup() {
    if [[ "${TRANSFER_SUCCEEDED}" == "true" && -n "${ARCHIVE_NAME}" && -f "${ARCHIVE_NAME}" ]]; then
        log_info "Cleaning up local archive..."
        rm -f "${ARCHIVE_NAME}"
    elif [[ -n "${ARCHIVE_NAME}" && -f "${ARCHIVE_NAME}" ]]; then
        log_warn "Keeping archive for retry: ${ARCHIVE_NAME}"
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
    
    if [[ -n "${SSH_KEY}" && ! -f "${SSH_KEY}" ]]; then
        log_error "SSH key not found: ${SSH_KEY}"
        exit 1
    fi
    
    log_info "All prerequisites met"
}

# Find existing archive or create new one
create_or_reuse_archive() {
    # Check for existing archive
    local existing_archive
    existing_archive=$(ls -t ${ARCHIVE_PATTERN} 2>/dev/null | head -n1 || true)
    
    if [[ -n "${existing_archive}" && -f "${existing_archive}" ]]; then
        ARCHIVE_NAME="${existing_archive}"
        local size
        size=$(du -h "${ARCHIVE_NAME}" | cut -f1)
        log_info "Reusing existing archive: ${ARCHIVE_NAME} (${size})"
        return 0
    fi
    
    # Create new archive with timestamp
    ARCHIVE_NAME="${PROJECT_NAME}_$(date +%Y%m%d_%H%M%S).tar.gz"
    log_info "Creating archive: ${ARCHIVE_NAME}"
    
    tar --exclude='.git' \
        --exclude='target' \
        --exclude='Cargo.lock' \
        --exclude='.DS_Store' \
        --exclude='deploy_test.sh' \
        --exclude='*.tar.gz' \
        --exclude='*.zip' \
        -czf "${ARCHIVE_NAME}" .
    
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
    
    if ! scp ${SSH_OPTS[@]+"${SSH_OPTS[@]}"} "${ARCHIVE_NAME}" "${REMOTE_USER}@${REMOTE_HOST}:/tmp/${ARCHIVE_NAME}"; then
        log_error "Failed to transfer archive"
        exit 1
    fi
    
    log_info "Transfer completed"
    TRANSFER_SUCCEEDED=true
}

# Deploy on remote server
deploy_on_remote() {
    log_info "Deploying on remote server..."
    
    # shellcheck disable=SC2087
    ssh ${SSH_OPTS[@]+"${SSH_OPTS[@]}"} "${REMOTE_USER}@${REMOTE_HOST}" "REMOTE_PATH='${REMOTE_PATH}' ARCHIVE_NAME='${ARCHIVE_NAME}'" bash << 'EOF'
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
        
        echo "[Remote] Preserving Cargo.lock..."
        if [[ -d "${REMOTE_PATH}" && -f "${REMOTE_PATH}/Cargo.lock" ]]; then
            cp "${REMOTE_PATH}/Cargo.lock" "/tmp/Cargo.lock.backup"
        fi
        
        echo "[Remote] Removing old version..."
        rm -rf "${REMOTE_PATH}"
        
        echo "[Remote] Creating directory..."
        mkdir -p "${REMOTE_PATH}"
        
        echo "[Remote] Extracting archive..."
        tar -xzf "/tmp/${ARCHIVE_NAME}" -C "${REMOTE_PATH}"
        
        echo "[Remote] Restoring Cargo.lock..."
        [[ -f "/tmp/Cargo.lock.backup" ]] && mv "/tmp/Cargo.lock.backup" "${REMOTE_PATH}/Cargo.lock" || true
        
        echo "[Remote] Configuring external target directory..."
        mkdir -p "${REMOTE_PATH}/.cargo"
        mkdir -p "$(dirname "${REMOTE_PATH}")/weaverdesktop-target"
        cat > "${REMOTE_PATH}/.cargo/config.toml" << 'CARGO_CONFIG'
[build]
target-dir = "../weaverdesktop-target"
CARGO_CONFIG
        
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
    create_or_reuse_archive
    transfer_archive
    deploy_on_remote
    
    log_info "Deployment finished successfully!"
    log_info "Remote path: ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}"
}

# Run main function
main "$@"
