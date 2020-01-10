use prettytable::{cell, row, Table};
use reqwest::Result;
use structopt::StructOpt;

mod types;

use types::{MatchHistory, Summoner};

#[derive(StructOpt)]
struct Cli {
    summoner_name: String,
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
            "{}/lol/match/v4/matchlists/by-account/{}?endIndex=20",
            HOST, summoner.account_id
        ))
        .header("X-Riot-Token", API_KEY)
        .send()?
        .json::<MatchHistory>()?;

    let mut table = Table::new();
    table.add_row(row!["Game type", "Champion"]);
    for match_data in match_history.matches {
        let game_type = match match_data.queue {
            420 => "Ranked Solo",
            430 => "Normal",
            440 => "Ranked Flex",
            450 => "ARAM",
            _ => "Unknown",
        };
        table.add_row(row![game_type, match_data.champion]);
    }
    table.printstd();

    Ok(())
}
