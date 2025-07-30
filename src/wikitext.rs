// SPDX-FileCopyrightText: Oscar Bender-Stone <oscar-bender-stone@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Modified
// zed [https://github.com/zed-industries/zed]
// under the Apache 2.0 license.
// - Adapted language_server_binary
//   to be used with wiki_lsp.

use std::fs;
use zed::LanguageServerId;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

struct WikitextLspBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct WikitextExtension {
    cached_binary_path: Option<String>,
}

impl WikitextExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<WikitextLspBinary> {
        let binary_settings = LspSettings::for_worktree("wikitext-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(WikitextLspBinary {
                path,
                args: binary_args,
            });
        };

        if let Some(path) = worktree.which("wikitext-lsp") {
            return Ok(WikitextLspBinary {
                path,
                args: binary_args,
            });
        };

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(WikitextLspBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        };

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(WikitextLspBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        println!("BAD: failed to get wikitext-lsp");

        // TODO: add user option to add wikitext_lsp.
        // This will NOT be run unless user-configured.
        // zed::set_language_server_installation_status(
        //             language_server_id,
        //             &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        //         );
        return Err(String::from(
            "Failed to find wikitext-lsp on PATH. Make sure to install it through node.",
        ));
    }
}

impl zed::Extension for WikitextExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let wikitext_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: wikitext_binary.path,
            args: wikitext_binary
                .args
                .unwrap_or_else(|| vec!["--stdio".to_owned()]),
            env: Default::default(),
        })
    }
}

zed::register_extension!(WikitextExtension);
