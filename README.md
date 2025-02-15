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

```
cargo install veha
```

#### Method 3: binary (Linux)

### Usage

In order to use this programm at its current development stage, you need to provide the required input arguments and the XML-files you provide need to follow the norms and standards devised as part of the VEHA project.

#### Required input arguments

- **input (_XHTML file_):** the path to the file which will be transformed via the programm.
- **template (_directory_):** the path to the directory which all the additional files and resources are to be accessed from via relative path.
- **output (_XHTML file_):** the path to the file which the end-result of the transformation will be written to [the content will be overwritten, if it already exists.]

#### VEHA norms and standards

##### XHTML Template file (Required)

in the input file, you need to wrap all you wish to be included in your output file within a `<vorlage>` tag:

```
<vorlage src="./relative-path-to-template.xhtml">
</vorlage>
```

and within your template directory, you need to include an empty `<slot>` tag which will be filled with the content between the pervious tags.
Below is an example of a typical template file for a webpage:

```
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN"
"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
	<head>
		<title></title>
	</head>
	<body>
		<slot></slot>
	</body>
</html>
```

##### Markdown (Optional)

In order to have your markdown file automatically parsed into HTML and wrapped within a specified tag and included in the output file, you can use the following XML-format:

```
<md src="./relative-path-to-markdown-file.md" tag="div" class="md text whatever-you-like"></md>
```

as you can see above, aside from `src` and `tag` attributes of the `<md>` tag, any other attribute will be carried out to the final tag.
In the above example the final tag will look something like this:

```
<div class="md text whatever-you-like">...</div>
```

##### CSV (Optional)

If you are unfamiliar with CSV files, the following might appear a bit of a strange behavior, and we are option to suggestion for change, but at the moment here is how the norm for our CSV-tag behaves:

```
<csv src="test">
	<...>
	<row src="row.xhtml" tag="div" class="wonderful">
	</row>
	<...>
</csv>
```

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
