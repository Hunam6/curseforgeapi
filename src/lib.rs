use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};

pub mod definitions;
pub mod requests;

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
    use crate::definitions::SearchModsParams;

    #[tokio::test]
    async fn manual_testing() -> Result<()> {
        dotenvy::dotenv().ok();
        let api_key = std::env::var("CURSEFORGE_API_KEY")?;

        let cf = CurseForge::new(&api_key)?;

        // Search for Minecraft mods
        let mods = cf
            .search_mods(&SearchModsParams {
                game_id: 432, // Minecraft
                search_filter: Some("Complementary Shaders".to_string()),
                sort_field: Some(definitions::ModsSearchSortField::TotalDownloads),
                sort_order: Some(definitions::SortOrder::Desc),
                ..Default::default()
            })
            .await?;

        for m in mods.data {
            println!("{} - {} downloads", m.name, m.download_count);
        }

        Ok(())
    }
}
