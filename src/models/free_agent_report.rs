use std::fmt::{Debug, Pointer};
use serde::{Deserialize};
use crate::models::roster::Position;
use crate::models::yahoo_league_resource::YahooXmlLeagueResource;

#[serde_with::serde_as]
#[derive(serde::Serialize, Deserialize, Clone, Debug, Default)]
// struct FreeAgentReport<'a> {
pub struct FreeAgentReport {
    pub free_agents: YahooXmlLeagueResource,
    pub total_players: Option<i32>,
    pub positions: Option<Vec<Position>>,
}

impl<'a> FreeAgentReport {
    pub fn new<'b>(
        free_agents: YahooXmlLeagueResource,
        total_players: Option<i32>,
        positions: Option<Vec<Position>>,
    ) -> FreeAgentReport {
        Self {
            free_agents,
            total_players,
            positions,
        }
    }

    pub fn default() -> Self {
        FreeAgentReport{
            free_agents: Default::default(),
            total_players: None,
            positions: None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum LoadDistribution {
    FRONT,
    BACK,
    EQUAL,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ReportBounds {
    pub start_week: u64,
    pub end_week: u64,
    pub team_id: u64,
}

impl ReportBounds {
    pub fn start_week(&self) -> Option<u64> { Some(self.start_week) }
    pub fn end_week(&self) -> Option<u64> { Some(self.end_week) }
    pub fn team(&self) -> Option<u64> { Some(self.team_id) }
}
