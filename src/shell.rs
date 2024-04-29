use std::collections::HashMap;
use std::fs;
use std::io;

use crate::{count_newlines, PRIVATE_KEY_REGEX};

fn search(delete: bool) -> io::Result<()> {
    let shells = read_shells("/etc/shells")?;
    let shell_history_paths: HashMap<&str, &str> = HashMap::from([
        ("/bin/bash", ".bash_history"),
        ("/bin/zsh", ".zsh_history"),
        ("/bin/tcsh", ".history"),
        ("/usr/bin/fish", ".config/fish/fish_history"),
    ]);

    if let Some(home_path) = dirs::home_dir() {
        for shell in shells {
            if let Some(history_file) = shell_history_paths.get(shell.as_str()) {
                let history_path = home_path.join(history_file);
                if history_path.exists() {
                    let content = fs::read_to_string(&history_path)?;
                    if delete {
                        if PRIVATE_KEY_REGEX.is_match(&content) {
                            println!(
                                "Private keys found in history file {}, removing them.",
                                &history_path.as_path().display()
                            );
                            /* Clean the history files */
                            let cleaned = PRIVATE_KEY_REGEX.replace_all(&content, "");
                            fs::write(history_path, cleaned.as_ref())
                                .expect("Failed to write to history file");
                        }
                    } else {
                        /* Just warn and print the lines */
                        let matches = PRIVATE_KEY_REGEX.find_iter(&content);
                        let lines = matches.map(|key| count_newlines(&content[..key.start()]) + 1);
                        for line in lines {
                            println!(
                                "Private key found on line {} in file {}",
                                line,
                                &history_path.as_path().display()
                            );
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to find the home directory for the current user.");
    }

    Ok(())
}

fn read_shells(path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let lines = content.lines();
    let mut shells = Vec::new();

    for line in lines {
        if !line.starts_with('#') && !line.trim().is_empty() {
            shells.push(line.trim().to_string());
        }
    }

    Ok(shells)
}
