use crate::definitions::{
    GetCategoriesParams, GetCategoriesResponse, GetFeaturedModsRequestBody, GetFeaturedModsResponse, GetFilesResponse,
    GetFingerprintMatchesRequestBody, GetFingerprintMatchesResponse, GetFingerprintsFuzzyMatchesResponse,
    GetFuzzyMatchesRequestBody, GetGameResponse, GetGamesParams,
    GetGamesResponse, GetMinecraftModLoaderResponse, GetMinecraftModLoadersParams, GetMinecraftModLoadersResponse,
    GetMinecraftVersionResponse, GetMinecraftVersionsParams, GetMinecraftVersionsResponse,
    GetModDescriptionParams, GetModFileResponse, GetModFilesParams,
    GetModFilesRequestBody, GetModFilesResponse, GetModResponse, GetModsRequestBody,
    GetModsResponse, GetVersionTypesResponse, GetVersionsResponse, GetVersionsResponseV1,
    SearchModsParams, SearchModsResponse, StringResponse, CF_URL,
    CF_V2_URL,
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

    pub async fn get_mod(&self, mod_id: i32) -> Result<GetModResponse> {
        let url = format!("{CF_URL}/mods/{mod_id}");
        self.get(&url, &()).await
    }

    pub async fn get_mods(&self, body: &GetModsRequestBody) -> Result<GetModsResponse> {
        self.post(concatcp!(CF_URL, "/mods"), body).await
    }

    pub async fn get_featured_mods(
        &self,
        body: &GetFeaturedModsRequestBody,
    ) -> Result<GetFeaturedModsResponse> {
        self.post(concatcp!(CF_URL, "/mods/featured"), body).await
    }

    pub async fn get_mod_description(
        &self,
        mod_id: i32,
        params: &GetModDescriptionParams,
    ) -> Result<StringResponse> {
        let url = format!("{CF_URL}/mods/{mod_id}/description");
        self.get(&url, &params).await
    }

    pub async fn get_mod_file(&self, mod_id: i32, file_id: i32) -> Result<GetModFileResponse> {
        let url = format!("{CF_URL}/mods/{mod_id}/files/{file_id}");
        self.get(&url, &()).await
    }

    pub async fn get_mod_files(
        &self,
        mod_id: i32,
        params: &GetModFilesParams,
    ) -> Result<GetModFilesResponse> {
        let url = format!("{CF_URL}/mods/{mod_id}/files");
        self.get(&url, &params).await
    }

    pub async fn get_files(&self, body: &GetModFilesRequestBody) -> Result<GetFilesResponse> {
        self.post(concatcp!(CF_URL, "/mods/files"), body).await
    }

    pub async fn get_mod_files_changelog(
        &self,
        mod_id: i32,
        file_id: i32,
    ) -> Result<StringResponse> {
        let url = format!("{CF_URL}/mods/{mod_id}/files/{file_id}/changelog");
        self.get(&url, &()).await
    }

    pub async fn get_mod_file_download_url(
        &self,
        mod_id: i32,
        file_id: i32,
    ) -> Result<StringResponse> {
        let url = format!("{CF_URL}/mods/{mod_id}/files/{file_id}/download-url");
        self.get(&url, &()).await
    }

    pub async fn get_fingerprints_matches_by_game_id(
        &self,
        game_id: i32,
        body: &GetFingerprintMatchesRequestBody,
    ) -> Result<GetFingerprintMatchesResponse> {
        let url = format!("{CF_URL}/fingerprints/{game_id}");
        self.post(&url, body).await
    }

    pub async fn get_fingerprints_matches(
        &self,
        body: &GetFingerprintMatchesRequestBody,
    ) -> Result<GetFingerprintMatchesResponse> {
        self.post(concatcp!(CF_URL, "/fingerprints"), body).await
    }

    pub async fn get_fingerprints_fuzzy_matches_by_game_id(
        &self,
        game_id: i32,
        body: &GetFuzzyMatchesRequestBody,
    ) -> Result<GetFingerprintsFuzzyMatchesResponse> {
        let url = format!("{CF_URL}/fingerprints/fuzzy/{game_id}");
        self.post(&url, body).await
    }

    pub async fn get_fingerprints_fuzzy_matches(
        &self,
        body: &GetFuzzyMatchesRequestBody,
    ) -> Result<GetFingerprintsFuzzyMatchesResponse> {
        self.post(concatcp!(CF_URL, "/fingerprints/fuzzy"), body)
            .await
    }

    pub async fn get_minecraft_versions(
        &self,
        params: &GetMinecraftVersionsParams,
    ) -> Result<GetMinecraftVersionsResponse> {
        self.get(concatcp!(CF_URL, "/minecraft/versions"), params)
            .await
    }

    pub async fn get_minecraft_version(
        &self,
        version: &str,
    ) -> Result<GetMinecraftVersionResponse> {
        let url = format!("{CF_URL}/minecraft/versions/{version}");
        self.get(&url, &()).await
    }

    pub async fn get_minecraft_mod_loaders(
        &self,
        params: &GetMinecraftModLoadersParams,
    ) -> Result<GetMinecraftModLoadersResponse> {
        self.get(concatcp!(CF_URL, "/minecraft/modloader"), params)
            .await
    }

    pub async fn get_minecraft_mod_loader(
        &self,
        mod_loader: &str,
    ) -> Result<GetMinecraftModLoaderResponse> {
        let url = format!("{CF_URL}/minecraft/modloader/{mod_loader}");
        self.get(&url, &()).await
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

    async fn post<B, R>(&self, url: &str, body: &B) -> Result<R>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        Ok(self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?
            .json::<R>()
            .await?)
    }
}
