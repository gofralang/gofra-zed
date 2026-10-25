# Gofra for Zed

[Gofra](https://github.com/gofralang/gofra) language support for the
[Zed](https://zed.dev) editor.

![CI](https://github.com/gofralang/gofra-zed/actions/workflows/ci.yml/badge.svg)

## Features

- Syntax highlighting for `.gof` and `.gofra` files.
- Bracket matching and auto indentation (Gofra blocks are keyword delimited,
  `end` closes a block).
- Code outline for functions, lambdas, structures, unions and type aliases.
- **Language server** integration (`gofra lsp`): diagnostics (syntax / type / lint),
  document symbols, hover, go-to-definition and completion. The server speaks the
  [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
  over stdin/stdout, so it also works in any other LSP capable editor.

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
Cargo.toml / src/lib.rs           the language server shim
rust-toolchain.toml              wasm32 compiler target
languages/gofra/config.toml       the language configuration
languages/gofra/highlights.scm    syntax highlighting queries
languages/gofra/brackets.scm      bracket matching queries
languages/gofra/indents.scm       auto indentation queries
languages/gofra/outline.scm       code outline queries
test/fixtures/example.gof         a sample file used by the CI
```

The extension ships the tree-sitter grammar reference and the tree-sitter queries,
plus a small Rust shim (`src/lib.rs`) that launches `gofra lsp` for the Zed
language server.

## Installation prerequisites

The language server is launched with `gofra lsp`. Make sure `gofra` is on the
PATH that Zed inherits (installing via `pipx` or `pip install -e` on the
[gofra](https://github.com/gofralang/gofra) repository is enough). When this is
not possible, override the binary in `settings.json`:

```json
{
  "lsp": {
    "gofra": {
      "binary": { "path": "/absolute/path/to/gofra", "arguments": ["lsp"] }
    }
  }
}
```

The `lsp.gofra.binary` setting has priority over the PATH lookup performed by
the extension.

## Development

The queries are validated by CI against the grammar that `extension.toml`
points at. The extension itself is built by Zed as a WebAssembly module, which
requires the `cargo` toolchain and the `wasm32-wasip1` target:

```bash
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --release
```

Zed builds the dev extension through the locally installed `cargo`; install it
with `rustup` if it is missing.

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
