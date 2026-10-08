# Crustdata MCP for Zed

Zed extension that adds the hosted [Crustdata](https://crustdata.com) MCP server to the Agent Panel: real-time B2B data for agents, with search and enrichment across 1B+ people and 200M+ company profiles.

Docs: https://docs.crustdata.com/general/mcp

## How it works

Crustdata's server is a remote streamable HTTP endpoint (`https://install.crustdata.com/mcp`) secured with OAuth 2.1. Zed context-server extensions can only launch a local process, so this extension runs the [`mcp-remote`](https://www.npmjs.com/package/mcp-remote) npm package as a local stdio bridge to the hosted endpoint. Zed installs `mcp-remote` for you and uses its bundled Node.js.

On first use a browser window opens so you can sign in to Crustdata. The extension itself stores no credentials.

If you prefer not to use a bridge, Zed can also connect to remote servers natively: add this to your Zed settings.

```json
{
  "context_servers": {
    "crustdata": { "url": "https://install.crustdata.com/mcp" }
  }
}
```

## Development

Install as a dev extension from Zed (`zed: install dev extension`) and select this directory.

## License

MIT
