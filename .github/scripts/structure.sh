#!/bin/bash

# Set the directory to be logged relative to the script location and the output file
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$SCRIPT_DIR/../../"
OUTPUT_FILE="$SCRIPT_DIR/../../workspace_log.md"

# Function to log directory structure
log_directory() {
    local DIR_PATH=$1
    local INDENT=$2

    # List all files and directories including hidden ones, excluding special entries and excluded files
    local ITEMS=()
    for ITEM in "$DIR_PATH"/* "$DIR_PATH"/.*; do
        local ITEM_NAME=$(basename "$ITEM")
        if [[ "$ITEM_NAME" != "." && "$ITEM_NAME" != ".." && "$ITEM_NAME" != "target" && "$ITEM_NAME" != ".git" && "$ITEM_NAME" != ".env" ]]; then
            ITEMS+=("$ITEM")
        fi
    done
    local ITEM_COUNT=${#ITEMS[@]}
    local COUNTER=0

    for ITEM in "${ITEMS[@]}"; do
        COUNTER=$((COUNTER + 1))

        # Determine the tree structure symbols
        if [ "$COUNTER" -eq "$ITEM_COUNT" ]; then
            TREE_SYMBOL="└──"
            NEXT_INDENT="${INDENT}    "
        else
            TREE_SYMBOL="├──"
            NEXT_INDENT="${INDENT}│   "
        fi

        # Get the base name of the item
        local ITEM_NAME=$(basename "$ITEM")

        if [ -d "$ITEM" ]; then
            # If item is a directory, log it and call the function recursively
            echo "${INDENT}${TREE_SYMBOL} ${ITEM_NAME}/" >> "$OUTPUT_FILE"
            log_directory "$ITEM" "$NEXT_INDENT"
        else
            # If item is a file, log it
            echo "${INDENT}${TREE_SYMBOL} ${ITEM_NAME}" >> "$OUTPUT_FILE"
        fi
    done
}

# Remove the output file if it exists
rm -f "$OUTPUT_FILE"

# Start logging from the workspace directory
echo "$(basename "$WORKSPACE_DIR")/" > "$OUTPUT_FILE"
log_directory "$WORKSPACE_DIR" ""

echo "Workspace structure logged to $OUTPUT_FILE"
