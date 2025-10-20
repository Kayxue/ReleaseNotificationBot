use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod RequestBody {

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ReleaseRequestBody<'a> {
        pub action: &'a str,
        pub changes: Option<Value>,
        pub enterprise: Option<Value>,
        pub installation: Option<Value>,
        pub organization: Option<Value>,
        pub release: Release<'a>,
        pub repository: Value,
        pub sender: Option<Value>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Release<'a> {
        pub assets: Vec<Option<Asset<'a>>>,
        pub assets_url: &'a str,
        pub author: Option<Author<'a>>,
        pub body: Option<&'a str>,
        pub created_at: Option<&'a str>,
        pub draft: bool,
        pub html_url: &'a str,
        pub id: u64,
        pub immutable: bool,
        pub name: Option<&'a str>,
        pub node_id: &'a str,
        pub prerelease: bool,
        pub published_at: Option<&'a str>,
        pub reactions: Option<Value>,
        pub tag_name: &'a str,
        pub tarball_url: Option<&'a str>,
        pub target_commitish: &'a str,
        pub updated_at: Option<&'a str>,
        pub upload_url: &'a str,
        pub url: &'a str,
        pub zipball_url: Option<&'a str>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Author<'a> {
        pub avatar_url: Option<&'a str>,
        pub deleted: Option<bool>,
        pub email: Option<&'a str>,
        pub events_url: Option<&'a str>,
        pub followers_url: Option<&'a str>,
        pub following_url: Option<&'a str>,
        pub gists_url: Option<&'a str>,
        pub gravatar_id: Option<&'a str>,
        pub html_url: Option<&'a str>,
        pub id: u64,
        pub login: &'a str,
        pub name: Option<&'a str>,
        pub node_id: Option<&'a str>,
        pub organizations_url: Option<&'a str>,
        pub received_events_url: Option<&'a str>,
        pub repos_url: Option<&'a str>,
        pub site_admin: Option<bool>,
        pub starred_url: Option<&'a str>,
        pub subscriptions_url: Option<&'a str>,
        #[serde(rename = "type")]
        pub type_field: Option<&'a str>,
        pub url: Option<&'a str>,
        pub user_view_type: Option<&'a str>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Asset<'a> {
        pub url: &'a str,
        pub browser_download_url: &'a str,
        pub id: u64,
        pub node_id: &'a str,
        pub name: &'a str,
        pub label: Option<&'a str>,
        pub state: &'a str,
        pub content_type: &'a str,
        pub size: u64,
        pub download_count: u64,
        pub created_at: &'a str,
        pub updated_at: &'a str,
        pub uploader: Value,
    }
}
