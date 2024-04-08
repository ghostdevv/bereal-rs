use chrono::{Local, TimeZone};
use clap::{Parser, Subcommand};
use color_eyre::eyre::{eyre, Result};

#[derive(Parser, Debug)]
#[command(version, about, author)]
struct Cli {
    #[arg(long, short = 'k', help = "Your bereal.devin.fun api key")]
    api_key: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get the latest moments from bereal
    Latest {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = bereal::BerealClient::new(&cli.api_key);

    match cli.command {
        Commands::Latest {} => {
            let latest = client.latest_moments().await?;

            println!("Europe:\t\t{}", fmt_date(latest.regions.europe_west.ts)?);
            println!("America:\t{}", fmt_date(latest.regions.us_central.ts)?);
            println!("Asia east:\t{}", fmt_date(latest.regions.asia_east.ts)?);
            println!("Asia west:\t{}", fmt_date(latest.regions.asia_west.ts)?);
        }
    }

    Ok(())
}

fn fmt_date(timestamp: i64) -> Result<String> {
    let datetime = Local.timestamp_opt(timestamp, 0).single();

    match datetime {
        Some(ts) => Ok(ts.format("%Y-%m-%d %H:%M:%S").to_string()),
        None => Err(eyre!("No timestamp found")),
    }
}
