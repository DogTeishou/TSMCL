use reqwest::header::AUTHORIZATION;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::get_client_id;
use crate::player::Player;

#[derive(Serialize)]
struct Request {
    #[serde(rename = "identityToken")]
    identity_token: String,
}

#[derive(Deserialize)]
struct AccessResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Deserialize)]
struct EntitlementsResponse {
    #[serde(default)]
    items: Vec<Entitlement>,
}

#[derive(Deserialize)]
struct Entitlement {
    name: String,
}

/// Entitlement names that mean the account owns Minecraft: Java Edition.
const MINECRAFT_ENTITLEMENTS: [&str; 2] = ["product_minecraft", "game_minecraft"];

#[derive(Deserialize)]
struct Profile {
    id: String,
    name: String,
}

pub struct MinecraftAuth {
    client: Client,
    uhs: String,
    xsts_token: String,
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

impl MinecraftAuth {
    pub fn new(xsts_token: String, uhs: String, refresh_token: String) -> Self {
        MinecraftAuth {
            client: Client::new(),
            uhs,
            xsts_token,
            refresh_token,
            access_token: String::new(),
            expires_in: 0,
        }
    }

    pub(crate) async fn get_access_token(&mut self) -> Result<(), Error> {
        let request = Request {
            identity_token: format!("XBL3.0 x={};{}", self.uhs, self.xsts_token),
        };
        let response = self.client
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<AccessResponse>()
            .await?;
        self.access_token = response.access_token;
        self.expires_in = response.expires_in;
        Ok(())
    }

    /// Check that the account owns Minecraft: Java Edition.
    ///
    /// Accounts without the game still get a successful response, just with an empty
    /// `items` list, so the entitlement names must be checked.
    pub(crate) async fn is_have_minecraft(&self) -> Result<bool, Error> {
        let response: EntitlementsResponse = self.client
            .get("https://api.minecraftservices.com/entitlements/mcstore")
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(response
            .items
            .iter()
            .any(|item| MINECRAFT_ENTITLEMENTS.contains(&item.name.as_str())))
    }

    pub(crate) async fn get_player(&mut self) -> Result<Player, Error> {
        let response = self.client
            .get("https://api.minecraftservices.com/minecraft/profile")
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .send()
            .await?;
        // 404 means the account has no Java Edition profile (no game, or no username set yet)
        if response.status() == StatusCode::NOT_FOUND {
            return Err(Error::NoMinecraftAccount);
        }
        let profile: Profile = response.error_for_status()?.json().await?;
        Ok(Player {
            username: profile.name,
            uuid: profile.id,
            access_token: self.access_token.clone(),
            expires_in: self.expires_in,
            client_id: Some(get_client_id()?.to_string()),
            x_uid: None,
            refresh_token: Some(self.refresh_token.clone()),
        })
    }
}