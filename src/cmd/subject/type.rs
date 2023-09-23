use bc_envelope::prelude::*;
use clap::Args;

use crate::{data_types::{DataType, parse_data_type_to_envelope}, subject_args::{SubjectArgsLike, SubjectArgs}};

/// Create an envelope with the given subject.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    subject_args: SubjectArgs,
}

impl SubjectArgsLike for CommandArgs {
    fn subject_type(&self) -> DataType {
        self.subject_args.subject_type()
    }

    fn subject_value(&self) -> &str {
        self.subject_args.subject_value()
    }

    fn ur_tag(&self) -> Option<u64> {
        self.subject_args.ur_tag()
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        Ok(parse_data_type_to_envelope(self.subject_type(), self.subject_value(), self.ur_tag())?.ur_string())
    }
}
