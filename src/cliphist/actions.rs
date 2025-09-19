use super::types::{ClipboardEntry, CliphistError};
use std::io::Write;
use std::process::Command;

pub fn get_history() -> Result<Vec<ClipboardEntry>, CliphistError> {
    let output = Command::new("cliphist")
        .arg("list")
        .output()
        .map_err(|e| CliphistError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(CliphistError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut options = Vec::new();

    for line in stdout.lines() {
        // Each line is expected to be in the format: "index: content"
        if let Some((index_str, content)) = line.split_once("\t") {
            if let Ok(index) = index_str.parse::<u64>() {
                options.push(ClipboardEntry {
                    index,
                    content: content.to_string(),
                });
            } else {
                return Err(CliphistError::ParseError(format!(
                    "Invalid index: {}",
                    index_str
                )));
            }
        } else {
            return Err(CliphistError::ParseError(format!(
                "Invalid line format: {}",
                line
            )));
        }
    }

    Ok(options)
}

pub fn copy_history(id: u64) -> Result<(), CliphistError> {
    println!("Copying history entry with ID: {}", id);

    let make_error_message = |msg| {
        std::fmt::format(format_args!(
            "Failed to decode history entry {}: {}",
            id, msg
        ))
    };

    let output = Command::new("cliphist")
        .arg("decode")
        .arg(id.to_string())
        .output()
        .map_err(|e| CliphistError::CommandFailed(make_error_message(e.to_string())))?;

    let copy_output = Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(&output.stdout)?;
            }
            child.wait()
        })
        .map_err(|e| CliphistError::CommandFailed(e.to_string()))?;

    if !copy_output.success() {
        return Err(CliphistError::CommandFailed(
            "Failed to copy content to clipboard using wl-copy".to_string(),
        ));
    }

    Ok(())
}

pub fn wipe_history() -> Result<(), CliphistError> {
    let output = Command::new("cliphist")
        .arg("wipe")
        .output()
        .map_err(|e| CliphistError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(CliphistError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}
