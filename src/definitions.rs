use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

pub const CF_URL: &str = "https://api.curseforge.com/v1";
pub const CF_V2_URL: &str = "https://api.curseforge.com/v2";

// ============================================================================
// ENUMS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CoreStatus {
    Draft = 1,
    Test = 2,
    PendingReview = 3,
    Rejected = 4,
    Approved = 5,
    Live = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CoreApiStatus {
    Private = 1,
    Public = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GameVersionStatus {
    Approved = 1,
    Deleted = 2,
    New = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GameVersionTypeStatus {
    Normal = 1,
    Deleted = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FileStatus {
    Processing = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
    FailedPublishing = 15,
    Cooking = 16,
    Cooked = 17,
    UnderManualReview = 18,
    ScanningForMalware = 19,
    ProcessingFile = 20,
    PendingRelease = 21,
    ReadyForCooking = 22,
    PostProcessing = 23,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FileRelationType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HashAlgo {
    Sha1 = 1,
    Md5 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModLoaderInstallMethod {
    ForgeInstaller = 1,
    ForgeJarInstall = 2,
    ForgeInstallerV2 = 3,
    FabricInstaller = 4,
    QuiltInstaller = 5,
    NeoForgeInstaller = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModsSearchSortField {
    Featured = 1,
    Popularity = 2,
    LastUpdated = 3,
    Name = 4,
    Author = 5,
    TotalDownloads = 6,
    Category = 7,
    GameVersion = 8,
    EarlyAccess = 9,
    FeaturedReleased = 10,
    ReleasedDate = 11,
    Rating = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

// ============================================================================
// CORE STRUCTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub index: i32,
    pub page_size: i32,
    pub result_count: i32,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameAssets {
    pub icon_url: Option<String>,
    pub tile_url: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub date_modified: DateTime<Utc>,
    pub assets: GameAssets,
    pub status: CoreStatus,
    pub api_status: CoreApiStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameVersion {
    pub id: i32,
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameVersionsByTypeV1 {
    #[serde(rename = "type")]
    pub version_type: i32,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameVersionsByType {
    #[serde(rename = "type")]
    pub version_type: i32,
    pub versions: Vec<GameVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameVersionType {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub is_syncable: bool,
    pub status: GameVersionTypeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub date_modified: DateTime<Utc>,
    pub is_class: Option<bool>,
    pub class_id: Option<i32>,
    pub parent_category_id: Option<i32>,
    pub display_index: Option<i32>,
}

// ============================================================================
// MOD STRUCTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModLinks {
    pub website_url: Option<String>,
    pub wiki_url: Option<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAuthor {
    pub id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAsset {
    pub id: i32,
    pub mod_id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    pub game_version_name: String,
    pub game_version_padded: String,
    pub game_version: String,
    pub game_version_release_date: DateTime<Utc>,
    pub game_version_type_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDependency {
    pub mod_id: i32,
    pub relation_type: FileRelationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileHash {
    pub value: String,
    pub algo: HashAlgo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileModule {
    pub name: String,
    pub fingerprint: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileIndex {
    pub game_version: String,
    pub file_id: i32,
    pub filename: String,
    pub release_type: FileReleaseType,
    pub game_version_type_id: Option<i32>,
    pub mod_loader: Option<ModLoaderType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: i32,
    pub game_id: i32,
    pub mod_id: i32,
    pub is_available: bool,
    pub display_name: Option<String>,
    pub file_name: Option<String>,
    pub release_type: FileReleaseType,
    pub file_status: FileStatus,
    pub hashes: Vec<FileHash>,
    pub file_date: DateTime<Utc>,
    pub file_length: i64,
    pub download_count: i64,
    pub file_size_on_disk: Option<i64>,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
    pub sortable_game_versions: Vec<SortableGameVersion>,
    pub dependencies: Vec<FileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<i32>,
    pub alternate_file_id: Option<i32>,
    pub is_server_pack: Option<bool>,
    pub server_pack_file_id: Option<i32>,
    pub is_early_access_content: Option<bool>,
    pub early_access_end_date: Option<DateTime<Utc>>,
    pub file_fingerprint: i64,
    pub modules: Vec<FileModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub links: ModLinks,
    pub summary: Option<String>,
    pub status: ModStatus,
    pub download_count: i64,
    pub is_featured: bool,
    pub primary_category_id: i32,
    pub categories: Vec<Category>,
    pub class_id: Option<i32>,
    pub authors: Vec<ModAuthor>,
    pub logo: Option<ModAsset>,
    pub screenshots: Vec<ModAsset>,
    pub main_file_id: i32,
    pub latest_files: Vec<File>,
    pub latest_files_indexes: Vec<FileIndex>,
    pub latest_early_access_files_indexes: Vec<FileIndex>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub date_released: DateTime<Utc>,
    pub allow_mod_distribution: Option<bool>,
    pub game_popularity_rank: i32,
    pub is_available: bool,
    pub thumbs_up_count: i32,
    pub rating: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedModsResponse {
    pub featured: Vec<Mod>,
    pub popular: Vec<Mod>,
    pub recently_updated: Vec<Mod>,
}

// ============================================================================
// FINGERPRINT STRUCTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderFingerprint {
    pub foldername: String,
    pub fingerprints: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintMatch {
    pub id: i32,
    pub file: File,
    pub latest_files: Vec<File>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintFuzzyMatch {
    pub id: i32,
    pub file: File,
    pub latest_files: Vec<File>,
    pub fingerprints: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintsMatchesResult {
    pub is_cache_built: bool,
    pub exact_matches: Vec<FingerprintMatch>,
    pub exact_fingerprints: Vec<i64>,
    pub partial_matches: Vec<FingerprintMatch>,
    pub partial_match_fingerprints: HashMap<String, Vec<i64>>,
    pub installed_fingerprints: Vec<i64>,
    pub unmatched_fingerprints: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintFuzzyMatchResult {
    pub fuzzy_matches: Vec<FingerprintFuzzyMatch>,
}

// ============================================================================
// MINECRAFT STRUCTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftGameVersion {
    pub id: i32,
    pub game_version_id: i32,
    pub version_string: String,
    pub jar_download_url: String,
    pub json_download_url: String,
    pub approved: bool,
    pub date_modified: DateTime<Utc>,
    pub game_version_type_id: i32,
    pub game_version_status: GameVersionStatus,
    pub game_version_type_status: GameVersionTypeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftModLoaderIndex {
    pub name: String,
    pub game_version: String,
    pub latest: bool,
    pub recommended: bool,
    pub date_modified: DateTime<Utc>,
    #[serde(rename = "type")]
    pub loader_type: ModLoaderType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftModLoaderVersion {
    pub id: i32,
    pub game_version_id: i32,
    pub minecraft_game_version_id: i32,
    pub forge_version: String,
    pub name: String,
    #[serde(rename = "type")]
    pub loader_type: ModLoaderType,
    pub download_url: String,
    pub filename: String,
    pub install_method: ModLoaderInstallMethod,
    pub latest: bool,
    pub recommended: bool,
    pub approved: bool,
    pub date_modified: DateTime<Utc>,
    pub maven_version_string: String,
    pub version_json: String,
    pub libraries_install_location: String,
    pub minecraft_version: String,
    pub additional_files_json: String,
    pub mod_loader_game_version_id: i32,
    pub mod_loader_game_version_type_id: i32,
    pub mod_loader_game_version_status: GameVersionStatus,
    pub mod_loader_game_version_type_status: GameVersionTypeStatus,
    pub mc_game_version_id: i32,
    pub mc_game_version_type_id: i32,
    pub mc_game_version_status: GameVersionStatus,
    pub mc_game_version_type_status: GameVersionTypeStatus,
    pub install_profile_json: String,
}

// ============================================================================
// QUERY PARAMETER STRUCTS
// ============================================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGamesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoriesParams {
    pub game_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchModsParams {
    pub game_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i32>,
    /// Filter by a list of category ids (overrides categoryId). Format: [1,2,3...]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_version: Option<String>,
    /// Filter by a list of game version strings (overrides gameVersion). Format: ["1.19.1","1.19.2"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_versions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ModsSearchSortField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_loader_type: Option<ModLoaderType>,
    /// Filter by a list of mod loader types (overrides modLoaderType). Format: [Forge,Fabric]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_loader_types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_version_type_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_author_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModFilesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_loader_type: Option<ModLoaderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_version_type_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMinecraftVersionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_descending: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMinecraftModLoadersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
}

// ============================================================================
// REQUEST BODY STRUCTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeaturedModsRequestBody {
    pub game_id: i32,
    pub excluded_mod_ids: Vec<i32>,
    pub game_version_type_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFingerprintMatchesRequestBody {
    pub fingerprints: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFuzzyMatchesRequestBody {
    pub game_id: i32,
    pub fingerprints: Vec<FolderFingerprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModFilesRequestBody {
    pub file_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModsByIdsListRequestBody {
    pub mod_ids: Vec<i32>,
    pub filter_pc_only: Option<bool>,
}

// ============================================================================
// API RESPONSE STRUCTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringResponse {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponseOfListOfMinecraftGameVersion {
    pub data: Vec<MinecraftGameVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponseOfListOfMinecraftModLoaderIndex {
    pub data: Vec<MinecraftModLoaderIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponseOfMinecraftGameVersion {
    pub data: MinecraftGameVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponseOfMinecraftModLoaderVersion {
    pub data: MinecraftModLoaderVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCategoriesResponse {
    pub data: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGameResponse {
    pub data: Game,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGamesResponse {
    pub data: Vec<Game>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionTypesResponse {
    pub data: Vec<GameVersionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionsResponseV1 {
    pub data: Vec<GameVersionsByTypeV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionsResponse {
    pub data: Vec<GameVersionsByType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModResponse {
    pub data: Mod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModsResponse {
    pub data: Vec<Mod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchModsResponse {
    pub data: Vec<Mod>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeaturedModsResponse {
    pub data: FeaturedModsResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModFileResponse {
    pub data: File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModFilesResponse {
    pub data: Vec<File>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilesResponse {
    pub data: Vec<File>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFingerprintMatchesResponse {
    pub data: FingerprintsMatchesResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFingerprintsFuzzyMatchesResponse {
    pub data: FingerprintFuzzyMatchResult,
}
