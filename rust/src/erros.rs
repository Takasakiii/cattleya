use std::fmt::Display;

use reqwest::header::InvalidHeaderValue;

use crate::CattleyaState;

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

impl Display for VioletRequestErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VioletRequestErrors::InvalidHeaderValue(err) => {
                write!(f, "InvalidHeaderValue: {}", err)
            }
            VioletRequestErrors::CreateClientError(err) => write!(f, "CreateClientError: {}", err),
            VioletRequestErrors::InvalidUrl => write!(f, "InvalidURL"),
            VioletRequestErrors::FailedToSendRequest(status) => {
                write!(f, "Failed to send request with status code {}", status)
            }
        }
    }
}

pub enum CattleyaInitError {
    BaseUrlInvalid,
    Unspecified(String),
    InvalidTokenValue,
    CattleyaEarlyInit,
}

impl From<VioletRequestErrors> for CattleyaInitError {
    fn from(err: VioletRequestErrors) -> Self {
        match err {
            VioletRequestErrors::InvalidHeaderValue(_) => CattleyaInitError::InvalidTokenValue,
            VioletRequestErrors::CreateClientError(err) => {
                CattleyaInitError::Unspecified(err.to_string())
            }
            VioletRequestErrors::InvalidUrl => CattleyaInitError::BaseUrlInvalid,
            VioletRequestErrors::FailedToSendRequest(_) => unreachable!(),
        }
    }
}

impl From<CattleyaState> for CattleyaInitError {
    fn from(_: CattleyaState) -> Self {
        CattleyaInitError::CattleyaEarlyInit
    }
}
