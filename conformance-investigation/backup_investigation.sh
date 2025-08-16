#!/bin/bash

# Backup script for conformance investigation folder
# Creates versioned backups to prevent loss of work during investigation

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
INVESTIGATION_DIR="$(dirname "$0")"
BACKUP_DIR="${INVESTIGATION_DIR}/../investigation-backups"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
COMMIT_HASH=$(git rev-parse --short HEAD 2>/dev/null || echo "no-git")

# Function to create backup
create_backup() {
    local backup_type="$1"
    local description="$2"
    
    # Create backup directory if it doesn't exist
    mkdir -p "$BACKUP_DIR"
    
    # Determine backup name
    local backup_name="investigation_${backup_type}_${TIMESTAMP}_${COMMIT_HASH}"
    local backup_path="$BACKUP_DIR/$backup_name"
    
    echo -e "${YELLOW}Creating backup: $backup_name${NC}"
    
    # Copy investigation folder
    cp -r "$INVESTIGATION_DIR" "$backup_path"
    
    # Create metadata file
    cat > "$backup_path/BACKUP_INFO.txt" << EOF
Backup Information
==================
Type: $backup_type
Date: $(date)
Commit: $COMMIT_HASH
Description: $description
Source: $INVESTIGATION_DIR

Conformance Status at Backup:
$(cd .. && cargo run --bin oxc_prettier_conformance 2>&1 | grep "compatibility:" || echo "Could not determine")

Files in backup:
$(ls -la "$backup_path" | tail -n +2)
EOF
    
    echo -e "${GREEN}✓ Backup created: $backup_path${NC}"
    
    # Create symlink to latest backup
    ln -sf "$backup_name" "$BACKUP_DIR/latest"
    
    return 0
}

# Function to list backups
list_backups() {
    echo -e "${YELLOW}Available backups:${NC}"
    if [ -d "$BACKUP_DIR" ]; then
        ls -lt "$BACKUP_DIR" | grep -v "^total" | grep -v "latest" | head -20
        
        if [ -L "$BACKUP_DIR/latest" ]; then
            echo ""
            echo -e "${GREEN}Latest backup:${NC}"
            readlink "$BACKUP_DIR/latest"
        fi
    else
        echo "No backups found"
    fi
}

# Function to restore backup
restore_backup() {
    local backup_name="$1"
    
    if [ -z "$backup_name" ]; then
        echo -e "${RED}Error: Please specify backup name${NC}"
        list_backups
        return 1
    fi
    
    local backup_path="$BACKUP_DIR/$backup_name"
    
    if [ ! -d "$backup_path" ]; then
        echo -e "${RED}Error: Backup not found: $backup_path${NC}"
        list_backups
        return 1
    fi
    
    echo -e "${YELLOW}Restoring from: $backup_name${NC}"
    
    # Create backup of current state before restoring
    create_backup "before-restore" "Automatic backup before restore"
    
    # Remove current investigation folder contents (except this script and backups)
    find "$INVESTIGATION_DIR" -mindepth 1 -maxdepth 1 \
        -not -name "backup_investigation.sh" \
        -not -name "investigation-backups" \
        -exec rm -rf {} \;
    
    # Copy files from backup
    cp -r "$backup_path"/* "$INVESTIGATION_DIR/"
    
    # Remove the backup info file from restored directory
    rm -f "$INVESTIGATION_DIR/BACKUP_INFO.txt"
    
    echo -e "${GREEN}✓ Restored from backup: $backup_name${NC}"
    echo ""
    echo "Restored files:"
    ls -la "$INVESTIGATION_DIR"
}

# Function to clean old backups
clean_backups() {
    local keep_count="${1:-10}"
    
    if [ ! -d "$BACKUP_DIR" ]; then
        echo "No backups to clean"
        return 0
    fi
    
    local total_backups=$(find "$BACKUP_DIR" -maxdepth 1 -type d -name "investigation_*" | wc -l)
    
    if [ "$total_backups" -le "$keep_count" ]; then
        echo "Only $total_backups backups exist, keeping all (threshold: $keep_count)"
        return 0
    fi
    
    echo -e "${YELLOW}Cleaning old backups (keeping latest $keep_count)${NC}"
    
    # Find and remove old backups
    find "$BACKUP_DIR" -maxdepth 1 -type d -name "investigation_*" -print0 | \
        xargs -0 ls -dt | \
        tail -n +$((keep_count + 1)) | \
        while read -r old_backup; do
            echo "Removing: $(basename "$old_backup")"
            rm -rf "$old_backup"
        done
    
    echo -e "${GREEN}✓ Cleanup complete${NC}"
}

# Function to show backup diff
show_diff() {
    local backup_name="$1"
    
    if [ -z "$backup_name" ]; then
        backup_name=$(readlink "$BACKUP_DIR/latest" 2>/dev/null)
        if [ -z "$backup_name" ]; then
            echo -e "${RED}Error: No backup specified and no latest backup found${NC}"
            return 1
        fi
    fi
    
    local backup_path="$BACKUP_DIR/$backup_name"
    
    if [ ! -d "$backup_path" ]; then
        echo -e "${RED}Error: Backup not found: $backup_path${NC}"
        return 1
    fi
    
    echo -e "${YELLOW}Comparing current state with: $backup_name${NC}"
    echo ""
    
    # Compare directories
    diff -r --brief "$backup_path" "$INVESTIGATION_DIR" 2>/dev/null | \
        grep -v "BACKUP_INFO.txt" | \
        grep -v "backup_investigation.sh" || \
        echo -e "${GREEN}No differences found${NC}"
}

# Main script logic
case "${1:-help}" in
    create)
        create_backup "${2:-manual}" "${3:-Manual backup}"
        ;;
    list)
        list_backups
        ;;
    restore)
        restore_backup "$2"
        ;;
    clean)
        clean_backups "${2:-10}"
        ;;
    diff)
        show_diff "$2"
        ;;
    auto)
        # Automatic backup before major changes
        create_backup "auto" "Automatic backup before changes"
        ;;
    help|--help|-h)
        cat << EOF
Conformance Investigation Backup Manager

Usage: ./backup_investigation.sh [command] [options]

Commands:
    create [type] [description]  Create a new backup
                                 type: manual|auto|checkpoint|before-fix
                                 Example: ./backup_investigation.sh create checkpoint "Before ternary fix"
    
    list                        List all available backups
    
    restore [backup_name]       Restore from a specific backup
                               Example: ./backup_investigation.sh restore investigation_checkpoint_20250816_143022_abc123
    
    clean [keep_count]          Remove old backups, keeping the latest N (default: 10)
                               Example: ./backup_investigation.sh clean 5
    
    diff [backup_name]          Show differences between current and backup
                               If no name given, compares with latest
    
    auto                        Create automatic backup (used by scripts)
    
    help                        Show this help message

Examples:
    # Before making major changes
    ./backup_investigation.sh create checkpoint "Before refactoring docs"
    
    # View available backups
    ./backup_investigation.sh list
    
    # Restore if something goes wrong
    ./backup_investigation.sh restore investigation_checkpoint_20250816_143022_abc123
    
    # Compare current state with latest backup
    ./backup_investigation.sh diff

Backup Location: $BACKUP_DIR
EOF
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        echo "Use './backup_investigation.sh help' for usage information"
        exit 1
        ;;
esac