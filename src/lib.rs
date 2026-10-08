//! Zed extension registering the hosted Crustdata MCP server as a context server.
//!
//! Crustdata's server is a remote streamable HTTP endpoint with OAuth 2.1. Zed's extension API
//! can only return a local process to spawn, so this extension runs the `mcp-remote` npm package
//! (installed by Zed at runtime) as a local stdio bridge to the hosted endpoint. The OAuth sign-in
//! happens in the browser on first use. No credentials are stored or read by this extension.

use std::env;
use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "mcp-remote";
const PACKAGE_VERSION: &str = "0.1.38";
const ENTRYPOINT: &str = "node_modules/mcp-remote/dist/proxy.js";
const SERVER_URL: &str = "https://install.crustdata.com/mcp";

struct CrustdataMcpExtension;

impl zed::Extension for CrustdataMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let installed = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let entrypoint = env::current_dir()
            .map_err(|e| format!("Failed to resolve the extension directory: {e}"))?
            .join(ENTRYPOINT);

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![
                entrypoint.to_string_lossy().to_string(),
                SERVER_URL.to_string(),
            ],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(CrustdataMcpExtension);
