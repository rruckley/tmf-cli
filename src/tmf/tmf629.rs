//! TMF629 CLI Module

use clap::Subcommand;
use tmflib::tmf629::customer::Customer;

use crate::Output;

use super::{
    display_name,
    display_opt,
    iterate_name,
    TMFOperation
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

pub fn handle_tmf629(client : &mut TMFClient, module : TMF629Modules, opts : Option<QueryOptions>,output : Output) -> Result<(),TMFError> {
    match module {
        TMF629Modules::Customer { op } => {
            match op {
                TMFOperation::Create { name, desc } => {
                    // Assumption is there is already an organization with the same name.
                    let org_name = match desc {
                        Some(d) => d,
                        None => name.clone(),
                    };
                    let filter = QueryOptions::default()
                        .name(&org_name);
                    let orgs = client.tmf632().organization().list(Some(filter))?;
                    if orgs.len() == 1 {
                        let org =orgs.first().unwrap(); 
                        let customer = Customer::new(org.clone());
                        let new_cust = client.tmf629().customer().create(customer)?;
                        display_name(&new_cust);
                        Ok(())
                    } else {
                        Err(TMFError::from(format!("Could not find matchig organization with the name: '{}'",org_name).as_str()))
                    }

                }
                TMFOperation::List => {
                    let customers = client.tmf629().customer().list(opts)?;
                    iterate_name(&customers,output);
                    Ok(())
                },
                TMFOperation::Get { id } => {
                    let customer = client.tmf629().customer().get(id)?;
                    let the_first = customer.first().unwrap();
                    display_name(the_first);
                    display_opt("Status",&the_first.status);
                    Ok(())
                }
                _ => {
                    Err(TMFError::from("Not implemented"))
                }
            }
        },
    }
}