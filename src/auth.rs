use std::marker::PhantomData;

use reqwest::header::HeaderValue;
use serde::Serialize;

use base64::{engine::general_purpose::URL_SAFE, Engine};

use crate::{
    sha::Sha256, user::UserAuth, Auth, ClientInfo, JellyfinClient, KeyAuth, NoAuth, Result,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AuthUserNameReq<'a> {
    username: &'a str,
    pw: &'a str,
}
impl<Sha: Sha256> JellyfinClient<NoAuth, Sha> {
    pub fn auth_key(self, key: impl ToString) -> JellyfinClient<KeyAuth, Sha> {
        let key = key.to_string();
        let auth_header = make_key_auth_header::<Sha>(&key, &self.client_info, &self.device_name);
        JellyfinClient {
            url: self.url,
            client: self.client,
            client_info: self.client_info,
            device_name: self.device_name,
            auth: KeyAuth {
                access_key: key,
                header: auth_header,
            },
            _phantom: PhantomData,
        }
    }

    pub async fn auth_user_name(
        self,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<JellyfinClient<Auth, Sha>> {
        let req = self
            .client
            .post(format!("{}Users/AuthenticateByName", self.url))
            .json(&AuthUserNameReq {
                username: username.as_ref(),
                pw: password.as_ref(),
            })
            .send()
            .await?;

        let auth: UserAuth = req.json().await?;
        let auth_header = make_user_auth_header::<Sha>(&auth, &self.client_info, &self.device_name);
        Ok(JellyfinClient {
            url: self.url,
            client: self.client,
            client_info: self.client_info,
            device_name: self.device_name,
            auth: Auth {
                user: auth.user,
                session_info: auth.session_info,
                access_token: auth.access_token,
                server_id: auth.server_id,
                header: auth_header,
            },
            _phantom: PhantomData,
        })
    }
}

fn make_user_auth_header<Sha: Sha256>(
    user_auth: &UserAuth,
    client_info: &ClientInfo,
    device_name: &str,
) -> HeaderValue {
    let mut val = r#"MediaBrowser Token=""#.to_string();
    val += &user_auth.access_token;
    val += r#"", Client=""#;
    val += &client_info.name;
    val += r#"", Version=""#;
    val += &client_info.version;
    val += r#"", Device=""#;
    URL_SAFE.encode_string(device_name.as_bytes(), &mut val);
    val += r#"", DeviceId""#;
    make_user_client_id::<Sha>(user_auth, client_info, device_name, &mut val);
    val.push('"');
    HeaderValue::try_from(val).expect("invalid client info for header value")
}

fn make_user_client_id<Sha: Sha256>(
    user_auth: &UserAuth,
    client_info: &ClientInfo,
    device_name: &str,
    out: &mut String,
) {
    let mut digest = Sha::new();
    digest.update(client_info.name.as_bytes());
    digest.update(client_info.version.as_bytes());
    digest.update(device_name.as_bytes());
    digest.update(user_auth.user.id.as_bytes());
    let hash = digest.finalize();
    URL_SAFE.encode_string(hash, out);
}

fn make_key_auth_header<Sha: Sha256>(
    key: &str,
    client_info: &ClientInfo,
    device_name: &str,
) -> HeaderValue {
    let mut val = r#"MediaBrowser Token=""#.to_string();
    val += key;
    val += r#"", Client=""#;
    val += &client_info.name;
    val += r#"", Version=""#;
    val += &client_info.version;
    val += r#"", Device=""#;
    URL_SAFE.encode_string(device_name.as_bytes(), &mut val);
    val += r#"", DeviceId""#;
    make_key_client_id::<Sha>(client_info, device_name, &mut val);
    val.push('"');
    HeaderValue::try_from(val).expect("invalid client info for header value")
}

fn make_key_client_id<Sha: Sha256>(client_info: &ClientInfo, device_name: &str, out: &mut String) {
    let mut digest = Sha::new();
    digest.update(client_info.name.as_bytes());
    digest.update(client_info.version.as_bytes());
    digest.update(device_name.as_bytes());
    let hash = digest.finalize();
    URL_SAFE.encode_string(hash, out);
}
