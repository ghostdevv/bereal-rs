// todo add doc comments

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Time {
    ts: i64,
    utc: String,
}

#[derive(Debug, Deserialize)]
pub struct Moment {
    id: String,
    ts: i64,
    utc: String,
}

#[derive(Deserialize, Debug)]
pub struct Regions<T> {
    #[serde(rename = "us-central")]
    us_central: T,

    #[serde(rename = "europe-west")]
    europe_west: T,

    #[serde(rename = "asia-west")]
    asia_west: T,

    #[serde(rename = "asia-east")]
    asia_east: T,
}

#[derive(Debug, Deserialize)]
pub struct LatestMoments {
    regions: Regions<Moment>,
    now: Time,
}

#[derive(Debug, Deserialize)]
pub struct AllMoments {
    regions: Regions<Vec<Moment>>,
}

pub struct BerealClient {
    api_key: String,
}

pub enum Limit {
    Count(i16),
    Default,
    All,
}

impl BerealClient {
    pub fn new(api_key: &str) -> BerealClient {
        BerealClient {
            api_key: api_key.to_string(),
        }
    }

    pub async fn latest_moments(&self) -> Result<LatestMoments, reqwest::Error> {
        reqwest::Client::new()
            .get("https://bereal.devin.rest/v1/moments/latest")
            .query(&[("api_key", &self.api_key)])
            .send()
            .await?
            .json::<LatestMoments>()
            .await
    }

    pub async fn all_moments(&self, limit: Limit) -> Result<AllMoments, reqwest::Error> {
        let limit_param = match limit {
            Limit::Count(num) => num.to_string(),
            Limit::All => String::from("NONE"),
            Limit::Default => String::from("90"),
        };

        reqwest::Client::new()
            .get("https://bereal.devin.rest/v1/moments/all")
            .query(&[("api_key", &self.api_key), ("limit", &limit_param)])
            .send()
            .await?
            .json::<AllMoments>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn create_client() -> BerealClient {
        match env::var("BR_API_KEY") {
            Err(_) => panic!("Missing BR_API_KEY environment variable"),
            Ok(key) => BerealClient::new(&key),
        }
    }

    #[tokio::test]
    async fn latest_moments() {
        if let Err(e) = create_client().latest_moments().await {
            panic!("{:?}", e);
        }
    }

    #[tokio::test]
    async fn all_moments() {
        match create_client().all_moments(Limit::Count(1)).await {
            Err(e) => panic!("{:?}", e),
            Ok(moments) => {
                if moments.regions.europe_west.len() != 1 {
                    panic!("Limit did not apply correctly")
                }
            }
        };
    }

    #[tokio::test]
    async fn all_moments_limit_default() {
        match create_client().all_moments(Limit::Default).await {
            Err(e) => panic!("{:?}", e),
            Ok(moments) => {
                let count = moments.regions.europe_west.len();

                if count != 90 {
                    panic!("Limit did not apply correctly, found a count of {}", count)
                }
            }
        };
    }

    #[tokio::test]
    async fn all_moments_limit_all() {
        match create_client().all_moments(Limit::All).await {
            Err(e) => panic!("{:?}", e),
            Ok(moments) => {
                let count = moments.regions.europe_west.len();

                if count <= 90 {
                    panic!("Limit did not apply correctly, found a count of {}", count)
                }
            }
        };
    }
}
