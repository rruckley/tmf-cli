
use clap::{Parser,Subcommand};
use log::info;
use tmf_client::{Operations, TMFClient};
use tmf_client::common::tmf_error::TMFError;
use tmflib::{HasDescription, HasId, HasName};

#[derive(Parser,Debug)]
#[command(version, about)]
struct Args {
    #[arg(long, help = "Override HOST environment variable")]
    hostname: Option<String>,

    #[command(subcommand)]
    tmf : Option<TMF>,

    #[clap(global = true)]
    #[arg(short = 'l')]
    limit: Option<u32>,

    #[clap(global = true)]
    #[arg(short = 'o')]
    offset: Option<u32>,
}

#[derive(Clone, Subcommand,Debug)]
pub enum TMF {
    TMF620 {
        #[command(subcommand)]
        op: Operation
    },
    TMF622 {
        #[command(subcommand)]
        op: Operation
    },
    TMF629 {
        #[command(subcommand)]
        op : Operation
    },
    TMF632 {
        #[command(subcommand)]
        op : Operation
    },
    TMF633 {
        #[command(subcommand)]
        op : Operation
    },
    TMF648 {
        #[command(subcommand)]
        op : Operation
    },
    TMF674 {
        #[command(subcommand)]
        op : Operation
    }
}

#[derive(Clone, Subcommand,Debug)]
pub enum Operation {
    List,
    Get,
    Create,
    Update,
    Delete
}

fn iterate_name<T : HasId + HasName>(items : &Vec<T>) {
    items.iter().for_each(|i| {
        println!("Item: [{}] {} [{}]",T::get_class(),i.get_name(),i.get_id());
    });
}

fn iterate_desc<T : HasId + HasDescription>(items : &Vec<T>) {
    items.iter().for_each(|i| {
        println!("Item: [{}] {} [{}]",T::get_class(),i.get_description(),i.get_id());
    });
}

fn main() -> Result<(),TMFError> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    env_logger::init();
    info!("Version\t: {} v{}",pkg,ver);

    let args = Args::parse();

    // Find a host
    let host = match args.hostname {
        Some(h) => h,
        None => String::from("http://localhost:8001"),
    };
        
    info!("Host\t: {}",&host); 

    let mut client = TMFClient::new(host);

    match args.tmf {
        Some(o) => {
            match o {
                TMF::TMF620 { op } => {
                    let catalogs = client.tmf620().catalog().list(None)?;
                    iterate_name(&catalogs);
                },
                TMF::TMF622 { op } => {
                    let orders = client.tmf622().order().list(None)?;
                    iterate_desc(&orders);

                },
                TMF::TMF629 { op } => {
                    let customers = client.tmf629().customer().list(None)?;
                    iterate_name(&customers);
                },
                TMF::TMF632 { op } => {
                    let individuals = client.tmf632().individual().list(None)?;
                    iterate_name(&individuals);
                }
                TMF::TMF633 { op } => {
                    let candidates = client.tmf633().candidate().list(None)?;
                    iterate_name(&candidates);
                }
                TMF::TMF648 { op } => {
                    let quotes = client.tmf648().quote().list(None)?;
                    iterate_desc(&quotes);
                },
                TMF::TMF674 { op } => {
                    let sites = client.tmf674().site().list(None)?;
                    iterate_name(&sites);
                }
            }
            Ok(())
        },
        None => {
            Err(TMFError::from("No option selected"))
        }
    }
}
