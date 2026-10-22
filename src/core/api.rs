//! Public interface of the core module

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use crate::core::backend::{
    search_for_repos,
    exec_async_check
};
use crate::utils::{
    APP_NAME,
    AppFile,
    repo_has_entry,
    repos_valid
};

use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::Path;

use colored::Colorize;

/// Scans only specified directories
pub fn scan_dirs(mut dirs: Vec<String>, tracking_file: &AppFile, scan_hidden: bool) -> Result<String, String> {
    // Remove duplicates
    dirs.sort_unstable();
    dirs.dedup();

    // Directories validation

    let mut dirs_ok = true;

    for dir in &mut dirs {
        let path = Path::new(&dir);

        // Check if the path exists
        if let Ok(p) = path.try_exists() {
            if !p {
                eprintln!("{APP_NAME}: Directory '{dir}' does not exist");
                dirs_ok = false;
                continue;
            }
        }
        else {
            eprintln!("{APP_NAME}: Cannot check the existance of directory '{dir}'");
            dirs_ok = false;
            continue;
        }

        // Check if the path leads to a file
        if path.is_file() {
            eprintln!("{APP_NAME}: '{dir}' is not a directory");
            dirs_ok = false;
        }

        // Check if the path contains valid UTF-8 characters
        // and make it absolute, if it does
        if let Some(s) = fs::canonicalize(&dir)
            .map_err(|e| format!("{dir}: {e}"))?
            .to_str() {
            *dir = s.to_string();
        }
        else {
            eprintln!("{APP_NAME}: {dir}: The path contains invalid UTF-8 characters");
            dirs_ok = false;
        }
    }

    if !dirs_ok {
        return Err(String::from("Directories validation failed"));
    }

    Ok(search_for_repos(dirs.as_slice(), tracking_file, scan_hidden)?)
}

/// Scans all directories in user's /home
pub fn scan_all(home_dir: String, tracking_file: &AppFile, scan_hidden: bool) -> Result<String, String> {
    Ok(search_for_repos(&[home_dir], tracking_file, scan_hidden)?)
}

/// Prints the contents of the app file to the standard output
pub fn list(app_file: &AppFile) -> Result<(), String> {
    if app_file.contents.is_empty() {
        return Err(format!("The file '{}' is empty or doesn't exist yet", app_file.path));
    }

    print!("{}", app_file.contents.bold());

    Ok(())
}

/// Writes the paths of the specified repos to the app file
pub fn add_entries(mut repos: Vec<String>, app_file: &AppFile) -> Result<(), String> {
    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    repos = repos_valid(repos.as_slice())?;

    // Open/create the app file for writing
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(app_file.path.clone())
        .map_err(|e| format!("{}: {e}", app_file.path))?;

    for repo in repos {
        // Check if the app file already
        // contains the git repository path
        if repo_has_entry(repo.as_str(), app_file.contents.as_str()) {
            println!("{APP_NAME}: '{repo}' already has an entry in the '{}' file", app_file.path);
            continue;
        }

        // Add the path of the git repository to the app file
        file.write_all(
            format!("{repo}\n").as_bytes())
            .map_err(|e| format!("{}: {e}", app_file.path))?;
    }

    Ok(())
}

/// Removes only specified repositories from the app file
pub fn remove_entries(mut repos: Vec<String>, app_file: &AppFile) -> Result<(), String> {
    if app_file.contents.is_empty() {
        return Err(format!("The file '{}' is empty or doesn't exist yet", app_file.path));
    }

    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    let mut repos_ok = true;

    // Repositories validation
    for repo in &repos {
        // Check if the app file contains the git repository
        if !repo_has_entry(repo.as_str(), app_file.contents.as_str()) {
            eprintln!("{APP_NAME}: '{repo}' has no entry in the '{}' file", app_file.path);
            repos_ok = false;
        }
    }

    if !repos_ok {
        return Err(String::from("Repositories validation failed"));
    }

    let mut file_lines: Vec<&str> = app_file.contents.lines().collect();

    // Open/create the app file for writing
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(app_file.path.clone())
        .map_err(|e| format!("{}: {e}", app_file.path))?;

    for repo in repos {
        // Remove specified repositories from the vector
        if let Some(last) = file_lines.last() {
            if repo.trim() == last.trim() {
                file_lines.pop();
            }
            else {
                file_lines.retain(|&x| x.trim() != repo.trim());
            }
        }
    }

    // Write the final changes to the app file
    file.write_all(file_lines.join("\n").as_bytes())
        .map_err(|e| format!("{}: {e}", app_file.path))?;

    Ok(())
}

/// Removes the tracking file
pub fn remove_all(tracking_file: &AppFile) -> Result<(), String> {
    if tracking_file.contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    fs::remove_file(tracking_file.path.clone()).map_err(|e| format!("{}: {e}", tracking_file.path))?;

    Ok(())
}

/// Asynchronously retrieves important details about each repo
/// in the repos Vec and prints them to the standard output
pub async fn check_repos(mut repos: Vec<String>, flags: &[bool]) -> Result<(), String> {
    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    repos = repos_valid(repos.as_slice())?;

    exec_async_check(repos, flags.to_vec()).await?;

    Ok(())
}

/// Asynchronously retrieves important details about each repo
/// in the tracking file and prints them to the standard output
pub async fn check_all(tracking_file: &AppFile, flags: &[bool]) -> Result<(), String> {
    if tracking_file.contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    // Put all the tracking file entries in a Vec to
    // avoid lifetime constraints on async tasks
    let track_file_lines: Vec<String> = tracking_file.contents
        .lines()
        .map(String::from)
        .collect();

    exec_async_check(track_file_lines, flags.to_vec()).await?;

    Ok(())
}
