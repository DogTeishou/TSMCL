use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::crypto;
use crate::error::Error;
use crate::microsoft::{DeviceCodeResponse, MicrosoftAuth};
use crate::minecraft::MinecraftAuth;
use crate::xbox::XboxAuth;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub username: String,
    pub uuid: String,
    pub access_token: String,
    pub expires_in: u64,
    pub client_id: Option<String>,
    pub x_uid: Option<String>,
    pub refresh_token: Option<String>,
}

impl Player {
    pub async fn new(f: impl Fn(DeviceCodeResponse)) -> Result<Self, Error> {
        let mut microsoft = MicrosoftAuth::new("".to_string())?;
        microsoft.get_device_code().await?;
        f(microsoft.device.clone());
        microsoft.get_tokens().await?;
        let mut xbox = XboxAuth::new(microsoft.token.access_token.unwrap_or(String::new()).clone());
        xbox.get_xbox_live_token().await?;
        xbox.get_xsts().await?;
        let mut minecraft = MinecraftAuth::new(xbox.token.clone(), xbox.ush.clone(), microsoft.token.refresh_token.unwrap_or(String::new()).clone());
        minecraft.get_access_token().await?;
        if !minecraft.is_have_minecraft().await? {
            return Err(Error::NoMinecraftAccount);
        }
        Ok(minecraft.get_player().await?)
    }

    pub async fn refresh(&mut self) -> Result<Self, Error> {
        if let Some(refresh_token) = &self.refresh_token {
            let mut microsoft = MicrosoftAuth::new(refresh_token.clone())?;
            microsoft.refresh_tokens().await?;
            let mut xbox = XboxAuth::new(microsoft.token.access_token.unwrap_or(String::new()).clone());
            xbox.get_xbox_live_token().await?;
            xbox.get_xsts().await?;
            let mut minecraft = MinecraftAuth::new(xbox.token.clone(), xbox.ush.clone(), microsoft.token.refresh_token.unwrap_or(String::new()).clone());
            minecraft.get_access_token().await?;
            if !minecraft.is_have_minecraft().await? {
                return Err(Error::NoMinecraftAccount);
            }
            Ok(minecraft.get_player().await?)
        } else {
            Err(Error::NoRefreshToken)
        }
    }

    pub async fn save(&self, dir: &Path) -> Result<(), Error> {
        let path = dir.join(format!("{}.toml", self.uuid));
        if !path.exists() {
            tokio::fs::File::create(&path).await?;
        }
        let mut stored = self.clone();
        stored.access_token = crypto::encrypt(&self.access_token)?;
        stored.refresh_token = self.refresh_token.as_deref().map(crypto::encrypt).transpose()?;
        let string = toml::to_string_pretty(&stored)?;
        tokio::fs::write(path, string).await?;
        Ok(())
    }

    pub async fn load(path: &Path, uuid: &str) -> Result<Player, Error> {
        let path = path.join(format!("{}.toml", uuid));
        let string = tokio::fs::read_to_string(path).await?;
        let mut player: Player = toml::from_str(&string)?;
        player.access_token = crypto::decrypt(&player.access_token)?;
        player.refresh_token = player.refresh_token.as_deref().map(crypto::decrypt).transpose()?;
        Ok(player)
    }

    /// Load every saved account in `dir`, sorted by username.
    ///
    /// Files that cannot be read or decrypted (e.g. old plaintext files, or files written
    /// with a key that is no longer in the credential store) are skipped, so one broken
    /// file does not hide the other accounts. A missing directory yields an empty list.
    pub async fn load_all(dir: &Path) -> Result<Vec<Player>, Error> {
        let mut entries = match tokio::fs::read_dir(dir).await {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e.into()),
        };

        let mut players = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "toml") {
                continue;
            }
            let Some(uuid) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if let Ok(player) = Self::load(dir, uuid).await {
                players.push(player);
            }
        }
        players.sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));
        Ok(players)
    }

    /// Delete the saved account with `uuid`. Deleting an account that does not exist is not an error.
    pub async fn delete(dir: &Path, uuid: &str) -> Result<(), Error> {
        match tokio::fs::remove_file(dir.join(format!("{}.toml", uuid))).await {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.into()),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use arboard::Clipboard;
    use crate::microsoft::DeviceCodeResponse;
    use crate::player::Player;

    #[tokio::test]
    async fn test_new() -> Result<(), Box<dyn std::error::Error>> {
        let player = Player::new(|device: DeviceCodeResponse| {
            println!("user_code: {}, verify_url: {}", device.user_code, device.verification_uri);
            open::that(device.verification_uri).unwrap();
            Clipboard::new().unwrap().set_text(device.user_code).unwrap();
        }).await?;
        println!("player: {:?}", player);
        Ok(())
    }

    #[tokio::test]
    async fn test_load() -> Result<(), Box<dyn std::error::Error>> {
        let player = Player::load(&Path::new("test"), "test").await?;
        println!("player: {:?}", player);
        Ok(())
    }

    #[tokio::test]
    async fn test_save() -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::current_dir()?.parent().unwrap().join(".tsmcl").join("accounts");
        let player = Player {
            username: "test".to_string(),
            uuid: "test".to_string(),
            access_token: "test".to_string(),
            refresh_token: Some("test".to_string()),
            expires_in: 0,
            client_id: None,
            x_uid: None,
        };
        player.save(&dir).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_save_load_encrypted() -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::temp_dir().join("tsmcl-test-accounts");
        tokio::fs::create_dir_all(&dir).await?;
        let player = Player {
            username: "test".to_string(),
            uuid: "encrypted-test".to_string(),
            access_token: "plain-access-token".to_string(),
            refresh_token: Some("plain-refresh-token".to_string()),
            expires_in: 0,
            client_id: None,
            x_uid: None,
        };
        player.save(&dir).await?;

        let on_disk = tokio::fs::read_to_string(dir.join("encrypted-test.toml")).await?;
        assert!(!on_disk.contains("plain-access-token"));
        assert!(!on_disk.contains("plain-refresh-token"));

        let loaded = Player::load(&dir, "encrypted-test").await?;
        assert_eq!(loaded.access_token, "plain-access-token");
        assert_eq!(loaded.refresh_token.as_deref(), Some("plain-refresh-token"));

        tokio::fs::remove_dir_all(&dir).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_accounts() -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::temp_dir().join("tsmcl-test-multi-accounts");
        let _ = tokio::fs::remove_dir_all(&dir).await;
        tokio::fs::create_dir_all(&dir).await?;

        for (name, uuid) in [("Steve", "uuid-steve"), ("alex", "uuid-alex")] {
            Player {
                username: name.to_string(),
                uuid: uuid.to_string(),
                access_token: format!("{name}-access"),
                refresh_token: Some(format!("{name}-refresh")),
                expires_in: 0,
                client_id: None,
                x_uid: None,
            }
            .save(&dir)
            .await?;
        }
        // A broken file must not prevent the others from loading
        tokio::fs::write(dir.join("broken.toml"), "not valid").await?;

        let players = Player::load_all(&dir).await?;
        let names: Vec<_> = players.iter().map(|p| p.username.as_str()).collect();
        assert_eq!(names, ["alex", "Steve"]);
        assert_eq!(players[1].access_token, "Steve-access");
        assert_eq!(players[1].refresh_token.as_deref(), Some("Steve-refresh"));

        Player::delete(&dir, "uuid-alex").await?;
        Player::delete(&dir, "uuid-alex").await?; // deleting twice is fine
        let players = Player::load_all(&dir).await?;
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].username, "Steve");

        assert!(Player::load_all(&dir.join("missing")).await?.is_empty());

        tokio::fs::remove_dir_all(&dir).await?;
        Ok(())
    }
}