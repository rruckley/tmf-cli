//! TMF633 CLI Module

//! TMF620 CLI Module

use clap::Subcommand;
use tmflib::tmf633::service_candidate::ServiceCandidate;
use tmflib::tmf633::service_catalog::ServiceCatalog;
use tmflib::tmf633::service_category::ServiceCategory;
use tmflib::tmf633::service_specification::ServiceSpecification;
use tmflib::HasDescription;

use crate::Output;

use super::{
    display_desc, display_name, display_opt, iterate_name, TMFOperation
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

pub fn handle_tmf633(client : &mut TMFClient, module : TMF633Modules, opts : Option<QueryOptions>, output : Output) -> Result<(),TMFError> {
    match module {
        TMF633Modules::Catalog { op } => {
            match op {
                TMFOperation::Create { name, desc } => {
                    let catalog = ServiceCatalog::new(name);
                    let new_cat = client.tmf633().catalog().create(catalog)?;
                    display_name(&new_cat);
                    Ok(())
                }
                TMFOperation::List => {
                    let catalogs = client.tmf633().catalog().list(opts)?;
                    iterate_name(&catalogs,output);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let catalog = client.tmf633().catalog().get(id)?;
                    let the_first = catalog.first().unwrap();
                    display_name(the_first);
                    Ok(())
                },
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }
        },
        TMF633Modules::Category { op } => {
            match op {
                TMFOperation::Create { name, desc } => {
                    let category = ServiceCategory::new(name)
                        .description(desc.unwrap_or_default());
                    let new_cat = client.tmf633().category().create(category)?;
                    display_name(&new_cat);
                    display_desc(&new_cat);
                    Ok(())
                }
                TMFOperation::List => {
                    let categories = client.tmf633().category().list(opts)?;
                    iterate_name(&categories,output);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let category = client.tmf633().category().get(id)?;
                    let the_first = category.first().unwrap();
                    display_name(the_first);
                    Ok(())
                },
                _ => {
                    Err(TMFError::from("Not implemented")) 
                }
            }
        },
        TMF633Modules::Candidate { op } => {
            match op {
                TMFOperation::Create { name, desc } => {
                    // Need to use the desc field as the id of a specificaton to link to this candidate
                    // let candidate = ServiceCandidate::new(name, specification_ref)
                    Err(TMFError::from("Not implemented"))
                }
                TMFOperation::List => {
                    let candidates = client.tmf633().candidate().list(opts)?;
                    iterate_name(&candidates,output);
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
                TMFOperation::Create { name, desc } => {
                    let specification = ServiceSpecification::new(name)
                    .description(desc.unwrap_or_default());
                    let new_spec = client.tmf633().specification().create(specification)?;
                    display_name(&new_spec);
                    display_opt("Desc", &new_spec.description);
                    Ok(())
                },
                TMFOperation::List => {
                    let specifications = client.tmf633().specification().list(opts)?;
                    iterate_name(&specifications,output);
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