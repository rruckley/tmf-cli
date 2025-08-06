//! TMF648 CLI Module

use clap::Subcommand;
use tmflib::common::note::Note;
use tmflib::tmf648::quote::Quote;
use tmflib::{
    HasName,
    // HasNote
};

use crate::Output;

use super::{display_desc, display_id, display_opt, iterate_name, TMFOperation};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF648Modules {
    Quote {
        #[command(subcommand, help = "Product Quote")]
        op: TMFOperation,
    },
}

pub fn handle_tmf648(
    client: &mut TMFClient,
    module: TMF648Modules,
    opts: Option<QueryOptions>,
    output: Output,
) -> Result<(), TMFError> {
    match module {
        TMF648Modules::Quote { op } => {
            match op {
                TMFOperation::Create { name, desc } => {
                    let mut quote = Quote::new();
                    quote.set_name(name);
                    if let Some(n) = desc {
                        // BUG: HasNote macro does not properly create the Vec<Note>
                        quote.note = Some(vec![Note::new(n)]);
                        // quote.add_note(Note::new(n));
                    };
                    // quote.description = desc.clone();
                    let new_quote = client.tmf648().quote().create(quote)?;
                    display_id(&new_quote);
                    display_opt("Desc", &new_quote.description);
                    Ok(())
                }
                TMFOperation::List => {
                    let quotes = client.tmf648().quote().list(opts)?;
                    iterate_name(&quotes, output);
                    Ok(())
                }
                TMFOperation::Get { id } => {
                    let quote = client.tmf648().quote().get(id)?;
                    let the_first = quote.first().unwrap();
                    display_desc(the_first);
                    display_opt("Category", &the_first.category);
                    display_opt("Version", &the_first.version);
                    // display_opt("External Id",&the_f)
                    Ok(())
                }
                _ => Err(TMFError::from("Not implemented")),
            }
        }
    }
}
