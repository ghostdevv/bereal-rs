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
pub struct RegionsMoments {
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
pub struct LatestMoments {
    regions: RegionsMoments,
    now: Time,
}

pub struct BerealClient {
    api_key: String,
}

impl BerealClient {
    pub fn new(api_key: &str) -> BerealClient {
        BerealClient {
            api_key: api_key.to_string(),
        }
    }

    pub async fn latest_moments(&self) -> Result<LatestMoments, reqwest::Error> {
        let latest_moments = reqwest::Client::new()
            .get("https://bereal.devin.rest/v1/moments/latest")
            .query(&[("api_key", &self.api_key)])
            .send()
            .await?
            .json::<LatestMoments>()
            .await?;

        Ok(latest_moments)
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
}
