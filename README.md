# Gofra for Zed

[Gofra](https://github.com/gofralang/gofra) language support for the
[Zed](https://zed.dev) editor.

![CI](https://github.com/gofralang/gofra-zed/actions/workflows/ci.yml/badge.svg)

## Features

- Syntax highlighting for `.gof` and `.gofra` files.
- Bracket matching and auto indentation (Gofra blocks are keyword delimited,
  `end` closes a block).
- Code outline for functions, lambdas, structures, unions and type aliases.

The highlighting is implemented with a [tree-sitter](https://tree-sitter.github.io)
grammar that lives in [gofralang/tree-sitter-gofra](https://github.com/gofralang/tree-sitter-gofra).

## Installation

### As a dev extension

1. Clone this repository:

   ```bash
   git clone https://github.com/gofralang/gofra-zed
   ```

2. In Zed open the extensions panel (`Cmd+Shift+X` / `Ctrl+Shift+X`), click the
   `...` menu and choose **Install Dev Extension**.
3. Select the `gofra-zed` directory.

The grammar is compiled by Zed on installation, so the first installation
downloads the WASI SDK (only once).

### From the extension registry

The extension is not published to the Zed extension registry yet.

## Repository layout

```
extension.toml                    the extension manifest
languages/gofra/config.toml       the language configuration
languages/gofra/highlights.scm    syntax highlighting queries
languages/gofra/brackets.scm      bracket matching queries
languages/gofra/indents.scm       auto indentation queries
languages/gofra/outline.scm       code outline queries
test/fixtures/example.gof         a sample file used by the CI
```

The extension has no Rust code - it only ships the tree-sitter grammar
reference and the tree-sitter queries.

## Development

The queries are validated by CI against the grammar that `extension.toml`
points at.

To work on the grammar itself, clone
[gofralang/tree-sitter-gofra](https://github.com/gofralang/tree-sitter-gofra)
and point `extension.toml` at the local checkout:

```toml
[grammars.gofra]
repository = "file:///path/to/tree-sitter-gofra"
rev = "<commit sha>"
```

Remember to commit the grammar before installing the dev extension - Zed
checks out the given revision.

## License

MIT
