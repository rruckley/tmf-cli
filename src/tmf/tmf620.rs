//! Handle TMF620 operations

use clap::Subcommand;

use super::{
    display_desc, display_name, iterate_name, TMFOperation
};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF620Modules {
    Catalog {
        #[command(subcommand, help = "Catalog Operations")]
        op : TMFOperation
    },
    Category {
        #[command(subcommand, help = "Category Operations")]
        op : TMFOperation      
    },
    Offering {
        #[command(subcommand, help = "Product Offering Operations")]
        op : TMFOperation      

    },
    Specification {
        #[command(subcommand, help = "Product Specification Operations")]
        op : TMFOperation      

    },
    Price {
        #[command(subcommand, help = "Product Offering Price Operations")]
        op : TMFOperation      

    },
}

pub fn handle_tmf620(client : &mut TMFClient, module : TMF620Modules, opts : Option<QueryOptions>) -> Result<(),TMFError> {
    match module {
        TMF620Modules::Catalog { op } => {
            match op {
                TMFOperation::List => {
                    let catalogs = client.tmf620().catalog().list(opts)?;
                    iterate_name(&catalogs);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let catalog = client.tmf620().catalog().get(id)?;
                    let the_first = catalog.first().unwrap();
                    display_name(the_first);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }
        },
        TMF620Modules::Category { op } => {
            match op {
                TMFOperation::List => {
                    let categories = client.tmf620().category().list(opts)?;
                    iterate_name(&categories);
                    Ok(())
                },
                _ => {
                    Err(TMFError::from("Not implemented")) 
                }
            }
        },
        TMF620Modules::Offering { op } => {
            match op {
                TMFOperation::List => {
                    let offerings = client.tmf620().product_offering().list(opts)?;
                    iterate_name(&offerings);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let offer = client.tmf620().product_offering().get(id)?;
                    let the_first = offer.first().unwrap();
                    display_desc(the_first);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented")) 
                }
            }
        },
        TMF620Modules::Specification { op } => {
            match op {
                TMFOperation::List => {
                    let specifications = client.tmf620().product_specification().list(opts)?;
                    iterate_name(&specifications);
                    Ok(())
                },
                _ => {
                    Err(TMFError::from("Not implemented"))
                }

            }
        },
        TMF620Modules::Price { op } => {
            match op {
                TMFOperation::List => {
                    let prices = client.tmf620().product_offering_price().list(opts)?;
                    iterate_name(&prices);
                    Ok(())
                },
                _ => {
                    Err(TMFError::from("Not implemented")) 
                }    
            }
        }
    }
}