use reqwest::Client as HttpClient;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use url::ParseError;

#[derive(Clone, Debug)]
pub struct Client {
    http: HttpClient,
    base_url: url::Url,
    token: String,
}

#[derive(Debug)]
pub enum ClientError {
    ParseError(ParseError),
    InvalidToken,
    RequestError(reqwest::Error),
}

impl From<ParseError> for ClientError {
    fn from(err: ParseError) -> Self {
        ClientError::ParseError(err)
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::RequestError(err)
    }
}

impl Client {
    pub fn new(token: &str) -> Result<Self, ClientError> {
        if token.is_empty() || !token.starts_with("up:yeah:") {
            return Err(ClientError::InvalidToken);
        }

        Ok(Self {
            http: HttpClient::new(),
            base_url: url::Url::parse("https://api.up.com.au/api/v1/")?,
            token: token.to_string(),
        })
    }

    fn auth_headers(&self) -> Result<HeaderMap, ClientError> {
        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", self.token))
            .map_err(|_| ClientError::InvalidToken)?;
        headers.insert(AUTHORIZATION, auth_value);
        Ok(headers)
    }

    //Utility function to verify auth is ok [https://developer.up.com.au/#get_util_ping]
    pub async fn ping(&self) -> Result<serde_json::Value, ClientError> {
        let url = self.base_url.join("util/ping")?;
        let headers = self.auth_headers()?;

        let response = self.http.get(url).headers(headers).send().await?;

        let json = response.json::<serde_json::Value>().await?;
        Ok(json)
    }
}
