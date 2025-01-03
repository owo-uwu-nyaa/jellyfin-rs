use serde::Serialize;

use crate::{sha::Sha256, Auth, JellyfinClient, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetUserViewsQuery<'s> {
    user_id: Option<&'s str>,
    include_external_content: Option<bool>,
    preset_views: Option<&'s [&'s str]>,
    include_hidden: Option<bool>,
}

impl<Sha: Sha256> JellyfinClient<Auth, Sha> {
    pub async fn get_user_views(
        &self,
        user_id: Option<&str>,
        include_external_content: Option<bool>,
        preset_views: Option<&[&str]>,
        include_hidden: Option<bool>,
    ) -> Result<serde_json::Value> {
        let req = self
            .get(format!("{}UserViews", self.url))
            .query(&GetUserViewsQuery {
                user_id,
                include_external_content,
                preset_views,
                include_hidden,
            })
            .send()
            .await?;
        Ok(req.json().await?)
    }
}
