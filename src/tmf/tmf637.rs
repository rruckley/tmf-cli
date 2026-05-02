//! TMF637 Product Inventory
//! 

use clap::Subcommand;
use tmflib::tmf637::v4::product::Product;

use crate::Output;

#[warn(unused_variables)]
use super::{display_name, display_opt, iterate_name, TMFOperation};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{BlockingOperations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF637Modules {
    Product {
        #[command(subcommand, help = "Product")]
        op: TMFOperation,
    },
}

pub fn handle_tmf637(
    client: &mut TMFClient,
    module: TMF637Modules,
    opts: Option<QueryOptions>,
    output: Output,
) -> Result<(), TMFError> {
    match module {
        TMF637Modules::Product { op } => {
            match op {
                TMFOperation::List => {
                    let products = client.tmf637().product().list(opts)?;
                    iterate_name(&products, output);
                    Ok(())
                }
                TMFOperation::Get { id } => {
                    let product = client.tmf637().product().get(id)?;
                    let the_first = product.first().unwrap();
                    display_name(the_first);
                    Ok(())
                }
                _ => Err(TMFError::from("Unsupported operation for Product")),
            }
        }
    }
}