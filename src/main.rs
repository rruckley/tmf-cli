use clap::{Parser, Subcommand};
use log::{error, info};
use tmf_client::common::tmf_error::TMFError;
use tmf_client::TMFClient;

mod tmf;
use tmf::tmf620::{handle_tmf620, TMF620Modules};
use tmf::tmf622::{handle_tmf622, TMF622Modules};
use tmf::tmf629::{handle_tmf629, TMF629Modules};
use tmf::tmf632::{handle_tmf632, TMF632Modules};
use tmf::tmf633::{handle_tmf633, TMF633Modules};
use tmf::tmf645::{handle_tmf645, TMF645Modules};
use tmf::tmf648::{handle_tmf648, TMF648Modules};
use tmf::tmf674::{handle_tmf674, TMF674Modules};

pub enum Output {
    Text,
    Json,
}

const HOSTNAME : &str = "https://localhost:8001";

#[derive(Parser, Debug)]
#[command(
    version,
    about = "CLI tool for interacting with TMF APIs",
    author = "Ryan Ruckley"
)]
struct Args {
    #[arg(long, help = "Override HOST environment variable")]
    hostname: Option<String>,

    #[command(subcommand)]
    tmf: TMFModules,

    #[clap(global = true)]
    #[arg(short = 'l', long)]
    limit: Option<u16>,

    #[clap(global = true)]
    #[arg(short = 'o', long)]
    offset: Option<u16>,

    #[clap(global = true)]
    #[arg(short = 'n', long)]
    name: Option<String>,

    #[clap(global = true)]
    #[arg(short = 'j', long, action)]
    json: bool,
}

#[derive(Subcommand, Debug)]
pub enum TMFModules {
    TMF620 {
        #[command(subcommand, help = "Product Catalog")]
        module: TMF620Modules,
    },
    TMF622 {
        #[command(subcommand, help = "Product Order")]
        module: TMF622Modules,
    },
    TMF629 {
        #[command(subcommand, help = "Customer")]
        module: TMF629Modules,
    },
    TMF632 {
        #[command(subcommand, help = "Party")]
        module: TMF632Modules,
    },
    TMF633 {
        #[command(subcommand, help = "Service Catalog")]
        module: TMF633Modules,
    },
    TMF645 {
        #[command(subcommand, help = "Service Qualification")]
        module: TMF645Modules,
    },
    TMF648 {
        #[command(subcommand, help = "Product Quote")]
        module: TMF648Modules,
    },
    TMF674 {
        #[command(subcommand, help = "Geographic Site")]
        module: TMF674Modules,
    },
}

fn main() -> Result<(), TMFError> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    env_logger::init();
    info!("Version\t: {} v{}", pkg, ver);

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
        None => String::from(HOSTNAME),
    };

    info!("Host\t: {}", &host);

    let mut client = TMFClient::new(host);

    let result = match args.tmf {
        TMFModules::TMF620 { module } => handle_tmf620(&mut client, module, Some(opts), output),
        TMFModules::TMF622 { module } => handle_tmf622(&mut client, module, Some(opts), output),
        TMFModules::TMF629 { module } => handle_tmf629(&mut client, module, Some(opts), output),
        TMFModules::TMF632 { module } => handle_tmf632(&mut client, module, Some(opts), output),
        TMFModules::TMF633 { module } => handle_tmf633(&mut client, module, Some(opts), output),
        TMFModules::TMF645 { module } => handle_tmf645(&mut client, module, Some(opts), output),
        TMFModules::TMF648 { module } => handle_tmf648(&mut client, module, Some(opts), output),
        TMFModules::TMF674 { module } => handle_tmf674(&mut client, module, Some(opts), output),
    };
    match result {
        Ok(r) => {
            info!("Successful operation");
            Ok(r)
        }
        Err(e) => {
            error!("Operation failed: {}", e);
            Err(e)
        }
    }
}
