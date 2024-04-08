use crate::{BerealClient, Limit, Region};
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
