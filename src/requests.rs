use crate::definitions::{
    GetGamesParams, GetGamesResponse, SearchModsParams, SearchModsResponse, CF_URL,
};
use crate::CurseForge;
use anyhow::Result;
use const_format::concatcp;
use serde::de::DeserializeOwned;
use serde::Serialize;

impl CurseForge {
    pub async fn get_games(&self, params: &GetGamesParams) -> Result<GetGamesResponse> {
        self.get(concatcp!(CF_URL, "/games"), params).await
    }

    pub async fn search_mods(&self, params: &SearchModsParams) -> Result<SearchModsResponse> {
        self.get(concatcp!(CF_URL, "/mods/search"), params).await
    }

    async fn get<P, R>(&self, url: &str, params: &P) -> Result<R>
    where
        P: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        Ok(self
            .client
            .get(url)
            .query(params)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await?)
    }
}
