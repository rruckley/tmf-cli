//! TMF620 CLI Module

use clap::Subcommand;
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::Category;

use crate::Output;

use super::{display_desc, display_json, display_name, display_opt, iterate_name, TMFOperation};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF620Modules {
    Catalog {
        #[command(subcommand, help = "Catalog Operations")]
        op: TMFOperation,
    },
    Category {
        #[command(subcommand, help = "Category Operations")]
        op: TMFOperation,
    },
    Offering {
        #[command(subcommand, help = "Product Offering Operations")]
        op: TMFOperation,
    },
    Specification {
        #[command(subcommand, help = "Product Specification Operations")]
        op: TMFOperation,
    },
    Price {
        #[command(subcommand, help = "Product Offering Price Operations")]
        op: TMFOperation,
    },
}

pub fn handle_tmf620(
    client: &mut TMFClient,
    module: TMF620Modules,
    opts: Option<QueryOptions>,
    output: Output,
) -> Result<(), TMFError> {
    match module {
        TMF620Modules::Catalog { op } => {
            match op {
                TMFOperation::Create { name, desc } => {
                    // Create a new object
                    let catalog = Catalog::new(name);
                    // .description(desc.unwrap_or_default());
                    let _new_catalog = client.tmf620().catalog().create(catalog)?;
                    // TODO: display_name(&new_catalog);
                    Ok(())
                }
                TMFOperation::List => {
                    let catalogs = client.tmf620().catalog().list(opts)?;
                    iterate_name(&catalogs, output);
                    Ok(())
                }
                TMFOperation::Get { id } => {
                    let catalog = client.tmf620().catalog().get(id)?;
                    let the_first = catalog.first().unwrap();
                    match output {
                        Output::Json => display_json(the_first),
                        Output::Text => display_name(the_first),
                    }
                    Ok(())
                }
                _ => Err(TMFError::from("Not implemented")),
            }
        }
        TMF620Modules::Category { op } => match op {
            TMFOperation::Create { name, desc } => {
                let category = Category::new(name).description(desc.unwrap_or_default());
                let new_cat = client.tmf620().category().create(category)?;
                display_name(&new_cat);
                display_opt("Desc", &new_cat.description);
                Ok(())
            }
            TMFOperation::List => {
                let categories = client.tmf620().category().list(opts)?;
                iterate_name(&categories, output);
                Ok(())
            }
            TMFOperation::Get { id } => {
                let category = client.tmf620().category().get(id)?;
                let the_first = category.first().unwrap();
                display_name(the_first);
                display_opt("Desc", &the_first.description);
                Ok(())
            }
            _ => Err(TMFError::from("Not implemented")),
        },
        TMF620Modules::Offering { op } => match op {
            TMFOperation::List => {
                let offerings = client.tmf620().product_offering().list(opts)?;
                iterate_name(&offerings, output);
                Ok(())
            }
            TMFOperation::Get { id } => {
                let offer = client.tmf620().product_offering().get(id)?;
                let the_first = offer.first().unwrap();
                display_desc(the_first);
                Ok(())
            }
            _ => Err(TMFError::from("Not implemented")),
        },
        TMF620Modules::Specification { op } => match op {
            TMFOperation::List => {
                let specifications = client.tmf620().product_specification().list(opts)?;
                iterate_name(&specifications, output);
                Ok(())
            }
            _ => Err(TMFError::from("Not implemented")),
        },
        TMF620Modules::Price { op } => match op {
            TMFOperation::List => {
                let prices = client.tmf620().product_offering_price().list(opts)?;
                iterate_name(&prices, output);
                Ok(())
            }
            _ => Err(TMFError::from("Not implemented")),
        },
    }
}
