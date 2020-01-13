use reqwest::Result;

mod champion_map;
use champion_map::CHAMPION_MAP;

mod types;
use types::{Cli, MatchDetails, MatchHistory, StructOpt, Summoner};

use chrono::{Local, TimeZone};

use prettytable::{cell, row, Table};

// Insert API key here
const API_KEY: &str = "";
const HOST: &str = "https://kr.api.riotgames.com";

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
            "{}/lol/match/v4/matchlists/by-account/{}?endIndex=20", //20 ?
            HOST, summoner.account_id
        ))
        .header("X-Riot-Token", API_KEY)
        .send()?
        .json::<MatchHistory>()?;

    let mut table = Table::new();
    table.add_row(row!["Time", "Game type", "Win Lose", "Lane", "Champion"]);

    for match_data in match_history.matches {
        let game_type = match match_data.queue {
            420 => "Ranked Solo",
            430 => "Normal",
            440 => "Ranked Flex",
            450 => "ARAM",
            _ => "Unknown",
        };

        let match_details = client
            .get(&format!(
                "{}/lol/match/v4/matches/{}",
                HOST, match_data.game_id
            ))
            .header("X-Riot-Token", API_KEY)
            .send()?
            .json::<MatchDetails>()?;

        let mut my_index: usize = 2;
        for participant_identities in match_details.participant_identities {
            if remove_whitespace(&args.summoner_name)
                == remove_whitespace(&participant_identities.player.summoner_name)
            {
                if participant_identities.participant_id <= 5 {
                    my_index = 0;
                    break;
                } else {
                    my_index = 1;
                }
            }
        }
        if my_index == 2 {
            panic!("Win Lose Error");
        }

        table.add_row(row![
            Local
                .timestamp_millis(match_data.timestamp)
                .format("%Y-%m-%d %H:%M"),
            game_type,
            match_details.teams[my_index].win,
            match_data.lane,
            CHAMPION_MAP.get(&match_data.champion).unwrap(),
        ]);
    }
    table.printstd();

    Ok(())
}
fn remove_whitespace(s: &str) -> String {
    s.to_lowercase().split_whitespace().collect()
}
