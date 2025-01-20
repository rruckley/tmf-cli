//! TMF633 CLI Module

//! TMF620 CLI Module

use clap::Subcommand;

use super::{
    display_desc, display_name, iterate_name, TMFOperation
};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF633Modules {
    Catalog {
        #[command(subcommand, help = "Catalog Operations")]
        op : TMFOperation
    },
    Category {
        #[command(subcommand, help = "Category Operations")]
        op : TMFOperation      
    },
    Candidate {
        #[command(subcommand, help = "Service Cadidate Operations")]
        op : TMFOperation      

    },
    Specification {
        #[command(subcommand, help = "Product Specification Operations")]
        op : TMFOperation      

    },
}

pub fn handle_tmf633(client : &mut TMFClient, module : TMF633Modules, opts : Option<QueryOptions>) -> Result<(),TMFError> {
    match module {
        TMF633Modules::Catalog { op } => {
            match op {
                TMFOperation::List => {
                    let catalogs = client.tmf633().catalog().list(opts)?;
                    iterate_name(&catalogs);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let catalog = client.tmf633().catalog().get(id)?;
                    let the_first = catalog.first().unwrap();
                    display_name(the_first);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }
        },
        TMF633Modules::Category { op } => {
            match op {
                TMFOperation::List => {
                    let categories = client.tmf633().category().list(opts)?;
                    iterate_name(&categories);
                    Ok(())
                },
                _ => {
                    Err(TMFError::from("Not implemented")) 
                }
            }
        },
        TMF633Modules::Candidate { op } => {
            match op {
                TMFOperation::List => {
                    let candidates = client.tmf633().candidate().list(opts)?;
                    iterate_name(&candidates);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let candidate = client.tmf633().candidate().get(id)?;
                    let the_first = candidate.first().unwrap();
                    display_name(the_first);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented")) 
                }
            }
        },
        TMF633Modules::Specification { op } => {
            match op {
                TMFOperation::List => {
                    let specifications = client.tmf633().specification().list(opts)?;
                    iterate_name(&specifications);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let specification = client.tmf633().specification().get(id)?;
                    let the_first = specification.first().unwrap();
                    display_desc(the_first);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }

            }
        },
    }
}