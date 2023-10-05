# add-header

[![License](https://img.shields.io/badge/license-MPL2.0-blue.svg)](https://www.mozilla.org/en-US/MPL/2.0/)
[![Crates.io](https://img.shields.io/crates/v/add-header.svg)](https://crates.io/crates/add-header)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.65+-red)
[![CI](https://github.com/ameknite/add-header/workflows/CI/badge.svg)](https://github.com/ameknite/add-header/actions?workflow=CI)

A cli to add headers to files.

## Install and Usage

You can install add-header with: `cargo install add-header` and run it in your project directory with: `add-header`

```bash
Usage: add-header [--header <header>] [--dir <dir>] [--extensions <extensions>] [--comment-style <comment-style>]

A cli to add headers to files

Options:
  --header          path to the header file, default to: ./NOTICE
  --dir             directory to apply the header, default to current dir: .
  --extensions      select files by extension, default to "rs", format:
                    comma-separated, e.g. "rs,js,kt"
  --comment-style   comment style, default to "//"
  --help            display usage information
```

## Purpose

Makes the process of adding header notices to files easier.

Like those requested by licenses such as MPL2.0:

```txt
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
```
