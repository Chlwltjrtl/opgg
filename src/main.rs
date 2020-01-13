use chrono::{Local, TimeZone};
use prettytable::{cell, row, Table};
use reqwest::{blocking::Client, Result, Url};
use serde::de::DeserializeOwned;

mod champion_map;
mod types;

use champion_map::CHAMPION_MAP;
use types::{Cli, MatchDetails, MatchHistory, StructOpt, Summoner};

// Insert API key here
const API_KEY: &str = "";
const HOST: &str = "https://kr.api.riotgames.com/";

fn main() -> Result<()> {
    let args = Cli::from_args();
    let client = reqwest::blocking::Client::new();

    let summoner: Summoner = riot_get(
        &client,
        &format!("/lol/summoner/v4/summoners/by-name/{}", args.summoner_name),
    )?;

    let match_history: MatchHistory = riot_get(
        &client,
        &format!(
            "/lol/match/v4/matchlists/by-account/{}?endIndex={}",
            summoner.account_id, args.match_history_len
        ),
    )?;

    let mut table = Table::new();
    table.add_row(row![
        "Id",
        "Time",
        "Game type",
        "Win Lose",
        "Lane",
        "Champion"
    ]);

    for (i, match_data) in match_history.matches.iter().enumerate() {
        let game_type = match match_data.queue {
            420 => "Ranked Solo",
            430 => "Normal",
            440 => "Ranked Flex",
            450 => "ARAM",
            _ => "Unknown",
        };

        let match_details: MatchDetails = riot_get(
            &client,
            &format!("/lol/match/v4/matches/{}", match_data.game_id),
        )?;
        let my_summoner_name = normalize_summoner_name(&args.summoner_name);
        let my_participant_id = match_details
            .participant_identities
            .into_iter()
            .find(|participant| {
                normalize_summoner_name(&participant.player.summoner_name) == my_summoner_name
            })
            .unwrap()
            .participant_id;
        let team_id = match my_participant_id {
            1..=5 => 0,
            6..=10 => 1,
            _ => panic!("Participant id error"),
        };

        table.add_row(row![
            i,
            Local
                .timestamp_millis(match_data.timestamp)
                .format("%Y-%m-%d %H:%M"),
            game_type,
            match_details.teams[team_id].win,
            match_data.lane,
            CHAMPION_MAP.get(&match_data.champion).unwrap(),
        ]);
    }
    table.printstd();

    Ok(())
}

fn normalize_summoner_name(s: &str) -> String {
    s.to_lowercase().split_whitespace().collect()
}
fn riot_get<T>(client: &Client, path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let url = Url::parse(HOST).unwrap().join(path).unwrap();
    client
        .get(url)
        .header("X-Riot-Token", API_KEY)
        .send()?
        .json::<T>()
}
