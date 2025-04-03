# veha (rs)

**veha-rs** is an open source utility that automates merging of written raw content and xml structures into web pages.

You can think of **veha-rs** as LaTex for content rich web pages, or as an incremental evolution to _PHP_, combined with _component-based_ approaches in _modern HTML_.

## Features

- **Easy** learn only a few extra [XML] tags, and you can create content-rich [static] websites
- **Fast** _Rust_ is one of the fastest languages [which you will notice the speed when rendering pages] and _static_ pages are one of the fastest ways to deliver content through web [which the viewers of your website will notice].
- **Multilingual support**
  - **Why |** with the advent of A.I. and its current pricing points, what a shame to not feature your content in as many languages as you can afford to review the A.I.-powered translation?
  - **How |** simply translate your plain-text or plain-tabular data via A.I and just provide a language tag! As easy as that!
- **Zero vendor lock-in**  _veha-rs_ is designed with separation of content and structure in mind, therefore:
  - your content remains yours and in formats you're most familiar/comfortable with.
  - as the only added layer of structure is a few simple tags, anyone can easily write a program to merge the content into the structure, in any programming language of your choice.
  - any additional layer of meta-data, are stored in TOML files; unlike many conventional _flat-file CMS_ solutions out there, for example: you don't need to fill a custom-made form to have a menu appear on your website. All such informations are to be stored in separate TOML files.
- **Plain text and tabular data** unlike conventional _static site generators_ (SSG), veha-rs supports CSV files by default.
- **Modular as a design philosophy** we believe in standardization and modularity reducing cost, improving productivity and resuability, hence **veha** and **veha-rs** are infused by this principle:
  - **Components everywhere** once your content is written and stored modulary, you will notice plenty use-cases, if it were stored in a format that couldn't easily be manipulated by codes.
  - **Behind the scene** _veha-rs_ itself is a collection of packages by other Rust developers and tearms; most of which can be easily replaced by similar packages.
  - **veha** regarding the standard itself please read the documentation [once released].

## About

### Technical

**Veha** is a set of standards to extend existing HTML [and XHTML] standard to work with raw.
**veha-rs** is a pilot test of these standards in Rust to be used in _server side rendering_.

At its core, you can think of **veha-rs** as a production line which takes

- modular written and tabular data
- an XHTML structure
- XHTML template/wrapper

and merges the data into the structure and forms a new XHTML file as output.

Additionally, as server side rendering enginge, you can supply a map of structure and output paths, and have **veha-rs** produce an entire website.

#### Supported data formats

Supported data formats:

- TOML
- CSV (comma separated values)
- MD (markdown)
- XHTML [though, only as a wrapper template]

Future support planned: SQLite, LaTex, etc [to suggest a new input format open an issue or contact the author via email please].

### Non-technical

CLI-program which takes XHTML documents and populates it with:

- CSV (comma separated values): think of it as excel files or tables but much more programmer friendlier and universally transferable
- Markdown: think of it as a word document but without all the fuss
- HTML: at the moment only simple nesting of an XHTML file as a template is supported

### Misc.

The German name for our code-base can be translated to: "Melter for Extendible Hypertext Markup Language" (Verschmelzer erweiterbare Hypertext-Auszeichnungssprache).

## Quickstart guide

### Step 0. Download and installation

#### Method 1. via Cargo

If you have _Rust_ can _Cargo_ installed, you can simply execute the following command:

```
cargo install veha
```

#### Method 2. Build from source repository

First clone the repository, then install the third-party packages via carge and finally run code-base via cargo

```
git clone https://git.schloosser.net/veha/veha-rs.git

cargo install

cargo run -- <input_file> <template_diretory> <output_file>
```

### Step 2. Usage

After installing the program, you can execute its functionality via the following two subcommands:

Please, keep in mind that your input file need to include the tags introduced by _veha standard_. A quick overview of said tags is available in the next section.

#### Subcommands

### `page`

The `page` subcommand processes a single XHTML page using the provided template and input file.

#### Usage

```
veha page <template> <input> <output> [--language <language_code>]
```

#### Arguments

- **template (_directory_):** the path to the directory which all the additional files and resources are to be accessed from via relative path.
- **input (_XHTML file_):** the path to the file which will be transformed via the program.
- **output (_XHTML file_):** the path to the file which the end-result of the transformation will be written to [the content will be overwritten, if it already exists.]
- **language (_optional_):** the language code for the content and webpage (e.g., `de`, `en`, `fr`).

### `site`

The `site` subcommand processes multiple XHTML pages using a mapping file that specifies input and output paths for each page.

#### Usage

```
veha site <template> <map> [--language <language_code>]
```

#### Arguments

- **template (_directory_):** the path to the directory which all the additional files and resources are to be accessed from via relative path.
- **map (_CSV file_):** the path to the CSV file which maps input files to output files.
- **language (_optional_):** the language code for the content and webpage (e.g., `de`, `en`, `fr`).

## Veha Standard

_Veha_ introduces a set of tags to empower including external raw data within a (X)HTML page.

The following observations can be made regarding these tags:

- the name of the tag is [often] the same as the most common extention for that particular data type. for example the `<md></md>` to include a _markdown_ file, which usually has the extension `.md`.
- most attributes are similar among tags and in-line with the existing HTML and XHTML conventions, whenever available. For example:
  - `src` for providing a relative path to the data.
  - `name` for providing the name of the tage which will be used to wrap the data within and replace the tag.
- for multilingual support, you have to provide the attribute `multilingual=""` to your data tag. [How it works read next segment].

**Warning (for veha-rs implementation)**: if you're coming from familiarity with HTML, please do not close tags within the same line; for example `<tag />` will have strange and unexpected behaviors.

### General attributes

#### `Multilingual`

When the attribute `multilingual` is set on a data tag, and **veha-rs** is provided with a `--lang fr`, for example, then the following files have to be present:

```
<md src="./post.md" multilingual=""></md>
```

This tag will be replaced with the content of `./post.fr.md` when input `--lang fr` was supplied to **veha-rs**.
otherwise if no input language is provided, the default file name will be read. In this case `./post.md`.

### Specific Tags

#### XHTML Template file (Required)

In the input file, you need to wrap all you wish to be included in your output file within a `<template>` tag:

```
<template src="./relative-path-to-template.xhtml">
<...>
</template>
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

##### Toml (Optional)

If you are familiar with [Tera](), you can use the `toml` tag to read and replace data from a Toml file;
Since it is possible to use the `tera` tag multiple times, you can optionally supply a `name` attribute:

```
<toml src="./relative-path/to/config.toml" name="optional"></toml>
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

For a quick example of what this tool is capable of, please look in the [beispiel](./beispiel/) directory of this repository:

- [Github](https://github.com/isaakengineer/veha-rs/tree/master/beispiel)
- [Intern](https://git.schloosser.net/veha/veha-rs/src/branch/master/beispiel)

### Production-grade examples

- Template for a resume website:
  - [template repository](https://git.schloosser.net/lebenslaeufe/webseite-vorlage)
  - [content repository](https://git.schloosser.net/lebenslaeufe/webseite-se.git)
  - [webseite](https://h.schloosser.org)

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit a pull request.

I'm looking forward to find collaborators who are eager and passionate to bring this idea into a full-fledged web framework ...

### Future steps

#### Short-term

Alternative motors:

- alternative to [tera]() [tinytemplate](https://docs.rs/tinytemplate/latest/tinytemplate/index.html)
- alternative to current markdown engine
- alternative to [quick-xml]

## Declarations

### Dedication

_to my soulamte_. **Valentine Gift 2025**.

### Status

#### Legal

- Company: [Techne (Gruppe)](https://techne.schloosser.com)
- Parent: [Schlösser (Gruppe)](https://schloosser.com)

#### Quality

- quality of source code: MVP-1
- quality of documentation: MD-starter
- stage of product growth: dangrous production-ready!

### Links

- [website](https://veha.techne.schloosser.com)
- main git repo: [Schlösser Frogejo](https://git.schloosser.net/veha/veha-rs)
- external git repo: [Github](https://github.com/isaakengineer/veha-rs)

### A.I. usage

The effort to translating _veha standards_ into a server side rendering enginge started alongside the time I was trying to experiment with A.I.

- the earlist attempts were prompts via [Qwen](https://qwen.ai/)
- later [DeepSeek](https://deepseek.com) chatbot was used to for starting input regarding areas I am not proficient in Rust
- finally the [GitHub Copilot](https://github.com/features/copilot) as a chat prompt to increase the efficiency of writing in Rust

The main credit for starting this project goes to Qwen, also because they released a coding assistant open-source which could be run locally, which ran with acceptable speed in previous gen CPU and integrated GPU!

### License

Copyright (C) 2025 Hossein Rezaei (pen name: Isaak Engineer)

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
