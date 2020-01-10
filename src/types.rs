use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Summoner {
    pub account_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MatchHistory {
    pub matches: Vec<MatchData>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MatchData {
    lane: Lane,
    game_id: u64,
    pub champion: u32,
    platform_id: Region,
    timestamp: u64,
    // Rank: 420, Normal: 430, ARAM: 450
    pub queue: u32,
    role: Role,
    season: u32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
enum Lane {
    Top,
    Jungle,
    Mid,
    Bottom,
    None,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
enum Region {
    Kr,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
enum Role {
    Duo,
    DuoCarry,
    DuoSupport,
    Solo,
    None,
}
