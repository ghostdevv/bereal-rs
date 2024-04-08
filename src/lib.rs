// todo add doc comments

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

#[derive(Debug, Deserialize)]
pub struct Time {
    pub ts: i64,
    pub utc: String,
}

#[derive(Debug, Deserialize)]
pub struct Moment {
    pub id: String,
    pub ts: i64,
    pub utc: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Regions<T> {
    #[serde(rename = "us-central")]
    pub us_central: T,

    #[serde(rename = "europe-west")]
    pub europe_west: T,

    #[serde(rename = "asia-west")]
    pub asia_west: T,

    #[serde(rename = "asia-east")]
    pub asia_east: T,
}

#[derive(Debug, Deserialize)]
pub struct LatestMoments {
    pub regions: Regions<Moment>,
    pub now: Time,
}

#[derive(Debug, Deserialize)]
pub struct AllMoments {
    pub regions: Regions<Vec<Moment>>,
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
    pub unix: i64,
    pub timestamp: String,
}

// #[derive(Debug, Deserialize)]
// pub struct LookupResultTimeLocalized {
//     // pub timezone: String,
//     pub timestamp: String,
// }

#[derive(Debug, Deserialize)]
pub struct LookupResult {
    pub region: Region,

    pub id: String,

    #[serde(rename = "UTC")]
    pub utc: LookupResultTimeUTC,
    // todo this seems broken in api
    // pub localized: LookupResultTimeLocalized,
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
