use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, author)]
struct Cli {
    #[arg(long, short = 'k', help = "Your bereal.devin.fun api key")]
    api_key: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Test {
        #[arg(short, long)]
        name: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = bereal::BerealClient::new(&cli.api_key);

    // println!("{}", cli)
}
