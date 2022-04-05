use std::{fmt::Display, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue},
    Client, ClientBuilder,
};
use serde::Serialize;

pub enum VioletRequestErrors {
    InvalidHeaderValue(InvalidHeaderValue),
    CreateClientError(reqwest::Error),
    InvalidUrl,
    FailedToSendRequest(u16),
}

impl From<InvalidHeaderValue> for VioletRequestErrors {
    fn from(err: InvalidHeaderValue) -> Self {
        VioletRequestErrors::InvalidHeaderValue(err)
    }
}

impl From<reqwest::Error> for VioletRequestErrors {
    fn from(err: reqwest::Error) -> Self {
        VioletRequestErrors::CreateClientError(err)
    }
}

#[derive(Serialize, Debug)]
pub struct VioletLogData<'a> {
    pub error_level: &'a str,
    pub message: &'a str,
    pub stack_trace: &'a str,
}

impl<'a> Display for VioletLogData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(format!("Invalid Log Data: {:#?}", self))
        )
    }
}

pub struct VioletRequest {
    reqwest_client: Client,
    base_url: String,
}

impl VioletRequest {
    pub fn new(
        token: &str,
        base_url: String,
        custom_timeout: Option<u64>,
    ) -> Result<Self, VioletRequestErrors> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Authentication", HeaderValue::from_str(token)?);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent("cattleya")
            .timeout(Duration::from_millis(custom_timeout.unwrap_or(30000)))
            .build()?;

        let violet = Self {
            base_url,
            reqwest_client: client,
        };

        Ok(violet)
    }

    pub async fn send_log<'a>(
        &self,
        log_data: &VioletLogData<'a>,
    ) -> Result<(), VioletRequestErrors> {
        let url = format!("{}/errors", &self.base_url);
        let mut limit = 5;

        loop {
            let response = self
                .reqwest_client
                .post(&url)
                .json(log_data)
                .send()
                .await
                .map_err(|err| {
                    log::error!("{}", err);
                    VioletRequestErrors::InvalidUrl
                })?;

            match response.status().as_u16() {
                201 => break Ok(()),
                err if limit > 0 => {
                    log::error!("{}", err);
                }
                err => {
                    log::error!("Original Error: {}\n\nRequest Error: {}", log_data, err);
                    break Err(VioletRequestErrors::FailedToSendRequest(err));
                }
            }

            limit -= 1;
        }
    }
}
