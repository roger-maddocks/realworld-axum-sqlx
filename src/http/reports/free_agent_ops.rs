use crate::http::clients::yahoo::yahoo_auth_client_builder::{get_reqwest_headermap, YahooAuthClientBuilder};
use crate::http::clients::yahoo::yahoo_client::{YahooAuthClient};
use reqwest::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use axum::extract::State;
use axum::{Json, Router};
use axum::routing::get;
use quick_xml;
use quick_xml::events::Event;
use quick_xml::Reader;
use crate::http::{ApiContext, Result};
//
// pub(crate) fn router() -> Router<ApiContext> {
//     Router::new()
//         .route("/api/reports/top_free_agents", get())
// }
//
// async fn get_top_free_agents(
//     ctx: State<ApiContext>,
//     Json(req): Json<crate::http::reports::report::ReportBounds>,
// ) -> Result<Json<<'static>> {
//
//     )

#[derive(Debug, Copy, Clone)]
pub enum League {
    Nhl,
    Nba,
    Mlb,
    Nfl,
}
impl fmt::Display for League {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Clone)]
pub struct YahooFantasyFactory {
    pub yahoo_client: YahooAuthClient,
    league: League,
}

struct Roster();

impl YahooFantasyFactory {
    pub async fn new_factory(league: League) -> YahooFantasyFactory {
        YahooFantasyFactory {
            yahoo_client: YahooAuthClientBuilder::new().build().await,
            league,
        }
    }

    // pub async fn get_top_ten_free_agents(&self) -> Result<(), Error> {
    //    //get free agents, then take each player and get last 5 game scores
    //    //  self.get_free_agents()
    // }

    pub async fn get_my_roster(&self) -> Result<(), Error> {
        let url = env!["YAHOO_V2_URL"].to_string() + "/team/427.l.28172.t.7/roster;";
        let client = reqwest::Client::new();
        // .headers(get_reqwest_headermap(&self.yahoo_client.)).await)

        let response = client
            .get(url)
            .headers(self.yahoo_client.auth_headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let doc = roxmltree::Document::parse(&response);

        for node in doc.unwrap().descendants() {
            println!("key: {:?} | val: {:?}", node.tag_name().name(), node.text());
        }

        // println!("roster: {:?}", response.unwrap());

        Ok(())
    }

    ///
    /// Different options to be implemented
    /// //"/league/427.l.28172/players;count=5;status=A;sort=PTS";
    /// //"/league/427.l.28172/players;status=A";
    /// //"/league/427.l.28172";
    /// //"/game/nhl";
    ///
    ///
    pub async fn get_free_agents(&mut self) -> Result<(), roxmltree::Error> {
        //-> Result<(), Error> {
        let url = env!["YAHOO_V2_URL"].to_string()
            + "/league/"
            + env!["YAHOO_LEAGUE_KEY"]
            // + "/league/427.l.28172/players;count=2;status=A;sort=PTS";
            + "/players;player_keys=427.p.5697/stats";
        let client = reqwest::Client::new();

        // self.yahoo_client.generate_get_request_headers().await;
        // .headers(get_reqwest_headermap(&self.yahoo_client.request_headers.clone()).await)

        let response = client
            .get(url)
            .headers(self.yahoo_client.auth_headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("result's: {:?}", &response);
        // println!("resulting no newlines: {:?}", &response.replace("\n", ""));

        let mut reader = Reader::from_str(&response);
        // reader.trim_text(true);

        let mut count = 0;
        let mut txt = Vec::new();
        let mut decl = Vec::new();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => count += 1,
                Event::Decl(e) => decl.push(e.into_owned()),
                Event::Text(e) => txt.push(e.unescape().unwrap().into_owned()),
                Event::Eof => break,
                _ => (),
            }
        }

        println!("faaaaa's: {:?}", txt);
        println!("faaaaa's: {:?}", decl);

        Ok(())
    }
    pub async fn get_league_stat_categories(&mut self) -> Result<(), roxmltree::Error> { //-> Result<(), Error> {

        // .headers(get_reqwest_headermap(&self.yahoo_client.request_headers.clone()).await)
        let url = env!["YAHOO_V2_URL"].to_string() + "/game/nhl/stat_categories"; // "/league/427.l.28172/players;count=1;status=A;sort=PTS";
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .headers(self.yahoo_client.request_headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("dummy head: {:#?}", &response.replace("\n", ""));

        Ok(())
    }

    pub async fn get_league_resource(&self) -> Result<String, Error> {
        let url = env!["YAHOO_V2_URL"].to_string() + "/users;use_login=1/games;game_keys=nhl/teams";
        let client = reqwest::Client::new();

        // .headers(get_reqwest_headermap(&self.yahoo_client.request_headers.clone()).await)

        let response = client
            .get(url)
            .headers(self.yahoo_client.request_headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await;

        response
    }

    // pub async fn get_test_roster() -> Roster {
    //     let mut my_roster = Roster::new();
    //
    //     let position = vec![C];
    //     let elias = Player::new(
    //         "Elias".to_string(),
    //         "Pettersson".to_string(),
    //         position.clone(),
    //         false,
    //         NhlFranchise::VancouverCanucks,
    //         Team::new("VAN".to_string(), 21),
    //         "".to_string(),
    //     );
    //     // "VAN", msf_id: 21
    //
    //     let zib = Player::new(
    //         "Mika".to_string(),
    //         "Zibanejad".to_string(),
    //         position.clone(),
    //         false,
    //         NhlFranchise::NewYorkRangers,
    //         Team::new("NYR".to_string(), 9),
    //         "".to_string(),
    //     );
    //     // "NYR", msf_id: 9
    //
    //     let nico = Player::new(
    //         "Nico".to_string(),
    //         "Hischier".to_string(),
    //         position.clone(),
    //         false,
    //         NhlFranchise::NewJerseyDevils,
    //         Team::new("NJD".to_string(), 7),
    //         "".to_string(),
    //     );
    //     // "NJD", msf_id: 7
    //
    //     my_roster.add_player(elias);
    //     my_roster.add_player(zib);
    //     my_roster.add_player(nico);
    //
    //     my_roster
    // }
}
