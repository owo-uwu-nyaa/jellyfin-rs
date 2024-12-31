use serde::{Deserialize, Serialize};

use crate::{err::JellyfinError, JellyfinClient};

use super::err::Result;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct UserIdQuery<'a> {
    pub user_id: &'a str,
}

impl JellyfinClient {
    pub async fn get_user_library_root(&self) -> Result<serde_json::Value> {
        let auth = self.auth.as_ref().ok_or(JellyfinError::AuthNotFound)?;

        let req = self
            .client
            .get(format!("{}Items/Root", self.url,))
            .query(&UserIdQuery {
                user_id: &auth.user.id,
            })
            .header("X-Emby-Authorization", auth.to_emby_header())
            .send()
            .await?;
        Ok(req.json().await?)
    }
}
