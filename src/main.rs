// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use argh::FromArgs;
use walkdir::WalkDir;

#[derive(FromArgs)]
/// Cargo Header
struct Args {
    /// path to the header file, default to: ./header
    #[argh(option, default = "PathBuf::from(\"./header\")")]
    file: PathBuf,

    /// directory to apply the header, default to current dir: .
    #[argh(option, default = "PathBuf::from(\".\")")]
    dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let Args { file, dir } = argh::from_env();
    let header = get_header_content(&file)?;
    insert_header(&dir, &header)?;
    Ok(())
}

fn get_header_content(header_path: &Path) -> anyhow::Result<String> {
    let mut header_file = File::open(header_path)
        .context("Header file not found. Use the --file option or create a ./header file.")?;

    let mut contents = String::new();
    header_file.read_to_string(&mut contents)?;

    // Add '//' to the beginning of each line
    let mut header_comment = contents
        .lines()
        .map(|line| format!("// {line}\n"))
        .collect();

    // Add a whole new line
    header_comment += "\n";

    Ok(header_comment)
}

fn insert_header(dir: &Path, header: &str) -> anyhow::Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let file_path = entry.path();

        // Skip if the file doesn't have a extension
        let Some(extension) = file_path.extension() else {
            continue;
        };

        // Skip if the file is not a rust file
        if extension != "rs" {
            continue;
        }

        // Read the existing content of the file
        let mut file = File::open(file_path)?;
        let mut existing_content = String::new();
        file.read_to_string(&mut existing_content)?;

        // Skip if the content of the header already exist
        if existing_content.contains(header.trim()) {
            continue;
        }

        // Create a new file with the same path
        let mut new_file = File::create(file_path)?;

        // Write the header followed by the existing content
        new_file.write_all(header.as_bytes())?;
        new_file.write_all(existing_content.as_bytes())?
    }
    Ok(())
}
