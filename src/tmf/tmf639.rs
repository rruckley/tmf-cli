//! TMF639 Resource Inventory 

use clap::Subcommand;
use tmflib::tmf639::resource::Resource;
use crate::Output;

#[warn(unused_variables)]
use super::{display_name, display_opt, iterate_name, TMFOperation};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF639Modules {
    Resource {
        #[command(subcommand, help = "Resource")]
        op: TMFOperation,
    },
}

pub fn handle_tmf639(
    client: &mut TMFClient,
    module: TMF639Modules,
    opts: Option<QueryOptions>,
    output: Output,
) -> Result<(), TMFError> {
    match module {
        TMF639Modules::Resource { op } => {
            match op {
                TMFOperation::List => {
                    let resources = client.tmf639().resource().list(opts)?;
                    iterate_name(&resources, output);
                    Ok(())
                }
                TMFOperation::Get { id } => {
                    let resource = client.tmf639().resource().get(id)?;
                    let the_first = resource.first().unwrap();
                    display_name(the_first);
                    Ok(())
                }
                _ => Err(TMFError::from("Unsupported operation for Resource")),
            }
        }
    }
}