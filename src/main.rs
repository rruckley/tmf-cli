
use clap::{Parser,Subcommand};
use log::info;
use tmf_client::{Operations, TMFClient};

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

fn main() {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    env_logger::init();
    info!(" {} v{}",pkg,ver);

    let args = Args::parse();

    // Find a host
    let host = match args.host {
        Some(h) => h,
        None => String::from("http://localhost:8000"),
    };
        
    info!("Using host :\t{}",&host); 

    let mut client = TMFClient::new(host);

    match args.tmf {
        Some(TMF::TMF620) => {
            let _cat = client.tmf620().catalog().list(None);
        },
        Some(TMF::TMF622) => todo!(),
        Some(TMF::TMF629) => todo!(),
        None => info!("Please choose an option"),
    }
}
