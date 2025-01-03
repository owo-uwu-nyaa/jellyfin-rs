use crate::{sha::Sha256, Authed, JellyfinClient};

use super::err::Result;

impl<Auth: Authed, Sha: Sha256> JellyfinClient<Auth, Sha> {
    pub async fn get_user_library_root(&self) -> Result<serde_json::Value> {
        let req = self.get(format!("{}Items/Root", self.url,)).send().await?;
        Ok(req.json().await?)
    }

    pub async fn get_user_library_latest_media(&self) -> Result<serde_json::Value> {
        let req = self.get(format!("{}Items/Latest", self.url)).send().await?;
        Ok(req.json().await?)
    }
}
