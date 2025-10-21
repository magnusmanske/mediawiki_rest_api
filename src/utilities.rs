use serde::Deserialize;

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
