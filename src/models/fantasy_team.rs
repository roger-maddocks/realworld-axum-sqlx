use std::fmt::{Debug, Pointer};
use serde::{Deserialize};
use crate::models::yahoo_team_resource::YahooXmlTeamResource;

#[serde_with::serde_as]
#[derive(serde::Serialize, Deserialize, Clone, Debug, Default)]
pub struct FantasyTeam {
    pub yahoo_xml_team: YahooXmlTeamResource,
}

impl<'a> FantasyTeam {
    pub fn new<'b>(
        yahoo_xml_team: YahooXmlTeamResource,
    ) -> FantasyTeam {
        Self {
            yahoo_xml_team
        }
    }

    pub fn default() -> Self {
        FantasyTeam{
            yahoo_xml_team: Default::default()
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct FantasyTeamReportBounds {
    pub team_number: u64,
}

impl FantasyTeamReportBounds {
    pub fn team_number(&self) -> Option<u64> { Some(self.team_number) }
}
