# VEHA

**VEHA** is a CLI programm [written in Rust] to automate merging modular written data into a component based XHTML web-page.

Supported data formats: CSV, MD, and XHTML [as a wrapper template]

Future support planned: SQLite, LaTex, etc [to suggest a new input format open an issue or contact the author via email please].

## Quickstart guide

### Download and installation

#### Method 1: from source repository

First clone the repository, then install the third-party packages via carge and finally run code-base via cargo

```
git clone https://git.schloosser.net/HAD/had-rs.git

cargo install

cargo run -- <input_file> <template_diretory> <output_file>
```

#### Method 2: from Cargo.io

#### Method 3: binary (Linux)

### Usage



## Description

The German name for our code-base can be translated to: "Hypertext Markup Language Printer".

CLI-program which takes XHTML documents and populates it with:

- CSV (comma separated values): think of it as excel files or tables but much more programmer friendlier and universally transferable
- Markdown: think of it as a word document but without all the fuss
- HTML: at the moment only simple nesting of an XHTML file as a template is supported

At the moment this project is at the pilot test stage.

I'm looking forward to find collaborators who are eager and passionate to bring this idea into a full-fledged web framework ...

## Quickstart

see the `beispiel` directory for examples.

## License

Copyright (C) 2025 Hossein Rezaei (penname: Isaak Engineer)

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
