// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    fmt::Write,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use clap::Parser;
use color_eyre::{
    eyre::{Context, Result},
    Section,
};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// path to the header file
    #[arg(long, default_value = "./NOTICE")]
    header: PathBuf,

    /// directory to apply the header
    #[arg(long, default_value = ".")]
    dir: PathBuf,

    /// select files by extension, e.g. rs,js,kt
    #[arg(short, long, default_values_t = ["rs".to_string()], value_delimiter = ',')]
    extensions: Vec<String>,

    /// comment style
    #[arg(short, long, default_value = "//")]
    comment_style: String,

    /// remove header, run first if you need to update your header
    #[arg(short, long)]
    remove: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let Args {
        header,
        dir,
        extensions,
        comment_style,
        remove,
    } = Args::parse();
    let header = get_header_content(&header, &comment_style)?;
    insert_header(&dir, &header, extensions, remove)?;
    Ok(())
}

fn get_header_content(header_path: &Path, comment_style: &str) -> Result<String> {
    let mut header_file = File::open(header_path)
        .wrap_err("Header file not found")
        .suggestion("Use the --header option or create a ./NOTICE file.")?;

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

fn insert_header(dir: &Path, header: &str, extensions: Vec<String>, remove: bool) -> Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let file_path = entry.path();

        // Skip if the file doesn't have a extension
        let Some(extension) = file_path.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        if extensions.iter().all(|e| e != extension) {
            continue;
        }

        // Read the existing content of the file
        let mut file = File::open(file_path)?;
        let mut existing_content = Vec::new();
        file.read_to_end(&mut existing_content)?;

        // Convert header to bytes
        let header_bytes = header.trim().as_bytes();

        // Skip if the content of the header already exists
        if !remove && existing_content.starts_with(header_bytes) {
            continue;
        }

        // Skip if the content of the header doesn't exists
        if remove && !existing_content.starts_with(header_bytes) {
            continue;
        }

        // Create a new file with the same path
        let mut new_file = File::create(file_path)?;

        // remove or add header
        if remove {
            existing_content.drain(0..=header_bytes.len() + '\n'.len_utf8());
        } else {
            std::io::Write::write_all(&mut new_file, header.as_bytes())?;
        }

        // Write existing content
        std::io::Write::write_all(&mut new_file, &existing_content)?;
    }
    Ok(())
}
