# HAD (Hypertext Auszeichungssprache Drucker)

The German name for our code-base can be translated to: "Hypertext Markup Language Printer".

CLI-program which takes XHTML documents and populates it with:

- CSV (comma separated values): think of it as excel files or tables but much more programmer friendlier and universally transferable
- Markdown: think of it as a word document but without all the fuss
- HTML: at the moment only simple nesting of an XHTML file as a template is supported

At the moment this project is at the pilot test stage.

I'm looking forward to find collaborators who are eager and passionate to bring this idea into a full-fledged web framework ...

## Quickstart

see the `beispiel` directory for examples.

Just clone and run it via cargo with the following input paths:

```
cargo run -- <input_file> <template_diretory> <output_file>
```
