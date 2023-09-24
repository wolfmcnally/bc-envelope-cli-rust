use clap::Args;

use crate::envelope_args::{EnvelopeArgs, EnvelopeArgsLike};

use super::elide_args::{ElideArgs, ElideArgsLike, Action};
use bc_envelope::prelude::*;

/// Elide all objects in the target.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    elide_args: ElideArgs,

    #[command(flatten)]
    envelope_args: EnvelopeArgs,
}

impl ElideArgsLike for CommandArgs {
    fn action(&self) -> Action {
        self.elide_args.action()
    }

    fn key(&self) -> Option<&str> {
        self.elide_args.key()
    }

    fn target(&self) -> &String {
        self.elide_args.target()
    }
}

impl EnvelopeArgsLike for CommandArgs {
    fn envelope(&self) -> Option<&str> {
        self.envelope_args.envelope()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let envelope = self.get_envelope()?;
        let result_envelope = self.run(envelope, false)?;
        Ok(result_envelope.ur_string())
    }
}
