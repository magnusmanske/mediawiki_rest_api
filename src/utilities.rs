use core::fmt;

use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct LicenseModel {
    pub url: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevisionTimestamp {
    pub id: usize,
    pub timestamp: String,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum HtmlFlavor {
    View,
    Stash,
    Fragment,
    Edit,
}

impl fmt::Display for HtmlFlavor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlFlavor::View => write!(f, "view"),
            HtmlFlavor::Stash => write!(f, "stash"),
            HtmlFlavor::Fragment => write!(f, "fragment"),
            HtmlFlavor::Edit => write!(f, "edit"),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LanguageLink {
    pub code: String,
    pub name: String,
    pub key: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserInfo {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileRevision {
    pub timestamp: String,
    pub user: UserInfo,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MediaType {
    pub mediatype: String,
    pub size: Option<usize>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub duration: Option<f64>,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileInfo {
    pub title: String,
    pub file_description_url: String,
    pub latest: FileRevision,
    pub preferred: MediaType,
    pub original: MediaType,
    pub thumbnail: Option<MediaType>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Lint {
    #[serde(alias = "type")]
    pub type_name: String,
    pub dsr: Vec<Option<usize>>,
    #[serde(alias = "templateInfo")]
    pub template_info: TemplateInfo,
    pub params: Value,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PageInfo {
    pub id: usize,
    pub key: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevisionInfo {
    pub id: usize,
    pub size: usize,
    pub delta: isize,
    pub comment: String,
    pub minor: bool,
    pub timestamp: String,
    pub content_model: String,
    pub page: PageInfo,
    pub license: LicenseModel,
    pub user: UserInfo,
}
