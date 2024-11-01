
use clap::{Parser,Subcommand};
use log::info;
use tmf_client::{Operations, TMFClient};
use tmf_client::common::tmf_error::TMFError;
use tmflib::{HasId,HasName};

#[derive(Parser,Debug)]
struct Args {
    #[arg(long, help = "Override HOST environment variable")]
    host: Option<String>,

    #[command(subcommand)]
    tmf: Option<TMF>,
}

#[derive(Subcommand,Debug)]
pub enum TMF {
    TMF620,
    TMF622,
    TMF629,
}

fn main() -> Result<(),TMFError> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    env_logger::init();
    info!("Version\t: {} v{}",pkg,ver);

    let args = Args::parse();

    // Find a host
    let host = match args.host {
        Some(h) => h,
        None => String::from("http://localhost:8000"),
    };
        
    info!("Host\t: {}",&host); 

    let mut client = TMFClient::new(host);

    match args.tmf {
        Some(TMF::TMF620) => {
            let cat = client.tmf620().catalog().list(None)?;
            cat.iter().for_each(|c| {
                info!("Catalog\t: {} [{}]",c.get_name(),c.get_id());
            });
            Ok(())
        },
        Some(TMF::TMF622) => {
            let order = client.tmf622().order().list(None)?;
            order.iter().for_each(|o| {
                info!("Order\t: {} [{}]",o.description.clone().unwrap_or("No description".to_string()),o.get_id());
            });
            Ok(())
        }
        Some(TMF::TMF629) => {
            Err(TMFError::from("tmf-client: TMF629 not implemented"))
        },
        None => {
            info!("Please choose an option");
            Ok(())
        },
    }
}
