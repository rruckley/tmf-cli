//! TMF622 CLI Module
//!

use clap::Subcommand;

use super::{display_desc, iterate_desc, TMFOperation};

use crate::Output;
use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF622Modules {
    Order {
        #[command(subcommand, help = "ProductOrder")]
        op: TMFOperation,
    },
}

pub fn handle_tmf622(
    client: &mut TMFClient,
    module: TMF622Modules,
    opts: Option<QueryOptions>,
    output: Output,
) -> Result<(), TMFError> {
    match module {
        TMF622Modules::Order { op } => match op {
            TMFOperation::List => {
                let orders = client.tmf622().order().list(opts)?;
                iterate_desc(&orders, output);
                Ok(())
            }
            TMFOperation::Get { id } => {
                let order = client.tmf622().order().get(id)?;
                let the_first = order.first().unwrap();
                display_desc(the_first);
                Ok(())
            }
            _ => Err(TMFError::from("Not implemented")),
        },
    }
}
