
use clap::{Parser,Subcommand};
use log::{info,error};
use tmf_client::TMFClient;
use tmf_client::common::tmf_error::TMFError;

mod tmf;
use tmf::tmf620::{
    TMF620Modules,
    handle_tmf620,
};
use tmf::tmf622::{
    TMF622Modules,
    handle_tmf622,
};
use tmf::tmf632::{
    TMF632Modules,
    handle_tmf632,
};
use tmf::tmf629::{
    TMF629Modules,
    handle_tmf629,
};
use tmf::tmf633::{
    TMF633Modules,
    handle_tmf633,
};
use tmf::tmf645::{
    TMF645Modules,
    handle_tmf645,
};
use tmf::tmf648::{
    TMF648Modules,
    handle_tmf648,
};
use tmf::tmf674::{
    TMF674Modules,
    handle_tmf674,
};

pub enum Output {
    Text,
    Json,
}

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

    #[clap(global = true)]
    #[arg(short = 'j', long, action)]
    json: bool,
}

#[derive(Subcommand,Debug)]
pub enum TMFModules {
    TMF620 {
        #[command(subcommand, help = "Product Catalog")]
        module : TMF620Modules,
    },
    TMF622 {
        #[command(subcommand, help = "Product Order")]
        module : TMF622Modules,
    },
    TMF629 {
        #[command(subcommand, help = "Customer")]
        module : TMF629Modules,
    },
    TMF632 {
        #[command(subcommand, help = "Party")]
        module : TMF632Modules,
    },
    TMF633 {
        #[command(subcommand, help = "Service Catalog")]
        module : TMF633Modules,
    },
    TMF645 {
        #[command(subcommand, help = "Service Qualification")]
        module : TMF645Modules,
    },
    TMF648 {
        #[command(subcommand, help = "Product Quote")]
        module : TMF648Modules,
    },
    TMF674 {
        #[command(subcommand, help = "Geographic Site")]
        module : TMF674Modules,
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

    let output = match args.json {
            true => Output::Json,
            false => Output::Text,
    };

    // Find a host
    let host = match args.hostname {
        Some(h) => h,
        None => String::from("http://localhost:8001"),
    };
        
    info!("Host\t: {}",&host); 

    let mut client = TMFClient::new(host);

    let result = match args.tmf {
        TMFModules::TMF620 { module } => {
            handle_tmf620(&mut client, module, Some(opts),output)
        },
        TMFModules::TMF622 { module } => {
            handle_tmf622(&mut client, module, Some(opts),output)
        },
        TMFModules::TMF629 { module } => {
            handle_tmf629(&mut client, module, Some(opts),output)
        },
        TMFModules::TMF632 { module } => {
            handle_tmf632(&mut client, module, Some(opts),output)
        },
        TMFModules::TMF633 { module } => {
            handle_tmf633(&mut client, module, Some(opts),output)
        },
        TMFModules::TMF645 { module } => {
            handle_tmf645(&mut client, module, Some(opts),output)
        },
        TMFModules::TMF648 { module } => {
            handle_tmf648(&mut client, module, Some(opts),output)
        },
        TMFModules::TMF674 { module } => {
            handle_tmf674(&mut client, module, Some(opts),output)
        }
    };
    match result {
        Ok(r) => {
            info!("Successful operation");
            Ok(r)
        },
        Err(e) => {
            error!("Operation failed: {}",e.message);
            Err(e)
        },
    }
}