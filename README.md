# bereal-rs

![bereal crate version](https://img.shields.io/crates/v/bereal?style=for-the-badge)

A library that wraps the bereal.devin.fun API for getting bereal moment data

```rs
use bereal::BerealClient;

#[tokio::main]
async fn main() {
    let client = BerealClient::new("API_KEY");

    println!("{:?}", client.latest_moments().await);
}
```
