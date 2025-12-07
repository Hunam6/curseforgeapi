use crate::definitions::{
    GetCategoriesParams, GetCategoriesResponse, GetGameResponse, GetGamesParams, GetGamesResponse, GetVersionTypesResponse,
    GetVersionsResponse, GetVersionsResponseV1, SearchModsParams, SearchModsResponse,
    CF_URL, CF_V2_URL,
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

    pub async fn get_game(&self, game_id: i32) -> Result<GetGameResponse> {
        let url = format!("{CF_URL}/games/{game_id}");
        self.get(&url, &()).await
    }

    #[deprecated(note = "Use get_versions instead")]
    pub async fn get_versions_v1(&self, game_id: i32) -> Result<GetVersionsResponseV1> {
        let url = format!("{CF_URL}/games/{game_id}/versions");
        self.get(&url, &()).await
    }

    pub async fn get_version_types(&self, game_id: i32) -> Result<GetVersionTypesResponse> {
        let url = format!("{CF_URL}/games/{game_id}/version-types");
        self.get(&url, &()).await
    }

    pub async fn get_versions(&self, game_id: i32) -> Result<GetVersionsResponse> {
        let url = format!("{CF_V2_URL}/games/{game_id}/versions");
        self.get(&url, &()).await
    }

    pub async fn get_categories(
        &self,
        params: &GetCategoriesParams,
    ) -> Result<GetCategoriesResponse> {
        self.get(concatcp!(CF_URL, "/categories"), params).await
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
