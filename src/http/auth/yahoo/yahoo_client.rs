use crate::builders::yahoo_auth_client_builder::YahooAuthClientBuilder;
use anyhow;
use oauth2;
use oauth2::http::{HeaderMap, HeaderValue};
use oauth2::url;
use oauth2::{ClientId, ClientSecret, RevocableToken};
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;
use url::{form_urlencoded, Url};

#[derive(Clone)]
pub struct YahooAuthClient {
    pub token_params: YahooTokenRequest,
    pub auth_params: YahooAuthRequest,
    pub refresh_token_params: YahooRefreshTokenRequest,
    pub token_url: String,
    pub auth_url: String,
    pub fantasy_sports_url: String,
    pub access_token: String,
    pub request_headers: HeaderMap,
    pub auth_headers: HeaderMap,
}

impl YahooAuthClient {
    pub fn builder() -> YahooAuthClientBuilder {
        YahooAuthClientBuilder::default()
    }
}

///Yahoo required fields for initiating auth request
#[derive(Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct YahooAuthRequest {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    response_type: String,
}
impl YahooAuthRequest {
    pub fn new() -> Self {
        YahooAuthRequest {
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            redirect_uri: "oob".to_string(),
            response_type: "code".to_string(),
        }
    }
}

///Yahoo required fields for acquiring access token
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct YahooTokenRequest {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    code: String,
    grant_type: String,
}
impl YahooTokenRequest {
    pub fn new() -> Self {
        YahooTokenRequest {
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            grant_type: "authorization_code".to_string(),
            redirect_uri: "oob".to_string(),
            code: env!("YAHOO_AUTH_CODE").to_string(),
        }
    }
}

///Yahoo required fields for refreshing access token
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct YahooRefreshTokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    refresh_token: String,
}
impl YahooRefreshTokenRequest {
    pub fn new() -> Self {
        YahooRefreshTokenRequest {
            grant_type: "refresh_token".to_string(),
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            redirect_uri: "oob".to_string(),
            refresh_token: env!("YAHOO_REFRESH_TOKEN").to_string(),
        }
    }
}

/// Yahoo's response values when requesting token refresh
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct YahooRefreshTokenResponse {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: Option<String>,
}

impl YahooRefreshTokenResponse {
    pub fn new() -> Self {
        YahooRefreshTokenResponse {
            access_token: None,
            refresh_token: None,
            expires_in: None,
            token_type: None,
        }
    }
}

///Yahoo required fields for Get request
#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YahooGetRequest {
    pub client_id: String,
    pub client_secret: String,
    pub authorization: String,
    pub refresh_token: String,
}
impl YahooGetRequest {
    pub fn new() -> Self {
        YahooGetRequest {
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            authorization: "".to_string(),
            refresh_token: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct YahooResponses {
    pub tokens: Vec<YahooRefreshTokenResponse>,
}

pub(crate) fn generate_refresh_token_headers(
    client_id: ClientId,
    secret: ClientSecret,
) -> HeaderMap {
    let mut headers: HeaderMap = Default::default();
    let urlencoded_id: String = form_urlencoded::byte_serialize(&client_id.as_bytes()).collect();
    let urlencoded_secret: String =
        form_urlencoded::byte_serialize(secret.secret().as_bytes()).collect();
    let b64_credential = base64::encode(&format!("{}:{}", &urlencoded_id, urlencoded_secret));

    headers.append(
        "Authorization-Code",
        HeaderValue::from_str(&format!("Basic {}", &b64_credential)).unwrap(),
    );
    headers.append(
        "Content-Type",
        "application/x-www-form-urlencoded"
            .to_string()
            .parse()
            .unwrap(),
    );
    headers
}

#[derive(Error, Debug)]
pub enum YahooError {
    #[error("fetching the data from yahoo! finance failed")]
    FetchFailed(String),
    #[error("deserializing response from yahoo! finance failed")]
    DeserializeFailed(#[from] serde_json::Error),
    #[error("connection to yahoo! finance server failed")]
    ConnectionFailed(#[from] reqwest::Error),
    #[error("yahoo! finance return invalid JSON format")]
    InvalidJson,
    #[error("yahoo! finance returned an empty data set")]
    EmptyDataSet,
    #[error("yahoo! finance returned inconsistent data")]
    DataInconsistency,
    #[error("constructing yahoo! finance client failed")]
    BuilderFailed,
}
