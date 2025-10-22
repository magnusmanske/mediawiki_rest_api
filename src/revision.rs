use std::collections::HashMap;

use serde_json::{Value, from_value};

use crate::{
    error::RestApiError,
    prelude::{Diff, HtmlFlavor, Lint, RestApi, RevisionInfo},
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

    /// Retrieves basic revision information and the link to the HTML.
    pub async fn get_bare(&self, api: &RestApi) -> Result<(RevisionInfo, String), RestApiError> {
        let path = format!("/revision/{}/bare", self.id);
        let params = HashMap::new();
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let j: Value = response.json().await?;
        let html_url = j["html_url"]
            .as_str()
            .ok_or(RestApiError::MissingResults)?
            .to_string();
        let ret = from_value::<RevisionInfo>(j)?;
        Ok((ret, html_url))
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

    /// Retrieves lint data for the revision.
    pub async fn get_lint(&self, api: &RestApi) -> Result<Vec<Lint>, RestApiError> {
        let path = format!("/revision/{}/lint", self.id);
        let params = HashMap::new();
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: Vec<Lint> = response.json().await?;
        Ok(ret)
    }

    pub async fn get_compare(&self, api: &RestApi, to: usize) -> Result<Diff, RestApiError> {
        let path = format!("/revision/{}/compare/{to}", self.id);
        let params = HashMap::new();
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: Diff = response.json().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest_api_builder::RestApiBuilder;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_REVISION_ID: usize = 1316925953;
    const TEST_REVISION_OLD_ID: usize = 1316608902;

    #[tokio::test]
    async fn test_get() {
        let api = RestApiBuilder::wikipedia("en").build();
        let revision = Revision::new(TEST_REVISION_ID);
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
        let revision = Revision::new(TEST_REVISION_ID);
        let html = revision
            .get_html(&api, false, HtmlFlavor::View)
            .await
            .expect("Failed to get page content");
        assert!(html.contains("<title>Rust (programming language)</title>"));
    }

    #[tokio::test]
    async fn test_get_with_html() {
        let api = RestApiBuilder::wikipedia("en").build();
        let revision = Revision::new(TEST_REVISION_ID);
        let (revision_info, html) = revision
            .get_with_html(&api, false, HtmlFlavor::View)
            .await
            .expect("Failed to get page content");
        assert_eq!(revision_info.size, 114334);
        assert!(html.contains("<title>Rust (programming language)</title>"));
    }

    #[tokio::test]
    async fn test_get_bare() {
        let api = RestApiBuilder::wikipedia("en").build();
        let revision = Revision::new(TEST_REVISION_ID);
        let (revision_info, html_url) = revision
            .get_bare(&api)
            .await
            .expect("Failed to get page content");
        assert_eq!(revision_info.size, 114334);
        assert_eq!(
            html_url,
            "https://en.wikipedia.org/w/rest.php/v1/revision/1316925953/html"
        );
    }

    #[tokio::test]
    async fn test_get_compare() {
        let v: String =
            std::fs::read_to_string("test_data/revision_compare.json").expect("Test file missing");
        let v: Value = serde_json::from_str(&v).expect("Failed to parse JSON");

        let mock_path =
            format!("w/rest.php/v1/revision/{TEST_REVISION_ID}/compare/{TEST_REVISION_OLD_ID}");
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(mock_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(&v))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();
        let revision = Revision::new(TEST_REVISION_ID);
        let result = revision
            .get_compare(&api, TEST_REVISION_OLD_ID)
            .await
            .expect("Failed to get page content");
        assert_eq!(result.diff.len(), 6);
        assert_eq!(result.from.sections.len(), 52);
        assert_eq!(result.to.sections.len(), 52);
    }

    #[tokio::test]
    async fn test_get_lint() {
        let v: String =
            std::fs::read_to_string("test_data/revision_lint.json").expect("Test file missing");
        let v: Value = serde_json::from_str(&v).expect("Failed to parse JSON");

        let mock_path = format!("w/rest.php/v1/revision/{TEST_REVISION_ID}/lint");
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(mock_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(&v))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();

        let page = Revision::new(TEST_REVISION_ID);
        let lints = page
            .get_lint(&api)
            .await
            .expect("Failed to get page content");
        assert_eq!(lints.len(), 14);
        assert!(lints.iter().any(|lint| lint.type_name == "duplicate-ids"
            && lint.template_info.name == "Template:Cite_web"));
    }
}
