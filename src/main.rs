use reqwest::Result;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    summoner_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Summoner {
    account_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct MatchHistory {
    matches: Vec<Match>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Match {
    lane: Lane,
    game_id: u64,
    champion: u32,
    platform_id: Region,
    timestamp: u64,
    // Rank: 420, Normal: 430, Random: 450
    queue: u32,
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

const HOST: &str = "https://kr.api.riotgames.com";
// Insert API key here
const API_KEY: &str = "";

fn main() -> Result<()> {
    let args = Cli::from_args();

    let client = reqwest::blocking::Client::new();
    let summoner = client
        .get(&format!(
            "{}/lol/summoner/v4/summoners/by-name/{}",
            HOST, args.summoner_name
        ))
        .header("X-Riot-Token", API_KEY)
        .send()?
        .json::<Summoner>()?;
    let match_history = client
        .get(&format!(
            "{}/lol/match/v4/matchlists/by-account/{}",
            HOST, summoner.account_id
        ))
        .header("X-Riot-Token", API_KEY)
        .send()?
        .json::<MatchHistory>()?;
    println!("{:?}", match_history);

    Ok(())
}
