#[derive(Debug, PartialEq)]
pub struct MySportsFeedProfile {
    api_key: String,
    api_pw: String,
}

impl MySportsFeedProfile {
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::default()
    }
    pub fn get_2024_daily_url() -> String {
        return env!("MSF_GAMES_2024_URL").to_owned();
    }
    pub fn get_2023_daily_url() -> String {
        return env!("MSF_GAMES_2023_URL").to_owned();
    }
}

#[derive(Default)]
pub struct ProfileBuilder {
    api_key: String,
    api_pw: String,
}

impl ProfileBuilder {
    pub fn new() -> ProfileBuilder {
        ProfileBuilder {
            api_key: String::from(env!("MSF_API_KEY")),
            api_pw: String::from("MYSPORTSFEEDS"),
        }
    }
    pub fn name(mut self, api_key: String, api_pw: String) -> ProfileBuilder {
        self.api_key = api_key;
        self.api_pw = api_pw;
        self
    }
    pub fn build(self) -> MySportsFeedProfile {
        MySportsFeedProfile {
            api_key: self.api_key,
            api_pw: self.api_pw,
        }
    }
    pub fn get_daily_url() -> String {
        return env!("MSF_GAMES_2024_URL").to_owned();
    }
}
