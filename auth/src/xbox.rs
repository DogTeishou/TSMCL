use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::error::Error;

#[derive(Serialize)]
struct Request<P> {
    #[serde(rename = "Properties")]
    properties: P,
    #[serde(rename = "RelyingParty")]
    relying_party: String,
    #[serde(rename = "TokenType")]
    token_type: String,
}

#[derive(Serialize)]
struct XSTSProperties {
    #[serde(rename = "SandboxId")]
    sandbox_id: String,
    #[serde(rename = "UserTokens")]
    user_tokens: Vec<String>,
}

#[derive(Serialize)]
struct UshProperties {
    #[serde(rename = "AuthMethod")]
    auth_method: String,
    #[serde(rename = "SiteName")]
    site_name: String,
    #[serde(rename = "RpsTicket")]
    rps_ticket: String,
}

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "Token")]
    token: String,
    #[serde(rename = "DisplayClaims")]
    display_claims: DisplayClaims,
}

#[derive(Deserialize)]
struct DisplayClaims {
    xui: Vec<Xui>,
}

#[derive(Deserialize)]
struct Xui {
    uhs: String,
}

pub struct XboxAuth {
    client: Client,
    pub(crate) access_token: String,
    pub(crate) token: String,
    pub(crate) ush: String,
}

impl XboxAuth {
    pub(crate) fn new(access_token: String) -> Self {
        XboxAuth {
            client: Client::new(),
            access_token,
            token: "".to_string(),
            ush: "".to_string(),
        }
    }

    pub(crate) async fn get_xbox_live_token(&mut self) -> Result<(), Error> {
        let request = Request {
            properties: UshProperties {
                auth_method: "RPS".to_string(),
                site_name: "user.auth.xboxlive.com".to_string(),
                rps_ticket: format!("d={}", self.access_token),
            },
            relying_party: "http://auth.xboxlive.com".to_string(),
            token_type: "JWT".to_string(),
        };
        let response = self.client
            .post("https://user.auth.xboxlive.com/user/authenticate")
            .json(&request)
            .send()
            .await?;
        let response: Response = response.json().await?;
        self.token = response.token;
        self.ush = response.display_claims.xui[0].uhs.clone();
        Ok(())
    }

    pub(crate) async fn get_xsts(&mut self) -> Result<(), Error> {
        let request = Request {
            properties: XSTSProperties {
                sandbox_id: "RETAIL".to_string(),
                user_tokens: vec![self.token.clone()],
            },
            relying_party: "rp://api.minecraftservices.com/".to_string(),
            token_type: "JWT".to_string(),
        };
        let response = self.client
            .post("https://xsts.auth.xboxlive.com/xsts/authorize")
            .json(&request)
            .send()
            .await?;
        let response: Response = response.json().await?;
        self.token = response.token;
        if response.display_claims.xui[0].uhs != self.ush {
            return Err(Error::XboxXuiMismatch);
        }
        Ok(())
    }
}