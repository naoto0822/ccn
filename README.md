# Claude Code Notifier (ccn)

Claude Code Notifier is a command-line tool that integrates with Claude Code hooks to send macOS notifications with editor jump functionality.

## Features

- Send notifications via terminal-notifier when Claude Code hooks are triggered
- Click notifications to jump to the current project directory in VS Code or Cursor
- Support for multiple editors (VS Code and Cursor)
- Process JSON input from Claude Code hooks with session information

## Installation

1. Install terminal-notifier:
```bash
brew install terminal-notifier
```

2. Build the ccn binary:
```bash
cargo build --release
```

## Usage

The tool is designed to be used as a Claude Code hook that receives JSON input via stdin:

```bash
echo '{"session_id":"123","transcript_path":"/path","hook_event_name":"Stop","message":"Task completed"}' | ./target/release/ccn
```

### Command Line Options

- `-t, --editor-type <EDITOR_TYPE>`: Specify editor type (vscode or cursor). Default: vscode

```bash
# Use with Cursor
echo '{"session_id":"123","transcript_path":"/path","hook_event_name":"Notification","message":"Hello"}' | ./target/release/ccn -t cursor

# Use with VS Code (default)
echo '{"session_id":"123","transcript_path":"/path","hook_event_name":"Stop"}' | ./target/release/ccn
```

### Hook Events

The tool handles different hook event types:
- `"Notification"`: Shows the provided message or event name
- `"Stop"`: Shows "完了しました" (Completed in Japanese)
- Other events: Shows the event name

## Claude Code Integration

Configure Claude Code hooks in your settings to pipe JSON data to ccn:

```json
{
  "hooks": {
    "post_tool": "/path/to/ccn/example_hook.sh"
  }
}
```

The hook receives JSON input with:
- `session_id`: Current Claude Code session ID
- `transcript_path`: Path to conversation transcript
- `hook_event_name`: Type of event (e.g., "Stop", "Notification")
- `message`: Optional message content

## Requirements

- macOS
- terminal-notifier
- VS Code or Cursor editor (for jump functionality)
- Claude Code