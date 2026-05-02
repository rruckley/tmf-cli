//! Service Inventory (TM Forum TMF638)

use clap::Subcommand;
use tmflib::tmf638::service::Service;
use crate::Output;

#[warn(unused_variables)]
use super::{display_name, display_opt, iterate_name, TMFOperation};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF638Modules {
    Service {
        #[command(subcommand, help = "Service")]
        op: TMFOperation,
    },
}

pub fn handle_tmf638(
    client: &mut TMFClient,
    module: TMF638Modules,
    opts: Option<QueryOptions>,
    output: Output,
) -> Result<(), TMFError> {
    match module {
        TMF638Modules::Service { op } => {
            match op {
                TMFOperation::List => {
                    let services = client.tmf638().service().list(opts)?;
                    iterate_name(&services, output);
                    Ok(())
                }
                TMFOperation::Get { id } => {
                    let service = client.tmf638().service().get(id)?;
                    let the_first = service.first().unwrap();
                    display_name(the_first);
                    Ok(())
                }
                _ => Err(TMFError::from("Unsupported operation for Service")),
            }
        }
    }
}