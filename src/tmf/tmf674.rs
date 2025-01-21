//! TMF674 CLI Module

use clap::Subcommand;

use super::{
    display_name,
    display_opt,
    iterate_name,
    TMFOperation
};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF674Modules {
    Site {
        #[command(subcommand, help = "Geographic Site")]
        op : TMFOperation
    },
}

pub fn handle_tmf674(client : &mut TMFClient, module : TMF674Modules, opts : Option<QueryOptions>) -> Result<(),TMFError> {
    match module {
        TMF674Modules::Site { op } => {
            match op {
                TMFOperation::List => {
                    let sites = client.tmf674().site().list(opts)?;
                    iterate_name(&sites);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let site = client.tmf674().site().get(id)?;
                    let the_first = site.first().unwrap();
                    display_name(the_first);
                    display_opt("Description", &the_first.description);
                    display_opt("Code",&the_first.code);
                    display_opt("Status", &the_first.status);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }
        },
    }
}