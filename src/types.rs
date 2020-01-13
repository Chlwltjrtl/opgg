use serde::Deserialize;
pub use structopt::StructOpt;
use strum_macros::Display;

#[derive(StructOpt)]
pub struct Cli {
    pub summoner_name: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Summoner {
    pub account_id: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MatchHistory {
    pub matches: Vec<Match>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Match {
    pub lane: Lane,
    pub game_id: u64,
    pub champion: u32,
    pub platform_id: Region,
    pub timestamp: i64,
    // Rank: 420, Normal: 430, Random: 450
    pub queue: u32,
    pub role: Role,
    pub season: u32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MatchDetails {
    pub game_id: u64,
    pub participant_identities: Vec<Player>,
    // pub game_type : String,
    // 1~5 player index : 0 , 6~10 player index : 1
    pub teams: Vec<TeamDetails>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Player {
    pub player: PlayerDetails,
    pub participant_id: u32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PlayerDetails {
    pub summoner_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TeamDetails {
    pub bans: Vec<BanChampion>,
    pub win: WinLose,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BanChampion {
    pub pick_turn: u32,
    pub champion_id: u32,
}

#[derive(Display, Debug, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Lane {
    Top,
    Jungle,
    Mid,
    Bottom,
    None,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Region {
    Kr,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Role {
    Duo,
    DuoCarry,
    DuoSupport,
    Solo,
    None,
}

#[derive(Display, Debug, Deserialize)]
pub enum WinLose {
    Win,
    #[serde(rename = "Fail")]
    Lose,
}
