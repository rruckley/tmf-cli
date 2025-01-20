//! TMF632 CLI Module

use clap::Subcommand;

use super::{
    display_name, display_opt, iterate_name, TMFOperation
};

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, QueryOptions, TMFClient};

#[derive(Subcommand, Clone, Debug)]
pub enum TMF632Modules {
    Individual {
        #[command(subcommand, help = "Operations on Individuals")]
        op : TMFOperation
    },
    Organization {
        #[command(subcommand, help = "Organization Operations")]
        op : TMFOperation
    },
}

pub fn handle_tmf632(client : &mut TMFClient, module : TMF632Modules, opts : Option<QueryOptions>) -> Result<(),TMFError> {
    match module {
        TMF632Modules::Individual { op } => {
            match op {
                TMFOperation::List => {
                    let individuals = client.tmf632().individual().list(opts)?;
                    iterate_name(&individuals);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let individual = client.tmf632().individual().get(id)?;
                    let the_first = individual.first().unwrap();
                    display_name(the_first);
                    display_opt("DOB", &the_first.birth_date);
                    display_opt("Title", &the_first.title);
                    display_opt("Gender", &the_first.gender);
                    display_opt("email:",&the_first.get_email());
                    // display_opt("Code", &the_first.c);
                    display_opt("Mobile", &the_first.get_mobile());
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }
        },
        TMF632Modules::Organization { op } => {
            match op {
                TMFOperation::List => {
                    let organization = client.tmf632().organization().list(opts)?;
                    iterate_name(&organization);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let organization = client.tmf632().organization().get(id)?;
                    let the_first = organization.first().unwrap();
                    display_name(the_first);
                    display_opt("Trading Name",&the_first.trading_name);
                    display_opt("Org. Type", &the_first.organization_type);
                    // display_opt("Status", &the_first.status);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }    
        }
    }
}