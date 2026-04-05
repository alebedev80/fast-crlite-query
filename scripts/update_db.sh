#!/bin/bash

# scripts/update_db.sh - Update CRLite filters using rust-query-crlite

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <path_to_rust_query_crlite> <db_dir>"
    exit 1
fi

UPDATE_BIN="$1"
DB_DIR="$2"

if [ ! -f "$UPDATE_BIN" ]; then
    echo "Error: Binary not found at $UPDATE_BIN"
    exit 1
fi

mkdir -p "$DB_DIR"

echo "[$(date)] Updating CRLite database in $DB_DIR using $UPDATE_BIN..."

# Execute update using the provided binary
"$UPDATE_BIN" --db "$DB_DIR" --update prod help > /dev/null 2>&1

if [ $? -eq 0 ]; then
    echo "[$(date)] CRLite DB updated successfully in $DB_DIR"
    # Cleanup old delta files (older than 30 days)
    find "$DB_DIR" -name "*.delta" -mtime +30 -delete
else
    echo "[$(date)] CRLite DB update failed!"
    exit 1
fi
