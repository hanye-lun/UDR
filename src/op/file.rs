use serde;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize,Debug)]
pub struct File {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "integrity")]
    pub integrity: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    #[serde(rename = "size")]
    pub size: u32,
}