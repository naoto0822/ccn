use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use clap::{Parser, ValueEnum};

mod tnr_wrapper;
use tnr_wrapper::TnrWrapper;

#[derive(Deserialize, Serialize, Debug)]
struct HookInput {
    session_id: String,
    transcript_path: String,
    hook_event_name: String,
    message: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum EditorType {
    Vscode,
    Cursor,
}

impl EditorType {
    fn schema(&self) -> &'static str {
        match self {
            EditorType::Vscode => "vscode",
            EditorType::Cursor => "cursor",
        }
    }
}

#[derive(Parser)]
#[command(name = "ccn")]
#[command(about = "Claude Code Notifier - Send notifications with editor jump capability")]
struct Cli {
    #[arg(short = 't', long)]
    editor_type: Option<EditorType>,
}

fn main() {
    let cli = Cli::parse();
    
    if let Ok(current_dir) = std::env::current_dir() {
        println!("Current directory: {}", current_dir.display());
    } else {
        eprintln!("Failed to get current directory");
    }
    
    let mut input = String::new();
    
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read from stdin: {}", e);
        std::process::exit(1);
    }

    let hook_input: HookInput = match serde_json::from_str(&input) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            std::process::exit(1);
        }
    };

    let editor_type = cli.editor_type.as_ref().unwrap_or(&EditorType::Vscode);
    send_notification(&hook_input, editor_type);
}

fn send_notification(hook_input: &HookInput, editor_type: &EditorType) {
    let wrapper = TnrWrapper::new();
    
    let (title, current_dir_path) = if let Ok(current_dir) = std::env::current_dir() {
        let dir_name = current_dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown");
        let title = format!("Claude Code - {}", dir_name);
        let path = current_dir.to_string_lossy().to_string();
        (title, path)
    } else {
        ("Claude Code - Unknown".to_string(), "".to_string())
    };
    
    let message = match hook_input.hook_event_name.as_str() {
        "Notification" => hook_input.message.clone().unwrap_or_else(|| hook_input.hook_event_name.clone()),
        "Stop" => "完了しました".to_string(),
        _ => hook_input.hook_event_name.clone(),
    };
    
    let editor_url = Some(format!("{}://file/{}", editor_type.schema(), current_dir_path));

    match wrapper.notify(&title, &message, Some("Notification"), editor_url.as_deref()) {
        Ok(_) => println!("Notification sent successfully"),
        Err(e) => eprintln!("Failed to send notification: {}", e),
    }
}


