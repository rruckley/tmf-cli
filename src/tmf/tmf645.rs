//! TMF645 API for Rust

use clap::Subcommand;
use tmflib::tmf645::check_service_qualification::CheckServiceQualification;
use tmflib::{
    HasDescription
    // HasNote
};

use crate::Output;

use super::{
    display_id,
    display_desc,
    display_opt,
    iterate_desc,
    TMFOperation
};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF645Modules {
    Qualification {
        #[command(subcommand, help = "Check Service Qualification")]
        op : TMFOperation
    },
}


pub fn handle_tmf645(client : &mut TMFClient, module : TMF645Modules, opts : Option<QueryOptions>,output : Output) -> Result<(),TMFError> {
    match module {
        TMF645Modules::Qualification { op } => {
            match op {
                TMFOperation::Create { name, desc } => {
                    let qualification  = CheckServiceQualification::new(name);
                    // qualification.set_name(name);
                    // if let Some(n) = desc {
                    //     // BUG: HasNote macro does not properly create the Vec<Note>
                    //     quote.note = Some(vec![Note::new(n)]);
                    //     // quote.add_note(Note::new(n));
                    // };
                    // quote.description = desc.clone();
                    let new_qual = client.tmf645().check_qualifcation().create(qualification)?;
                    display_id(&new_qual);
                    display_opt("Desc", &new_qual.description);
                    Ok(())
                }
                TMFOperation::List => {
                    let qualifications = client.tmf645().check_qualifcation().list(opts)?;
                    iterate_desc(&qualifications,output);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let quote = client.tmf648().quote().get(id)?;
                    let the_first = quote.first().unwrap();
                    display_desc(the_first);
                    display_opt("Category",&the_first.category);
                    display_opt("Version",&the_first.version);
                    // display_opt("External Id",&the_f)
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }
        },
    }
}