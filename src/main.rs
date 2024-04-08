use clap::Parser;
use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, author)]
struct Args {
    #[arg(long, short = 'k', help = "Your bereal.devin.fun api key")]
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct Time {
    ts: i64,
    utc: String,
}

#[derive(Debug, Deserialize)]
struct Moment {
    id: String,
    ts: i64,
    utc: String,
}

#[derive(Deserialize, Debug)]
struct RegionsMoments {
    #[serde(rename = "us-central")]
    us_central: Moment,

    #[serde(rename = "europe-west")]
    europe_west: Moment,

    #[serde(rename = "asia-west")]
    asia_west: Moment,

    #[serde(rename = "asia-east")]
    asia_east: Moment,
}

#[derive(Debug, Deserialize)]
struct LatestMoments {
    regions: RegionsMoments,
    now: Time,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let latest_moments = reqwest::Client::new()
        .get("https://bereal.devin.rest/v1/moments/latest")
        .query(&[("api_key", args.api_key)])
        .send()
        .await?
        .json::<LatestMoments>()
        .await?;

    println!("{:?}", latest_moments);

    Ok(())
}
