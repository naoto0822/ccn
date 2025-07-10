# Claude Code Notifier (ccn)

Claude Code Notifier is a command-line tool that integrates with Claude Code hooks to send macOS notifications with editor jump functionality.

**Platform Support**: macOS only  
**Editor Support**: VS Code and Cursor only

## Features

- Send notifications via terminal-notifier when Claude Code hooks are triggered
- Click notifications to jump to the current project directory in VS Code or Cursor
- Support for multiple editors (VS Code and Cursor)
- Process JSON input from Claude Code hooks with session information

## Requirements

- macOS
- terminal-notifier
- VS Code or Cursor editor (for jump functionality)
- Claude Code

## Installation

1. Install terminal-notifier:
```bash
brew install terminal-notifier
```

2. Configure macOS notification settings:
  - Open System Preferences → Notifications
  - Find and select terminal-notifier
  - Check "Allow Notifications"

3. Install ccn:
```bash
cargo install ccn
```

## Debug

The tool is designed to be used as a Claude Code hook that receives JSON input via stdin:

```bash
echo '{"session_id":"123","transcript_path":"/path","hook_event_name":"Stop"}' | ccn
```

### Command Line Options

- `-t, --editor-type <EDITOR_TYPE>`: Specify editor type (vscode or cursor). Default: vscode

```bash
# Use with Cursor
echo '{"session_id":"123","transcript_path":"/path","hook_event_name":"Notification","message":"Hello"}' | ccn -t cursor

# Use with VS Code (default)
echo '{"session_id":"123","transcript_path":"/path","hook_event_name":"Stop"}' | ccn
```

### Hook Events

The tool handles different hook event types:
- `"Notification"`: Shows the provided message or event name
- `"Stop"`: Shows "Completed"
- Other events: Shows the event name

## Claude Code Integration

Configure Claude Code hooks in your settings to use ccn:

```json
{
"hooks": {
  "Notification": [
    {
      "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "ccn -t cursor"
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "ccn -t cursor"
          }
        ]
      }
    ]
  }
}
```