//! Game-facing service endpoints: content, catalog, MCP commands. Not implemented in this base
//! project.

use serde_json::{json, Value};

/// Always an empty-shaped placeholder: no content page, no catalog, no MCP commands.
pub fn placeholder_response() -> Value {
    json!({ "error": "Not implemented in the RetroVerse base project." })
}
