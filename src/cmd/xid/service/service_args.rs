use bc_components::URI;
use bc_envelope::PublicKeyBase;
use bc_ur::URDecodable;
use bc_xid::XIDDocument;
use anyhow::Result;
use clap::Args;

use crate::cmd::xid::{utils::read_uri, xid_privilege::XIDPrivilege};

pub trait ServiceArgsLike {
    fn uri(&self) -> Option<&URI>;
    fn name(&self) -> &str;
    fn capability(&self) -> &str;
    fn permissions(&self) -> &[XIDPrivilege];
    fn keys(&self) -> &[PublicKeyBase];
    fn delegates(&self) -> &[XIDDocument];

    fn read_uri(&self) -> Result<URI> {
        read_uri(self.uri())
    }
}

fn parse_public_key_base(s: &str) -> Result<PublicKeyBase, String> {
    PublicKeyBase::from_ur_string(s).map_err(|e| e.to_string())
}

fn parse_xid_document(s: &str) -> Result<XIDDocument, String> {
    XIDDocument::from_ur_string(s).map_err(|e| e.to_string())
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct ServiceArgs {
    /// A user-assigned name for the key.
    #[arg(long, default_value = "")]
    name: String,

    /// The capability identifier of the service.
    #[arg(long, default_value = "")]
    capability: String,

    /// A specific key for use with the service. May be repeated.
    #[arg(long = "key", name = "PUBLIC_KEY_BASE", num_args = 1)]
    #[clap(value_parser = parse_public_key_base)]
    keys: Vec<PublicKeyBase>,

    /// A delegate for the service. May be repeated.
    #[arg(long = "delegate", name = "XID", num_args = 1)]
    #[clap(value_parser = parse_xid_document)]
    delegates: Vec<XIDDocument>,

    /// Grant a specific permission to the service. May be repeated.
    #[arg(long = "allow", name = "PRIVILEGE", default_value = "all", num_args = 1)]
    permissions: Vec<XIDPrivilege>,

    /// The service URI. If omitted, the URI will be read from stdin.
    uri: Option<URI>,
}

impl ServiceArgsLike for ServiceArgs {
    fn name(&self) -> &str {
        &self.name
    }

    fn capability(&self) -> &str {
        &self.capability
    }

    fn keys(&self) -> &[PublicKeyBase] {
        &self.keys
    }

    fn delegates(&self) -> &[XIDDocument] {
        &self.delegates
    }

    fn permissions(&self) -> &[XIDPrivilege] {
        &self.permissions
    }

    fn uri(&self) -> Option<&URI> {
        self.uri.as_ref()
    }
}
