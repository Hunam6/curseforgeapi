use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};

mod definitions;
mod requests;

pub struct CurseForge {
    client: reqwest::Client,
}

impl CurseForge {
    pub fn new(api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(api_key)
                .map_err(|_| anyhow::anyhow!("Invalid CURSE_FORGE_API_KEY value"))?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| anyhow::anyhow!("Unable to create CurseForge client"))?;

        Ok(Self { client })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::GetGamesParams;

    #[tokio::test]
    async fn manual_testing() {
        dotenvy::dotenv().ok();
        let cf = CurseForge::new(std::env::var("CURSE_FORGE_API_KEY").unwrap().as_str()).unwrap();

        dbg!(
            cf.get_games(&GetGamesParams {
                ..Default::default()
            })
            .await
            .unwrap()
        );

        ()
    }
}
