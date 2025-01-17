
use clap::{Parser,Subcommand};
use log::info;
use tmf_client::TMFClient;
use tmf_client::common::tmf_error::TMFError;

mod tmf;
use tmf::tmf620::{
    TMF620Modules,
    handle_tmf620,
};

#[derive(Parser,Debug)]
#[command(version, about = "CLI tool for interacting with TMF APIs", author = "Ryan Ruckley")]
struct Args {
    #[arg(long, help = "Override HOST environment variable")]
    hostname: Option<String>,

    #[command(subcommand)]
    tmf : TMFModules,

    #[clap(global = true)]
    #[arg(short = 'l')]
    limit: Option<u32>,

    #[clap(global = true)]
    #[arg(short = 'o')]
    offset: Option<u32>,
}





#[derive(Subcommand,Debug)]
pub enum TMFModules {
    TMF620 {
        #[command(subcommand, help = "Product Catalog")]
        module : TMF620Modules,
    }
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
        TMFModules::TMF620 { module } => {
            handle_tmf620(&mut client, module)
        }
    }
}