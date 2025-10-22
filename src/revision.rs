use std::collections::HashMap;

use serde_json::{Value, from_value};

use crate::{
    error::RestApiError,
    prelude::{HtmlFlavor, RestApi, RevisionInfo},
};

#[derive(Clone, Copy, Debug)]
pub struct Revision {
    id: usize,
}

impl Revision {
    pub const fn new(id: usize) -> Self {
        Self { id }
    }

    pub const fn id(&self) -> usize {
        self.id
    }

    /// Retrieves basic revision information and wikitext.
    pub async fn get(&self, api: &RestApi) -> Result<(RevisionInfo, String), RestApiError> {
        let path = format!("/revision/{}", self.id);
        let params = HashMap::new();
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let j: Value = response.json().await?;
        let wikitext = j["source"]
            .as_str()
            .ok_or(RestApiError::MissingResults)?
            .to_string();
        let ret = from_value::<RevisionInfo>(j)?;
        Ok((ret, wikitext))
    }

    /// Retrieves the HTML for the revision.
    pub async fn get_html(
        &self,
        api: &RestApi,
        stash: bool,
        flavor: HtmlFlavor,
    ) -> Result<String, RestApiError> {
        let path = format!("/revision/{}/html", self.id);
        let mut params = HashMap::new();
        params.insert("stash".to_string(), stash.to_string());
        params.insert("flavor".to_string(), flavor.to_string());
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret = response.text().await?;
        Ok(ret)
    }

    /// Retrieves basic revision information and the HTML for the revision.
    pub async fn get_with_html(
        &self,
        api: &RestApi,
        stash: bool,
        flavor: HtmlFlavor,
    ) -> Result<(RevisionInfo, String), RestApiError> {
        let path = format!("/revision/{}/with_html", self.id);
        let mut params = HashMap::new();
        params.insert("stash".to_string(), stash.to_string());
        params.insert("flavor".to_string(), flavor.to_string());
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let j: Value = response.json().await?;
        let html = j["html"]
            .as_str()
            .ok_or(RestApiError::MissingResults)?
            .to_string();
        let ret = from_value::<RevisionInfo>(j)?;
        Ok((ret, html))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest_api_builder::RestApiBuilder;
    // use wiremock::matchers::{method, path};
    // use wiremock::{Mock, MockServer, ResponseTemplate};
    //
    // test revision id 1316925953
    // test revision old_id 1316608902

    #[tokio::test]
    async fn test_get() {
        let api = RestApiBuilder::wikipedia("en").build();
        let revision = Revision::new(1316925953);
        let (revision_info, wikitext) = revision
            .get(&api)
            .await
            .expect("Failed to get page content");
        assert_eq!(revision_info.size, 114334);
        assert!(wikitext.contains("[[FreeBSD]]"));
    }

    #[tokio::test]
    async fn test_get_html() {
        let api = RestApiBuilder::wikipedia("en").build();
        let revision = Revision::new(1316925953);
        let html = revision
            .get_html(&api, false, HtmlFlavor::View)
            .await
            .expect("Failed to get page content");
        assert!(html.contains("<title>Rust (programming language)</title>"));
    }

    #[tokio::test]
    async fn test_get_with_html_rev() {
        let api = RestApiBuilder::wikipedia("en").build();
        let revision = Revision::new(1316925953);
        let (revision_info, html) = revision
            .get_with_html(&api, false, HtmlFlavor::View)
            .await
            .expect("Failed to get page content");
        assert_eq!(revision_info.size, 114334);
        assert!(html.contains("<title>Rust (programming language)</title>"));
    }
}
