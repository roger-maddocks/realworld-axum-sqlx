use std::ops::Sub;
use chrono::{Days, Duration, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use crate::http::Error;
use crate::models::roster::NhlFranchise::ColoradoAvalanche;
use crate::models::roster::Position::C;


/// Schedule Scaffolding
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct FantasyWeek {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl FantasyWeek {
    // pub fn new(beginning_week: u64, ending_week: u64) -> FantasyWeek {
    pub fn new(week: u64) -> FantasyWeek {
        FantasyWeek {
            start: NaiveDate::from_ymd_opt(2024, 9, 30)
                .unwrap()
                .checked_add_days(Days::new(week * 7))
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2024, 10, 6)
                .unwrap()
                .checked_add_days(Days::new(week * 7))
                .unwrap(),
            // start: NaiveDate::from_ymd_opt(2023, 10, 2)
            //     .unwrap()
            //     .checked_add_days(Days::new(week * 7))
            //     .unwrap(),
            // end: NaiveDate::from_ymd_opt(2023, 10, 8)
            //     .unwrap()
            //     .checked_add_days(Days::new(week * 7))
            //     .unwrap(),
        }
    }

    pub fn get_season_start_week(&self) -> FantasyWeek {
        FantasyWeek::new(1)
    }

    pub fn get_season_end_week(&self) -> FantasyWeek {
        FantasyWeek::new(26)
    }

    pub fn get_week(&self, week_number: u64) -> FantasyWeek {
        FantasyWeek::new(week_number)
    }

    pub fn get_week_range(
        &self,
        first_week_of_range: u64,
        last_week_of_range: u64,
    ) -> Vec<FantasyWeek> {
        let mut all_weeks = vec![];
        for week in first_week_of_range..last_week_of_range + 1 {
            all_weeks.push(FantasyWeek::new(week))
        }
        all_weeks
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub abbreviation: String,
    #[serde(rename = "id")]
    pub msf_id: u16, //NhlFranchise
}

impl Team {
    pub fn new(abbreviation: String, msf_id: u16) -> Team {
        Self {
            abbreviation,
            msf_id,
        }
    }
    pub fn default() -> Team {
        Self {
            abbreviation: "".to_string(),
            msf_id: 0,
        }
    }
}

impl Clone for Team {
    fn clone(&self) -> Self {
        Team {
            abbreviation: self.abbreviation.clone(),
            msf_id: self.msf_id.clone(), // franchise: Default::default(),
        }
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Team) -> bool {
        self.abbreviation == other.abbreviation
    }
}



/// Game Scaffolding
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub games: Vec<Game>,
    pub index: Option<i64>,
}

#[derive(Debug, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub schedule: Schedule,
    pub score: Score,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            schedule: self.schedule.clone(),
            score: self.score.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub away_score_total: Option<i32>,
    pub home_score_total: Option<i32>,
}

// impl Clone for Score {
//     fn clone(&self) -> Self {
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub away_team: Team,
    pub home_team: Team,
    pub start_time: String,
}

impl Schedule {
    pub fn update_start_time(&mut self) {
        self.start_time = NaiveDateTime::parse_from_str(&*self.start_time.clone(), "%Y-%m-%dT%H:%M:%S %Z").unwrap().sub(Duration::hours(7)).to_string();
    }

}

impl Clone for Schedule {
    fn clone(&self) -> Self {
        Schedule {
            away_team: self.away_team.clone(),
            home_team: self.home_team.clone(),
            start_time: self.start_time.clone(),
        }
    }
}
impl IntoIterator for Game {
    type Item = ();
    type IntoIter = GameIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct GameIntoIterator {
    game: Game,
    index: usize,
}

impl Game {}

impl Games {
    pub fn new() -> Games {
        Games { games: vec![], index: Some(0) }
    }
    pub fn get_weekly_games() -> std::result::Result<Games, Error> {
        Ok(Games { games: vec![], index: Some(0) })
    }

    pub fn get_daily_games(&self, day: NaiveDate) -> std::result::Result<Games, Error> {
        if day.leap_year() {}

        Ok(Games { games: vec![], index: Some(0) })
    }
}

impl Iterator for GameIntoIterator {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Player Scaffolding
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: Option<Name>,
    pub position: Vec<Position>,
    pub prioritize: bool,
    pub franchise: NhlFranchise,
    pub team: Team,
    pub yahoo_team_key: String,
}

impl Player {
    pub fn new(
        first_name: String,
        last_name: String,
        position: Vec<Position>,
        prioritize: bool,
        franchise: NhlFranchise,
        team: Team,
        yahoo_team_key: String,
    ) -> Player {
        Self {
            name: Option::from(Name::new(first_name, last_name)),
            position,
            prioritize,
            franchise,
            team,
            yahoo_team_key,
        }
    }
    pub fn default() -> Player {
        Self {
            name: Default::default(),
            position: vec![C],
            prioritize: false,
            franchise: ColoradoAvalanche,
            team: Default::default(),
            yahoo_team_key: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Position {
    #[default]
    C,
    LW,
    RW,
    D,
    G,
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq)]
pub enum NhlFranchise {
    AnaheimDucks,
    BostonBruins,
    BuffaloSabres,
    CalgaryFlames,
    CarolinaHurricanes,
    ChicagoBlackhawks,
    ColoradoAvalanche,
    ColumbusBlueJackets,
    DallasStars,
    DetroitRedWings,
    EdmontonOilers,
    FloridaPanthers,
    LosAngelesKings,
    MinnesotaWild,
    MontrealCanadiens,
    NashvillePredators,
    NewJerseyDevils,
    NewYorkIslanders,
    NewYorkRangers,
    OttawaSenators,
    PhiladelphiaFlyers,
    PittsburghPenguins,
    SanJoseSharks,
    SeattleKraken,
    StLouisBlues,
    TampaBayLightning,
    TorontoMapleLeafs,
    UtahHockeyClub,
    VancouverCanucks,
    VegasGoldenKnights,
    WashingtonCapitals,
    WinnipegJets,
}

impl Default for NhlFranchise {
    fn default() -> Self {
        ColoradoAvalanche
    }
}

impl PartialEq for NhlFranchise {
    fn eq(&self, other: &NhlFranchise) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Name {
    #[serde(rename = "full")]
    pub full_name: String,
    #[serde(rename = "first")]
    pub first_name: String,
    #[serde(rename = "last")]
    pub last_name: String,
    pub ascii_first: String,
    pub ascii_last: String,
}
impl Name {
    pub fn new(first: String, last: String) -> Name {
        Name {
            full_name: first.clone() + &last.clone(),
            first_name: first,
            last_name: last,
            ascii_first: "".to_string(),
            ascii_last: "".to_string(),
        }
    }
}
