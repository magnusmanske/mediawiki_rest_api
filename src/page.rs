use crate::{error::RestApiError, prelude::*};
use serde::Deserialize;
use serde_json::{Value, from_value};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct PageInfo {
    pub id: usize,
    pub key: String,
    pub title: String,
    pub latest: RevisionTimestamp,
    pub content_model: String,
    pub license: LicenseModel,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MediaResult {
    pub files: Vec<FileInfo>,
}

#[derive(Clone, Debug)]
pub struct Page {
    title: String,
}

impl Page {
    /// Creates a new page object with the given title.
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Retrieves basic page information and wikitext.
    pub async fn get(
        &self,
        api: &RestApi,
        follow_redirect: bool,
    ) -> Result<(PageInfo, String), RestApiError> {
        let path = format!("/page/{}", self.title);
        let mut params = HashMap::new();
        params.insert("redirect".to_string(), follow_redirect.to_string());
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
        let ret = from_value::<PageInfo>(j)?;
        Ok((ret, wikitext))
    }

    /// Retrieves basic page information and the URL for HTML retrieval.
    pub async fn get_bare(
        &self,
        api: &RestApi,
        follow_redirect: bool,
    ) -> Result<(PageInfo, String), RestApiError> {
        let path = format!("/page/{}/bare", self.title);
        let mut params = HashMap::new();
        params.insert("redirect".to_string(), follow_redirect.to_string());
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
        let ret = from_value::<PageInfo>(j)?;
        Ok((ret, html_url))
    }

    /// Retrieves the HTML for the page.
    pub async fn get_html(
        &self,
        api: &RestApi,
        follow_redirect: bool,
        stash: bool,
        flavor: HtmlFlavor,
    ) -> Result<String, RestApiError> {
        let path = format!("/page/{}/html", self.title);
        let mut params = HashMap::new();
        params.insert("redirect".to_string(), follow_redirect.to_string());
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

    /// Retrieves basic page information and the HTML for the page.
    pub async fn get_with_html(
        &self,
        api: &RestApi,
        follow_redirect: bool,
        stash: bool,
        flavor: HtmlFlavor,
    ) -> Result<(PageInfo, String), RestApiError> {
        let path = format!("/page/{}/with_html", self.title);
        let mut params = HashMap::new();
        params.insert("redirect".to_string(), follow_redirect.to_string());
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
        let ret = from_value::<PageInfo>(j)?;
        Ok((ret, html))
    }

    /// Retrieves the language links.
    pub async fn get_links_language(
        &self,
        api: &RestApi,
    ) -> Result<Vec<LanguageLink>, RestApiError> {
        let path = format!("/page/{}/links/language", self.title);
        let params = HashMap::new();
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: Vec<LanguageLink> = response.json().await?;
        Ok(ret)
    }

    /// Retrieves the used media.
    pub async fn get_links_media(&self, api: &RestApi) -> Result<MediaResult, RestApiError> {
        let path = format!("/page/{}/links/media", self.title);
        let params = HashMap::new();
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: MediaResult = response.json().await?;
        Ok(ret)
    }

    /// Retrieves lint data for the page.
    pub async fn get_lint(
        &self,
        api: &RestApi,
        follow_redirect: bool,
    ) -> Result<Vec<Lint>, RestApiError> {
        let path = format!("/page/{}/lint", self.title);
        let mut params = HashMap::new();
        params.insert("redirect".to_string(), follow_redirect.to_string());
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: Vec<Lint> = response.json().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest_api_builder::RestApiBuilder;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get() {
        let api = RestApiBuilder::wikipedia("en").build();
        let page = Page::new("Rust (programming language)");
        let (page_info, wikitext) = page
            .get(&api, false)
            .await
            .expect("Failed to get page content");
        assert_eq!(page_info.id, 29414838);
        assert!(wikitext.contains("Mozilla sponsorship"));
    }

    #[tokio::test]
    async fn test_get_bare() {
        let api = RestApiBuilder::wikipedia("en").build();
        let page = Page::new("Rust (programming language)");
        let (page_info, html_url) = page
            .get_bare(&api, false)
            .await
            .expect("Failed to get page content");
        assert_eq!(page_info.id, 29414838);
        assert_eq!(
            html_url,
            "https://en.wikipedia.org/w/rest.php/v1/page/Rust%20%28programming%20language%29/html"
        );
    }

    #[tokio::test]
    async fn test_get_html() {
        let api = RestApiBuilder::wikipedia("en").build();
        let page = Page::new("Rust (programming language)");
        let result = page
            .get_html(&api, false, false, HtmlFlavor::View)
            .await
            .expect("Failed to get page content");
        assert!(result.contains("<title>Rust (programming language)</title>"));
    }

    #[tokio::test]
    async fn test_get_with_html() {
        let api = RestApiBuilder::wikipedia("en").build();
        let page = Page::new("Rust (programming language)");
        let (page_info, html) = page
            .get_with_html(&api, false, false, HtmlFlavor::View)
            .await
            .expect("Failed to get page content");
        assert_eq!(page_info.id, 29414838);
        assert!(html.contains("<title>Rust (programming language)</title>"));
    }

    #[tokio::test]
    async fn test_get_links_language() {
        let api = RestApiBuilder::wikipedia("en").build();
        let page = Page::new("Rust (programming language)");
        let language_links = page
            .get_links_language(&api)
            .await
            .expect("Failed to get page content");
        assert!(
            language_links.iter().any(
                |link| link.code == "it" && link.title == "Rust (linguaggio di programmazione)"
            )
        );
    }

    #[tokio::test]
    async fn test_get_links_media() {
        let api = RestApiBuilder::wikipedia("en").build();
        let page = Page::new("Cambridge");
        let media_links = page
            .get_links_media(&api)
            .await
            .expect("Failed to get page content");
        assert!(
            media_links
                .files
                .iter()
                .any(|file| file.title == "Flag of England.svg")
        );
    }

    #[tokio::test]
    async fn test_get_lint() {
        let v: String =
            std::fs::read_to_string("test_data/page_lint.json").expect("Test file missing");
        let v: Value = serde_json::from_str(&v).expect("Failed to parse JSON");

        let mock_path = "w/rest.php/v1/page/Cambridge/lint";
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(mock_path))
            .respond_with(ResponseTemplate::new(200).set_body_json(&v))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();

        let page = Page::new("Cambridge");
        let lints = page
            .get_lint(&api, false)
            .await
            .expect("Failed to get page content");
        assert_eq!(lints.len(), 9);
        assert!(lints.iter().any(|lint| lint.type_name == "duplicate-ids"
            && lint.template_info.name == "Template:Cite_book"));
    }
}
