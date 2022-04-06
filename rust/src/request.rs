use std::fmt::Display;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde::Serialize;

use crate::erros::VioletRequestErrors;

#[derive(Serialize, Debug)]
pub(crate) struct VioletLogData {
    pub error_level: String,
    pub message: String,
    pub stack_trace: String,
}

impl Display for VioletLogData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).unwrap_or(format!("Invalid Log Data: {:#?}", self))
        )
    }
}

pub(crate) struct VioletRequest {
    reqwest_client: Client,
    base_url: String,
}

impl VioletRequest {
    pub fn new(token: &str, base_url: String) -> Result<Self, VioletRequestErrors> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Authentication", HeaderValue::from_str(token)?);

        let client = ClientBuilder::new().default_headers(headers).build()?;

        let violet = Self {
            base_url,
            reqwest_client: client,
        };

        Ok(violet)
    }

    pub async fn send_log(&self, log_data: VioletLogData) -> Result<(), VioletRequestErrors> {
        let url = format!("{}/errors", &self.base_url);
        let mut limit = 5;

        loop {
            let response = self
                .reqwest_client
                .post(&url)
                .json(&log_data)
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
