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
    pub id: Option<usize>,
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

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct DiffOffset {
    pub from: Option<usize>,
    pub to: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiffInfo {
    #[serde(alias = "lineNumber")]
    pub line_number: Option<usize>,
    pub offset: DiffOffset,
    pub text: String,
    #[serde(alias = "type")]
    pub type_id: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiffSection {
    pub heading: String,
    pub level: usize,
    pub offset: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiffSections {
    pub id: usize,
    pub sections: Vec<DiffSection>,
    pub slot_role: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Diff {
    pub diff: Vec<DiffInfo>,
    pub from: DiffSections,
    pub to: DiffSections,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Filter {
    Anonymous,
    Bot,
    Reverted,
    Minor,
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Filter::Anonymous => write!(f, "anonymous"),
            Filter::Bot => write!(f, "bot"),
            Filter::Reverted => write!(f, "reverted"),
            Filter::Minor => write!(f, "minor"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum HistoryFilterExtended {
    Anonymous,
    Temporary,
    Bot,
    Editors,
    Edits,
    Minor,
    Reverted,
    AnonEdits,
    BotEdits,
    RevertedEdits,
}

impl fmt::Display for HistoryFilterExtended {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anonymous => write!(f, "anonymous"),
            Self::Temporary => write!(f, "temporary"),
            Self::Bot => write!(f, "bot"),
            Self::Editors => write!(f, "editors"),
            Self::Edits => write!(f, "edits"),
            Self::Minor => write!(f, "minor"),
            Self::Reverted => write!(f, "reverted"),
            Self::AnonEdits => write!(f, "anonedits"),
            Self::BotEdits => write!(f, "botedits"),
            Self::RevertedEdits => write!(f, "revertededits"),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HistoryRevisionInfo {
    pub id: usize,
    pub size: usize,
    pub delta: isize,
    pub comment: String,
    pub minor: bool,
    pub timestamp: String,
    pub user: UserInfo,
}

#[derive(Clone, Debug, Deserialize)]
pub struct History {
    pub revisions: Vec<HistoryRevisionInfo>,
    pub latest: Option<String>,
    pub older: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct HistoryCounts {
    pub count: usize,
    pub limit: bool,
}
