use serde::Deserialize;

use crate::{error::RestApiError, prelude::*};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct PageWikitext {
    pub id: usize,
    pub key: String,
    pub title: String,
    pub latest: RevisionTimestamp,
    pub content_model: String,
    pub license: LicenseModel,
    pub source: String,
}

#[derive(Clone, Debug)]
pub struct Page {
    title: String,
}

impl Page {
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
        }
    }

    pub async fn get(
        &self,
        api: &RestApi,
        follow_redirect: bool,
    ) -> Result<PageWikitext, RestApiError> {
        let path = format!("/page/{}", self.title);
        let mut params = HashMap::new();
        params.insert("redirect".to_string(), follow_redirect.to_string());
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: PageWikitext = response.json().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::rest_api_builder::RestApiBuilder;

    use super::*;

    #[tokio::test]
    async fn test_get_page() {
        let api = RestApiBuilder::wikipedia("en").build();
        let page = Page::new("Rust (programming language)");
        let wikitext = page
            .get(&api, false)
            .await
            .expect("Failed to get page content");
        assert!(wikitext.source.contains("Mozilla sponsorship"));
    }
}
