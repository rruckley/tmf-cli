
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
    #[arg(short = 'l',long)]
    limit: Option<u16>,

    #[clap(global = true)]
    #[arg(short = 'o',long)]
    offset: Option<u16>,

    #[clap(global = true)]
    #[arg(short = 'n', long)]
    name: Option<String>,
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

    let mut opts = tmf_client::QueryOptions::default();
    if let Some(l) = args.limit {
        opts = opts.limit(l);
    };
    if let Some(o) = args.offset {
        opts = opts.offset(o);
    };
    if let Some(n) = args.name {
        opts = opts.name(n);
    }

    // Find a host
    let host = match args.hostname {
        Some(h) => h,
        None => String::from("http://localhost:8001"),
    };
        
    info!("Host\t: {}",&host); 

    let mut client = TMFClient::new(host);

    match args.tmf {
        TMFModules::TMF620 { module } => {
            handle_tmf620(&mut client, module, Some(opts))
        }
    }
}