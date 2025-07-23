use reqwest::{Client as HttpClient};
use url::ParseError;

#[derive(Clone, Debug)]
pub struct Client {
    http: HttpClient,
    base_url: url::Url,
    //look into adding a bearer token
    token: String
}

impl Client {
    pub fn new(token: &str) -> Result<Self, ParseError> {
        Ok(Self {
            http: HttpClient::new(),
            base_url: url::Url::parse("https://api.up.com.au/api/v1/")?,
            token: token.to_string(),
        })
    }
}