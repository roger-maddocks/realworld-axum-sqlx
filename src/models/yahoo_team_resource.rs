use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename = "fantasy_content")]
pub struct YahooXmlTeamResource {
    #[serde(rename = "team")]
    pub xml_roster: Option<XmlRoster>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "team")]
pub struct XmlRoster {
    #[serde(rename = "roster")]
    pub roster: Option<Roster>,
    pub name: Option<String>,
    pub team_key: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "roster")]
pub struct Roster {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub date: String,
    #[serde(rename = "players")]
    pub players: Option<Players>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "players")]
pub struct Players {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "player")]
    pub all_players: Option<Vec<crate::models::yahoo_league_resource::Player>>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename = "player")]
pub struct Player {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub player_key: Option<String>,
    pub player_id: Option<String>,
    pub name: Option<Name>,
    pub url: Option<String>,
    pub editorial_player_key: Option<String>,
    pub editorial_team_key: Option<String>,
    pub editorial_team_full_name: Option<String>,
    pub editorial_team_abbr: Option<String>,
    pub editorial_team_url: Option<String>,
    pub uniform_number: Option<String>,
    pub display_position: Option<String>,
    pub eligible_positions: Option<EligiblePositions>,
    // pub primary_position: Option<String>,
    pub headshot: Option<Headshot>,
    pub image_url: Option<String>,
    // pub is_undroppable: Option<String>,
    // pub position_type: Option<String>,
    pub eligible_positions_to_add: Option<EligiblePositionsToAdd>,
    pub has_player_notes: Option<String>,
    pub player_notes_last_timestamp: Option<String>,
    pub selected_position: Option<SelectedPosition>,
    // pub is_editable: Option<String>,
    // pub is_keeper: Option<IsKeeper>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    // #[serde(rename = "$text")]
    // pub text: Option<String>,
    pub full: Option<String>,
    pub first: Option<String>,
    pub last: Option<String>,
    // pub ascii_first: Option<String>,
    // pub ascii_last: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsKeeper {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub status: Option<Status>,
    pub cost: Option<Cost>,
    pub kept: Option<Kept>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cost {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kept {
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Headshot {
//     #[serde(rename = "$text")]
//     pub text: Option<String>,
//     pub url: Option<String>,
//     pub size: Option<String>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EligiblePositions {
    #[serde(rename = "$text")]
    // pub text: Option<String>,
    pub position: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EligiblePositionsToAdd {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectedPosition {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub coverage_type: Option<String>,
    pub date: Option<String>,
    pub position: Option<String>,
    pub is_flex: Option<String>,
}
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Headshot {
    url: Option<String>,
    size: Option<String>,
}





























//
// // use crate::http::report::Position;
// use chrono::NaiveDate;
// use serde::{Deserialize, Serialize};
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
// // #[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
// // pub struct Name {
// //     full: Option<String>,
// //     first: Option<String>,
// //     last: Option<String>,
// //     ascii_first: Option<String>,
// //     ascii_last: Option<String>,
// // }
// // #[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
// // pub struct SelectedPosition {
// //     coverage_type: Option<String>,
// //     date: Option<NaiveDate>,
// //     is_flex: Option<bool>,
// // }
