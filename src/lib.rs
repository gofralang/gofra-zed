//! Gofra extension for Zed.
//!
//! Provides tree-sitter based highlighting, outline, bracket matching and
//! auto indentation on its own, and wires up the Gofra language server
//! (`gofra lsp`) for diagnostics, document symbols, hover, go-to-definition
//! and completion.

use zed_extension_api::{self as zed, LanguageServerId, Result};

struct GofraExtension;

impl zed::Extension for GofraExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree.which("gofra") {
            return Ok(zed::Command {
                command: path,
                args: vec!["lsp".into()],
                env: Default::default(),
            });
        }

        Err(
            "gofra was not found on your PATH. Install it (`gofra lsp` is part of \
             the `gofra` CLI) or override the binary path in your settings.json: \
             \"lsp\": { \"gofra\": { \"binary\": { \"path\": \"...\", \"arguments\": [\"lsp\"] } }}"
                .into(),
        )
    }
}

zed::register_extension!(GofraExtension);
