// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    fmt::Write,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Context;
use argh::FromArgs;
use walkdir::WalkDir;

#[derive(FromArgs)]
/// A cli to add headers to files
struct Args {
    /// path to the header file, default to: ./NOTICE
    #[argh(option, default = "PathBuf::from(\"./NOTICE\")")]
    header: PathBuf,

    /// directory to apply the header, default to current dir: .
    #[argh(option, default = "PathBuf::from(\".\")")]
    dir: PathBuf,

    /// select files by extension, default to "rs", format: comma-separated, e.g. "rs,js,kt"
    #[argh(option, default = "String::from(\"rs\")")]
    extensions: String,

    /// comment style, default to "//"
    #[argh(option, default = "String::from(\"//\")")]
    comment_style: String,
}

fn main() -> anyhow::Result<()> {
    let Args {
        header: file,
        dir,
        extensions,
        comment_style,
    } = argh::from_env();
    let header = get_header_content(&file, &comment_style)?;
    insert_header(&dir, &header, &extensions)?;
    Ok(())
}

fn get_header_content(header_path: &Path, comment_style: &str) -> anyhow::Result<String> {
    let mut header_file = File::open(header_path)
        .context("Header file not found. Use the --file option or create a ./NOTICE file.")?;

    let mut contents = String::new();
    header_file.read_to_string(&mut contents)?;

    let mut header_comment = String::new();
    for line in contents.lines() {
        writeln!(&mut header_comment, "{comment_style} {line}")?;
    }

    // Add a whole new line
    header_comment += "\n";

    Ok(header_comment)
}

fn insert_header(dir: &Path, header: &str, extensions: &str) -> anyhow::Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let file_path = entry.path();

        // Skip if the file doesn't have a extension
        let Some(extension) = file_path.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        if extensions.split(',').all(|e| e != extension) {
            continue;
        }

        // Read the existing content of the file
        let mut file = File::open(file_path)?;
        let mut existing_content = Vec::new();
        file.read_to_end(&mut existing_content)?;

        // Convert header to bytes
        let header_bytes = header.trim().as_bytes();

        // Skip if the content of the header already exist
        if existing_content
            .windows(header_bytes.len())
            .any(|window| window == header_bytes)
        {
            continue;
        }

        // Create a new file with the same path
        let mut new_file = File::create(file_path)?;

        // Write the header followed by the existing content
        std::io::Write::write_all(&mut new_file, header.as_bytes())?;
        std::io::Write::write_all(&mut new_file, &existing_content)?;
    }
    Ok(())
}
