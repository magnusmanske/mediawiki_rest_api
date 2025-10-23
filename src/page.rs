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

    /// Retrieves history data for the page.
    pub async fn get_history(
        &self,
        api: &RestApi,
        filter: Option<Filter>,
        older_than: Option<usize>,
        newer_than: Option<usize>,
    ) -> Result<History, RestApiError> {
        let path = format!("/page/{}/history", self.title);
        let mut params = HashMap::new();
        if let Some(older_than) = older_than {
            params.insert("older_than".to_string(), older_than.to_string());
        }
        if let Some(newer_than) = newer_than {
            params.insert("newer_than".to_string(), newer_than.to_string());
        }
        if let Some(filter) = filter {
            params.insert("filter".to_string(), filter.to_string());
        }
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: History = response.json().await?;
        Ok(ret)
    }

    /// Retrieves history counts for the page.
    pub async fn get_history_counts(
        &self,
        api: &RestApi,
        filter: HistoryFilterExtended,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<HistoryCounts, RestApiError> {
        let path = format!("/page/{}/history/counts/{filter}", self.title);
        let mut params = HashMap::new();
        if let Some(from) = from {
            params.insert("from".to_string(), from.to_string());
        }
        if let Some(to) = to {
            params.insert("to".to_string(), to.to_string());
        }
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: HistoryCounts = response.json().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn get_mock_api(test_file: &str, test_path: &str) -> (RestApi, MockServer) {
        let mock_path = format!("w/rest.php/v1{}", test_path.replace(' ', "%20"));
        let mock_server = MockServer::start().await;

        let test_text: String =
            std::fs::read_to_string(format!("test_data/{test_file}")).expect("Test file missing");
        if test_file.ends_with(".json") {
            let json: Value = serde_json::from_str(&test_text).expect("Failed to parse JSON");
            Mock::given(method("GET"))
                .and(path(&mock_path))
                .respond_with(ResponseTemplate::new(200).set_body_json(&json))
                .mount(&mock_server)
                .await;
        } else {
            Mock::given(method("GET"))
                .and(path(&mock_path))
                .respond_with(ResponseTemplate::new(200).set_body_string(&test_text))
                .mount(&mock_server)
                .await;
        }

        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();
        (api, mock_server)
    }

    #[tokio::test]
    async fn test_get() {
        let (api, _mock_server) =
            get_mock_api("page_get.json", "/page/Rust (programming language)").await;
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
        let (api, _mock_server) = get_mock_api(
            "page_get_bare.json",
            "/page/Rust (programming language)/bare",
        )
        .await;
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
        let (api, _mock_server) = get_mock_api(
            "page_get_html.html",
            "/page/Rust (programming language)/html",
        )
        .await;
        let page = Page::new("Rust (programming language)");
        let result = page
            .get_html(&api, false, false, HtmlFlavor::View)
            .await
            .expect("Failed to get page content");
        assert!(result.contains("<title>Rust (programming language)</title>"));
    }

    #[tokio::test]
    async fn test_get_with_html() {
        let (api, _mock_server) = get_mock_api(
            "page_get_with_html.json",
            "/page/Rust (programming language)/with_html",
        )
        .await;
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
        let (api, _mock_server) = get_mock_api(
            "page_links_language.json",
            "/page/Rust (programming language)/links/language",
        )
        .await;
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
        let (api, _mock_server) =
            get_mock_api("page_links_media.json", "/page/Cambridge/links/media").await;
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
        let (api, _mock_server) = get_mock_api("page_lint.json", "/page/Cambridge/lint").await;
        let page = Page::new("Cambridge");
        let lints = page
            .get_lint(&api, false)
            .await
            .expect("Failed to get page content");
        assert_eq!(lints.len(), 9);
        assert!(lints.iter().any(|lint| lint.type_name == "duplicate-ids"
            && lint.template_info.name == "Template:Cite_book"));
    }

    #[tokio::test]
    async fn test_get_history() {
        let (api, _mock_server) = get_mock_api(
            "page_history.json",
            "/page/Rust (programming language)/history",
        )
        .await;
        let page = Page::new("Rust (programming language)");
        let history = page
            .get_history(&api, None, None, None)
            .await
            .expect("Failed to get page content");
        assert_eq!(history.revisions.len(), 20);
    }

    #[tokio::test]
    async fn test_get_history_counts() {
        let (api, _mock_server) = get_mock_api(
            "page_history_counts.json",
            "/page/Cambridge/history/counts/anonymous",
        )
        .await;
        let page = Page::new("Cambridge");
        let hc = page
            .get_history_counts(&api, HistoryFilterExtended::Anonymous, None, None)
            .await
            .expect("Failed to get page content");
        assert_eq!(hc.count, 1289);
    }
}
