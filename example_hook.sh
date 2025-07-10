#!/bin/bash

# Claude Code Hook Script Example
# This script receives JSON input from Claude Code hooks and forwards it to ccn

# Build the ccn binary first if it doesn't exist
SCRIPT_DIR="$(dirname "$0")"
CCN_BINARY="$SCRIPT_DIR/target/release/ccn"

if [ ! -f "$CCN_BINARY" ]; then
    echo "Building ccn binary..." >&2
    cd "$SCRIPT_DIR"
    cargo build --release
fi

# Read JSON input from stdin and pipe it to ccn
# You can specify editor type with -t flag (vscode or cursor)
# Default is vscode

# For VS Code (default):
cat | "$CCN_BINARY"

# For Cursor, uncomment the line below and comment the line above:
# cat | "$CCN_BINARY" -t cursor

# Example of the JSON input format that Claude Code provides:
# {
#   "session_id": "unique-session-id",
#   "transcript_path": "/path/to/transcript",
#   "hook_event_name": "Stop",
#   "message": "Optional message content"
# }