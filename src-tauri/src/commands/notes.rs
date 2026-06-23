use std::path::PathBuf;
use tauri::command;

/// Default Obsidian vault path for Catalyst Church
const VAULT_PATH: &str =
    "/Users/michael/Documents/Optimization Work/Obsidian Vault";

/// Save a sermon transcript to the Obsidian vault as a Markdown note.
///
/// Creates `Sermons/YYYY-MM-DD Sermon Transcript.md` in the vault.
/// Returns the absolute path of the saved file.
#[command]
pub fn save_sermon_to_obsidian(
    transcript: String,
    detected_verses: Vec<String>,
    sermon_title: Option<String>,
) -> Result<String, String> {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let title = sermon_title.unwrap_or_else(|| "Sermon Transcript".to_string());

    let sermons_dir = PathBuf::from(VAULT_PATH).join("Sermons");
    std::fs::create_dir_all(&sermons_dir)
        .map_err(|e| format!("Failed to create Sermons folder: {e}"))?;

    let filename = format!("{date} - {title}.md");
    let note_path = sermons_dir.join(&filename);

    let verses_section = if detected_verses.is_empty() {
        String::new()
    } else {
        format!(
            "\n## Verses Referenced\n\n{}\n",
            detected_verses
                .iter()
                .map(|v| format!("- {v}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    let content = format!(
        "---\ndate: {date}\ntags:\n  - sermon\n  - transcript\ncreated_by: Catalyst Scripture\n---\n\n# {title}\n\n**Date:** {date}  \n**Source:** Live service auto-transcription\n{verses_section}\n## Transcript\n\n{transcript}\n"
    );

    std::fs::write(&note_path, content)
        .map_err(|e| format!("Failed to write transcript: {e}"))?;

    log::info!("Sermon transcript saved to {}", note_path.display());
    Ok(note_path.to_string_lossy().to_string())
}
