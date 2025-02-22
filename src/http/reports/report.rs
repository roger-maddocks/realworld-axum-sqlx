use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use chrono::NaiveDate;
use de::from_str;
use itertools::Itertools;
use quick_xml::de;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Pointer};

use crate::http::clients::my_sports_feed::my_sports_feed_client::MySportsFeedProfile;
use crate::http::clients::yahoo::yahoo_auth_client_builder::YahooAuthClientBuilder;
use crate::http::clients::yahoo::yahoo_client::YahooAuthClient;
use crate::http::reports::report::LoadDistribution::{BACK, EQUAL, FRONT};
use crate::http::{ApiContext, Result};
use crate::models::fantasy_team::{FantasyTeam, FantasyTeamReportBounds};
use crate::models::free_agent_report::{FreeAgentReport, LoadDistribution, ReportBounds};
use crate::models::roster::{FantasyWeek, Game, Games, Player, Position, Team};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/reports/overloaded", get(get_overloaded_report))
        .route("/api/reports/free_agents", get(get_free_agents))
        .route("/api/reports/fantasy_team", get(get_fantasy_team))
        .route("/api/reports/collision_report", get(get_collision_report))
}

/// A wrapper type for all requests/responses from these routes.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ReportBody<T> {
    report: T,
}

async fn get_collision_report(
    ctx: State<ApiContext>,
    Json(req): Json<ReportBounds>,
) -> Result<Json<ReportBody<OverloadedReport<'static>>>> {
    let mut week_index = vec![];
    let mut weekly_reports: Vec<OverloadedReport> = vec![];
    let start_week: u64 = req.start_week;
    let team: u64 = req.team_id;
    let report = get_loaded_schedule_report(FantasyWeek::new(start_week).clone()).await;
    let mut index:u64  = 0;

    week_index.push(1);
    weekly_reports.push(report);

    let mut report_base = week_index.iter().zip(weekly_reports.iter());
    let mut current_report = &mut report_base.next().unwrap().1.clone();
    let mut my_team = get_team_roster(team).await?.report;

    current_report.update_four_or_more();
    current_report.update_three_game_teams();

    // for player in my_team
    //     .team.clone()
    //     .team.unwrap()
    //     .roster.unwrap()
    //     .players.unwrap()
    //     .all_players.unwrap()
    // {
    //     for team_schedule in current_report.game_count.iter()
    //     {
    //         let player_org = player.editorial_team_abbr.clone().unwrap();
    //         let team_org = team_schedule.0.abbreviation.to_string();
    //         if player_org == team_org && team_schedule.1 < &3 {
    //             println!("{:?} has a light schedule", player.name.clone().unwrap().full.unwrap());
    //         }
    //         if player_org == team_org && team_schedule.1 == &3 {
    //             println!("{:?} has a normal schedule", player.name.clone().unwrap().full.unwrap());
    //         }
    //         if player_org == team_org && team_schedule.1 > &3 {
    //             println!("{:?} has a heavy schedule", player.name.clone().unwrap().full.unwrap());
    //         }
    //     }
    // }
    for player in my_team
        .yahoo_xml_team.clone()
        .xml_roster.unwrap()
        .roster.unwrap()
        .players.unwrap()
        .all_players.unwrap()
    {
        for team_schedule in current_report.front_heavy_teams.iter()
        {
            let player_org = player.editorial_team_abbr.clone().unwrap();
            let team_org = team_schedule.0.abbreviation.to_string();

            if player_org == team_org {
                println!("{:?} has a front heavy schedule", player.name.clone().unwrap().full.unwrap());
            }
        }
    }



    Ok(Json(ReportBody{ report: current_report.to_owned() }))
}


async fn get_overloaded_report(
    ctx: State<ApiContext>,
    Json(req): Json<ReportBounds>,
) -> Result<Json<ReportBody<OverloadedReport<'static>>>> {

    let mut week_index = vec![];
    let mut weekly_reports: Vec<OverloadedReport> = vec![];

    let start_week: u64 = req.start_week;
    let report = get_loaded_schedule_report(FantasyWeek::new(start_week).clone()).await;

    week_index.push(1);
    weekly_reports.push(report);

    let mut report_base = week_index.iter().zip(weekly_reports.iter());
    let mut current_report = &mut report_base.next();
    let mut final_report: OverloadedReport = current_report.unwrap().1.clone();

    final_report.update_four_or_more();
    final_report.update_three_game_teams();

    Ok(Json(ReportBody{ report: final_report }))
}

async fn get_free_agents(
    ctx: State<ApiContext>,
    Json(req): Json<ReportBounds>,
// ) -> Result<Json<ReportBody<FreeAgentReport<'static>>>> {
    ) -> Result<Json<ReportBody<FreeAgentReport>>> {

    let url = env!["YAHOO_V2_URL"].to_string()
        + "/league/"
        // + "/team/"
        + env!["YAHOO_LEAGUE_KEY"]
        + "/players;count=10;status=A;sort=PTS"; //Available

        // + ".t.7/roster/players";
        // + "/players;count=10;search=nathan";
        // + "/players;count=10;status=T;sort=PTS"; //Taken
        // + "/league/427.l.28172/players;count=2;status=A;sort=PTS";
        // + "/players;player_keys=427.p.5697/stats";
        // + "/players;player_keys=453.p.5697/stats";

    //Needs implementation but pulls real data if appended to env!["YAHOO_LEAGUE_KEY"]
    // + ".t.7/roster/players";
    //get league resource
    // let url = env!["YAHOO_V2_URL"].to_string() + "/users;use_login=1/games;game_keys=nhl/teams";
    // self.yahoo_client.generate_get_request_headers().await;
    // .headers(get_reqwest_headermap(yahoo_client.request_headers).clone())

    let response = get_yahoo_client_response(YahooAuthClientBuilder::new().build().await, url, Client::new()).await;

    println!("our xml guy: {:?}", &response);

    let fa_report = FreeAgentReport::new(from_str(&response).unwrap(), None, None);

    Ok(Json(ReportBody{ report: fa_report }))
}

async fn get_fantasy_team(
    ctx: State<ApiContext>,
    Json(req): Json<FantasyTeamReportBounds>,
) -> Result<Json<ReportBody<FantasyTeam>>> {

    let url = env!["YAHOO_V2_URL"].to_string()
        + "/team/"
        + env!["YAHOO_LEAGUE_KEY"]
        + ".t.7/roster/players";

    let response = get_yahoo_client_response(YahooAuthClientBuilder::new().build().await, url, Client::new()).await;

    println!("our xml guy: {:?}", &response);

    let fantasy_roster = FantasyTeam::new(from_str(&response).unwrap());
    Ok(Json(ReportBody{ report: fantasy_roster }))
}

async fn get_team_roster(team_id: u64) -> Result<ReportBody<FantasyTeam>> {

    let url = env!["YAHOO_V2_URL"].to_string()
        + "/team/"
        + env!["YAHOO_LEAGUE_KEY"]
        + ".t." + &*team_id.to_string() + "/roster/players";

    let response = get_yahoo_client_response(
        YahooAuthClientBuilder::new().build().await,
        url,
        Client::new()
    ).await;

    let fantasy_roster = FantasyTeam::new(from_str(&response).unwrap());

    Ok((ReportBody{ report: fantasy_roster }))
}

async fn get_yahoo_client_response(yahoo_client: YahooAuthClient, url: String, client: Client) -> String {
     client
        .get(url)
        .headers(yahoo_client.request_headers.clone())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .replace('\n', "")
}

pub async fn get_loaded_schedule_report<'a>(this_week: FantasyWeek) -> OverloadedReport<'a> {
    let mut report = OverloadedReport::default();

    for days in this_week.start.iter_days().take(7).enumerate() {
        let mut daily_schedule = get_games_for_day(&days.1).await;
        daily_schedule.index = Some(report.index);

        // TODO: 8/28/24 should be removed if the times are ever sent/parsed correctly
        for game in daily_schedule.games.iter_mut() {
            game.schedule.update_start_time();
        }

        report.daily_games.push(update_incorrect_game_abbreviations(&mut daily_schedule).to_owned());


        let mut report_iter = report
            .daily_games
            .iter()
            .nth(report.index as usize)
            .unwrap()
            .games
            .iter();

        report.home_teams = report_iter
            .to_owned()
            .map(|each_game| each_game.schedule.home_team.to_owned())
            .collect();

        report.away_teams = report_iter
            .to_owned()
            .map(|each_game| each_game.schedule.away_team.to_owned())
            .collect();

        register_games(
            &mut report.game_count,
            &report.home_teams.to_owned(),
            &report.away_teams.to_owned(),
        ).await;

        match report.index {
            x if x < 3 => {
                register_games(
                    &mut report.front_heavy_teams,
                    &report.home_teams,
                    &report.away_teams,
                ).await
            }
            x if x == 3 => {
                register_games(
                    &mut report.front_heavy_teams,
                    &report.home_teams,
                    &report.away_teams,
                )
                    .await;
                register_games(
                    &mut report.back_heavy_teams,
                    &report.home_teams,
                    &report.away_teams,
                ).await;
            }
            x if x > 3 => {
                register_games(
                    &mut report.back_heavy_teams,
                    &report.home_teams,
                    &report.away_teams,
                ).await
            }
            _ => panic!("Error while trying to determine front/back heavy schedules"),
        }
        report.index += 1
    }
    report
}


#[serde_with::serde_as]
#[derive(serde::Serialize, Deserialize, Clone, Debug)]
struct OverloadedReport<'a> {
    pub fantasy_weeks: Cow<'a, Vec<FantasyWeek>>,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub teams_playing_four_or_more: HashMap<Team, LoadDistribution>,
    pub top_free_agents: Vec<Player>,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub game_count: HashMap<Team, i32>,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub front_heavy_teams: HashMap<Team, i32>, //teams with 3 games Monday - Thursday
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub back_heavy_teams: HashMap<Team, i32>,  //teams with 3 games Thursday - Sunday
    pub daily_games: Vec<Games>,
    pub home_teams: Vec<Team>,
    pub away_teams: Vec<Team>,
    pub index: i64,
}

impl<'a> OverloadedReport<'a> {
    pub fn new<'b>(
        fantasy_weeks: Cow<'a, Vec<FantasyWeek>>,
        teams_playing_four_or_more: HashMap<Team, LoadDistribution>,
        top_free_agents: Vec<Player>,
        game_count: HashMap<Team, i32>,
        front_heavy_teams: HashMap<Team, i32>,
        back_heavy_teams: HashMap<Team, i32>,
        daily_games: Vec<Games>,
        home_teams: Vec<Team>,
        away_teams: Vec<Team>,
        index: i64,
    ) -> OverloadedReport<'a> {
        Self {
            fantasy_weeks,
            teams_playing_four_or_more,
            top_free_agents,
            game_count,
            front_heavy_teams,
            back_heavy_teams,
            daily_games,
            home_teams,
            away_teams,
            index,
        }
    }

    pub fn default() -> Self {
        OverloadedReport {
            fantasy_weeks: Default::default(),
            teams_playing_four_or_more: Default::default(),
            top_free_agents: vec![],
            game_count: Default::default(),
            front_heavy_teams: Default::default(),
            back_heavy_teams: Default::default(),
            daily_games: vec![],
            home_teams: vec![],
            away_teams: vec![],
            index: 0,
        }
    }


    pub fn get_top_free_agent_scorers(&self, _position: Position) {
        //query Yahoo league for FA
        //Grab top X (5?) by points scored in the previous week
        //if Position not supplied, just top 5 overall scorers
    }

    pub fn update_four_or_more(&mut self) {
        for (key, value) in self.game_count.iter() {
            if *value >= 4 {
                if self.front_heavy_teams.get_key_value(key) == Some((&key, &3)) {
                    println!("| Team: {} | front heavy schedule |", key.abbreviation);
                    self.teams_playing_four_or_more.insert(key.clone(), FRONT);
                } else if self.back_heavy_teams.get_key_value(key) == Some((&key, &3)) {
                    println!("| Team: {} | back heavy schedule  |", key.abbreviation);
                    self.teams_playing_four_or_more.insert(key.clone(), BACK);
                } else {
                    println!("| Team: {} | distributed schedule |", key.abbreviation);
                    self.teams_playing_four_or_more.insert(key.clone(), EQUAL);
                }

                continue;
            }
        }
    }

    ///update front/back heavy teams by removing teams that have less than 3 games
    pub fn update_three_game_teams(&mut self) {
        self.front_heavy_teams = self.front_heavy_teams.iter()
            .map(|s| (s.0.clone(), *s.1))
            .filter(|x| x.1 > 2)
            .collect();

        self.back_heavy_teams = self.back_heavy_teams.iter()
            .map(|s| (s.0.clone(), *s.1))
            .filter(|x| x.1 > 2)
            .collect();
    }
}

async fn get_games_for_day(date: &NaiveDate) -> Games {

    let mut games_today: Games = reqwest::Client::new()
        .get(MySportsFeedProfile::get_2024_daily_url() + &date.format("%Y%m%d").to_string())
        .basic_auth(env!("MSF_API_KEY"), Some(env!("MSF_PASSWORD")))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    games_today
}

async fn teams_with_three_loaded_games(indexed_report: &mut Option<(&u64, &OverloadedReport<'_>)>) {
    let mut this_indexed_report = indexed_report.unwrap();
    let mut report = this_indexed_report.1;

    get_loaded_teams(
        &mut report.game_count.clone(),
        &mut report.front_heavy_teams.clone(),
        "front loaded",
    )
        .await;

    get_loaded_teams(
        &mut report.game_count.clone(),
        &mut report.back_heavy_teams.clone(),
        "back loaded",
    )
        .await;
}
pub async fn get_loaded_teams(
    game_count: &mut HashMap<Team, i32>,
    loaded_teams: &mut HashMap<Team, i32>,
    description: &str,
) {
    let mut index: i32 = 0;

    for (key, value) in loaded_teams.iter() {
        if *value >= 3 {
            if game_count.get_key_value(key) != Some((&key, &4)) {
                print!("|");
                print!("Team: {} | {} lineup|", key.abbreviation.to_string(), description);
                print!(" ");
                index += 1;
            }
        }
        continue;
    }
    if index < 1 {
        println!("No teams {} this week", description);
    }
    println!();
}

async fn register_games(
    game_count: &mut HashMap<Team, i32>,
    home_teams: &Vec<Team>,
    away_teams: &Vec<Team>,
) -> () {
    update_load_count(game_count, home_teams);
    update_load_count(game_count, away_teams);
}

fn update_load_count(game_count: &mut HashMap<Team, i32>, team_collection: &Vec<Team>) {
    for team in team_collection {
        match game_count.get(&team) {
            Some(count) => {
                game_count.insert(team.to_owned(), count + 1);
            }
            None => {
                game_count.insert(team.to_owned(), 1);
            }
        }
    }

}

fn update_incorrect_game_abbreviations(game: &mut Games) -> &mut Games {

    for mut g in &mut game.games
    {
        if g.schedule.home_team.abbreviation == "FLO" {
            g.schedule.home_team.abbreviation = "FLA".parse().unwrap()
        }

        if g.schedule.away_team.abbreviation == "FLO" {
            g.schedule.away_team.abbreviation = "FLA".parse().unwrap()
        }
    }

    game
}