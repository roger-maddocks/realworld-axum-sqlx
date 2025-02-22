// // use crate::models::player::Position;
// use chrono::NaiveDate;
// use serde::Deserialize;
// use crate::models::roster::Position;
//
// #[derive(Default, serde::Serialize, Deserialize, Debug, Clone)]
// pub struct YahooPlayers {
//     players: Option<Vec<YahooPlayer>>,
// }
//
// #[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct YahooPlayer {
//     player_key: Option<String>,
//     player_id: Option<u64>,
//     name: Option<Name>,
//     url: Option<String>,
//     editorial_player_key: Option<String>,
//     editorial_team_key: Option<String>,
//     editorial_team_full_name: Option<String>,
//     editorial_team_abbr: Option<String>,
//     editorial_team_url: Option<String>,
//     is_keeper: Option<Keeper>,
//     uniform_number: Option<u64>,
//     display_position: Option<String>,
//     headshot: Option<Headshot>,
//     is_undroppable: Option<bool>,
//     position_type: Option<String>,
//     primary_position: Option<String>,
//     eligible_positions: Option<Vec<Option<Position>>>,
//     eligible_positions_to_add: Option<Vec<Option<Position>>>,
//     has_player_notes: Option<bool>,
//     player_notes_last_timestamp: Option<u64>,
// }
//
// #[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Keeper {
//     status: Option<String>,
//     cost: Option<String>,
//     kept: Option<String>,
// }
// #[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Name {
//     full: Option<String>,
//     first: Option<String>,
//     last: Option<String>,
//     ascii_first: Option<String>,
//     ascii_last: Option<String>,
// }
// #[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Headshot {
//     url: Option<String>,
//     size: Option<String>,
// }
// #[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct SelectedPosition {
//     coverage_type: Option<String>,
//     date: Option<NaiveDate>,
//     is_flex: Option<bool>,
// }
