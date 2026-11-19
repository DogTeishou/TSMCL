use std::time::Duration;
use reqwest::Client;
use crate::error::Error;
use serde::{Deserialize};
use crate::{get_client_id, SCOPE};

#[derive(Deserialize, Clone)]
pub struct DeviceCodeResponse {
    pub user_code: String,
    pub verification_uri: String,
    device_code: String,
}

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub(crate) struct MicrosoftAuth {
    client_id: &'static str,
    pub(crate) device: DeviceCodeResponse,
    pub(crate) token: TokenResponse,
    client: Client,
}

impl DeviceCodeResponse {
    fn new() -> Self {
        Self {
            user_code: String::new(),
            verification_uri: String::new(),
            device_code: String::new(),
        }
    }
}

impl MicrosoftAuth {
    pub fn new(refresh_token: String) -> Result<Self, Error> {
        let client = Client::new();
        Ok(Self {
            client_id: get_client_id()?,
            device: DeviceCodeResponse::new(),
            token: TokenResponse {
                access_token: Option::from(String::new()),
                refresh_token: Some(refresh_token),
                error: None,
            },
            client
        })
    }

    pub(crate) async fn get_device_code(&mut self) -> Result<(), Error> {
        let response = self.client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
            .form(&[
                ("client_id", self.client_id),
                ("scope", SCOPE),
            ])
            .send()
            .await?;

        let data: DeviceCodeResponse = response.json().await?;
        self.device = data;
        Ok(())
    }

    pub(crate) async fn get_tokens(&mut self) -> Result<(), Error> {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let response = self.client
                .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
                .form(&[
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                    ("device_code", &self.device.device_code),
                    ("client_id", self.client_id),
                ])
                .send()
                .await?;

            self.token = response.json().await?;
            if let Some(error) = &self.token.error {
                if error.eq("authorization_pending") {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                } else if error.eq("slow_down") {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                } else if error.eq("bad_verification_code") {
                    return Err(Error::BadVerificationCode);
                } else if error.eq("authorization_declined") {
                    return Err(Error::AuthorizationDeclined);
                } else if error.eq("expired_token") {
                    return Err(Error::ExpiredToken);
                } else {
                    return Err(Error::MicrosoftError(error.clone()));
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    pub(crate) async fn refresh_tokens(&mut self) -> Result<(), Error> {
        if let Some(refresh_token) = &self.token.refresh_token {
            let response = self.client
                .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
                .form(&[
                    ("grant_type", "refresh_token"),
                    ("refresh_token", refresh_token),
                    ("client_id", self.client_id),
                    ("scope", SCOPE),
                ])
                .send()
                .await?;

            self.token = response.json().await?;
            Ok(())
        } else {
            Err(Error::NoRefreshToken)
        }
    }
}