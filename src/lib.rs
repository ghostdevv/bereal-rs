// todo add doc comments

use serde::{Deserialize, Serialize};

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
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Region {
    #[serde(rename = "us-central")]
    USCentral,

    #[serde(rename = "europe-west")]
    EuropeWest,

    #[serde(rename = "asia-west")]
    AsiaEast,

    #[serde(rename = "asia-east")]
    AsiaWest,
}

impl Region {
    pub fn to_string(&self) -> String {
        match self {
            Region::USCentral => String::from("us-central"),
            Region::EuropeWest => String::from("europe-west"),
            Region::AsiaEast => String::from("asia-east"),
            Region::AsiaWest => String::from("asia-east"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LookupResultTimeUTC {
    unix: i64,
    timestamp: String,
}

// #[derive(Debug, Deserialize)]
// pub struct LookupResultTimeLocalized {
//     // timezone: String,
//     timestamp: String,
// }

#[derive(Debug, Deserialize)]
pub struct LookupResult {
    region: Region,

    id: String,

    #[serde(rename = "UTC")]
    utc: LookupResultTimeUTC,
    // todo this seems broken in api
    // localized: LookupResultTimeLocalized,
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

    pub async fn all_moments(&self, limit: &Limit) -> Result<AllMoments, reqwest::Error> {
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

    pub async fn lookup(
        &self,
        time_zone: &str,
        date: &str,
        region: &Region,
    ) -> Result<LookupResult, reqwest::Error> {
        reqwest::Client::new()
            .get("https://bereal.devin.rest/v1/moments/lookup")
            .query(&[
                ("api_key", &self.api_key),
                ("time_zone", &time_zone.to_string()),
                ("date", &date.to_string()),
                ("region", &region.to_string()),
            ])
            .send()
            .await?
            .json::<LookupResult>()
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
    async fn lookup() {
        let result = create_client()
            .lookup("Europe/London", "2024-04-07", &Region::EuropeWest)
            .await;

        match result {
            Err(e) => panic!("{:?}", e),
            Ok(result) => {
                assert_eq!(result.region.to_string(), Region::EuropeWest.to_string());
                assert_eq!(result.utc.timestamp, "2024-04-07 11:43:05");
            }
        };
    }

    #[tokio::test]
    async fn all_moments() {
        match create_client().all_moments(&Limit::Count(1)).await {
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
        match create_client().all_moments(&Limit::Default).await {
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
        match create_client().all_moments(&Limit::All).await {
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
