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

Notice: at the present level of the maturity of our codebase, please do not try closing XML (or HTML) tags within the same line; for example `<tag />` might have strange and unexpected behaviors.

##### XHTML Template file (Required)

in the input file, you need to wrap all you wish to be included in your output file within a `<vorlage>` tag:

```
<vorlage src="./relative-path-to-template.xhtml">
<...>
</vorlage>
```

and within your template directory, you need to include an empty `<slot>` tag which will be replaced with the content between the pervious tags.
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

###### Entrance tag

```
<csv src="test">
	<...>
	<row src="row.xhtml" tag="div" class="wonderful">
	</row>
	<...>
</csv>
```

###### Required row XML-template component file

As you might have guessed by now, within this component file, you can use a predefined XML-tag `<column>` which accepts the attribute `tag`;
Additionally, you have supply the `content` attribute for the programm to determine which column you are referring;
optionally, you could supply an `attribute` attribute, which will tell the program to insert the content instead of the text content of the specified _tag_, as an _attribute_ to the specified tag.

A typical example will look like this:

```
<column tag="div" class="header" content="title"></column>
```

A typical usecase for this optional attribute is when you would like the content of a particular column to be used as the `href` attribute for a link:

```
<column tag="a" class="title" content="link" attribute="href"></column>
```

##### Example

Putting it all together, you can look at the below CSV example and the corresponding `row.xhtml` component:

```
acronym,description,link
CSV,comma seperated value,https://b-greve.gitbook.io/beginners-guide-to-clean-data/common-csv-problems/untitled
MD,markdown,https://hackernoon.com/a-beginners-guide-to-markdown-everything-you-need-to-know-to-get-started
XHTML,extensible hypertext markup langauge,https://www.nayuki.io/page/practical-guide-to-xhtml
```

```
<div class="row">
	<column tag="a" class="title" content="link" attribute="href">
		<column tag="div" class="short" content="acronym">
		</column>
		<column tag="div" class="long" content="description">
		</column>
	</column>
	<footer>The link provided here is the result of a preliminary on web search engines; we take no responsibility for their content!</footer>
</div>
```

Notice: as you can see in this example, it is possible to use other HTML [and of course XHTML] tags within this component file;
They will be repeated for each row of the CSV file.

## Examples

### Quick-and-Dirty

For a quick example of what this tool is capable of, please look in the [beispiel](./beispiel/) directory of this repository.

### Production-grade examples

-

## About

The German name for our code-base can be translated to: "Hypertext Markup Language Printer".

CLI-program which takes XHTML documents and populates it with:

- CSV (comma separated values): think of it as excel files or tables but much more programmer friendlier and universally transferable
- Markdown: think of it as a word document but without all the fuss
- HTML: at the moment only simple nesting of an XHTML file as a template is supported

At the moment this project is at the pilot test stage.

I'm looking forward to find collaborators who are eager and passionate to bring this idea into a full-fledged web framework ...

## Status

### Maturity

- current state: MVP

### Dedication

_to my soulamte_. **Valentine Gift 2025**.

### License

Copyright (C) 2025 Hossein Rezaei (penname: Isaak Engineer)

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
