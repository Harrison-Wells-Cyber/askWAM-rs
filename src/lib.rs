//! The platform-independent CLI and request/output contracts of the native client.
pub mod options;
pub mod report;

#[cfg(windows)]
pub mod wam;

pub const DEFAULT_CLIENT_ID: &str = "1fec8e78-bce4-4aaf-ab1b-5451cc387264";
pub const DEFAULT_SCOPE: &str = "https://graph.microsoft.com/.default";
pub const DEFAULT_AUTHORITY: &str = "organizations";
pub const PROVIDER_ID: &str = "https://login.microsoft.com";
pub const CAE_CLAIMS: &str = r#"{"access_token":{"xms_cc":{"values":["cp1"]}}}"#;
pub const RESERVED_SCOPES: &str = " openid offline_access profile";

pub const HELP: &str = "askWAM - silent Windows WAM token acquisition (Rust)

Usage: askwam [options]

  --client-id VALUE       Public client ID (default: Microsoft Teams)
  --scope VALUE           v2 scope string (default: Graph .default)
  --resource VALUE        WAM v1-compatible resource; exclusive with --scope
  --authority VALUE       Provider authority (default: organizations)
  --enum                  Enumerate accounts and exit
  --account-id VALUE      Select an exact WAM account ID
  --username VALUE        Select one exact, unique username (case-insensitive)
  --claims-json VALUE     Raw OAuth claims object
  --cae                   Request CAE client capability cp1
  --hide                  Omit the access token from output
  --timeout-seconds N     1-300 seconds per operation (default: 30)
  -h, --help              Show this help

Scope mode adds openid, offline_access, and profile for WAM v2 compatibility.
Resource mode sends an empty scope plus the WAM resource property.
The client never performs interactive authentication.
";
