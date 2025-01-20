//! TMF629 CLI Module

use clap::Subcommand;

use super::{
    display_name, iterate_name, TMFOperation
};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF629Modules {
    Customer {
        #[command(subcommand, help = "Customer")]
        op : TMFOperation
    },
}

pub fn handle_tmf629(client : &mut TMFClient, module : TMF629Modules, opts : Option<QueryOptions>) -> Result<(),TMFError> {
    match module {
        TMF629Modules::Customer { op } => {
            match op {
                TMFOperation::List => {
                    let customers = client.tmf629().customer().list(opts)?;
                    iterate_name(&customers);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let customer = client.tmf629().customer().get(id)?;
                    let the_first = customer.first().unwrap();
                    display_name(the_first);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }
        },
    }
}