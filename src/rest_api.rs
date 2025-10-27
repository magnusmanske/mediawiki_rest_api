use crate::{bearer_token::BearerToken, error::RestApiError, rest_api_builder::RestApiBuilder};
use reqwest::header::HeaderMap;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct RestApi {
    client: reqwest::Client,
    user_agent: String,
    api_url: String, // eg https://en.wikipedia.org/w/rest.php
    api_version: u8,
    pub token: Arc<RwLock<BearerToken>>,
}

// Public functions
impl RestApi {
    /// Returns a `RestApiBuilder`. Wrapper around `RestApiBuilder::new()`.
    pub fn builder<S: Into<String>>(api_url: S) -> Result<RestApiBuilder, RestApiError> {
        RestApiBuilder::new(api_url)
    }

    pub async fn get_edit_token(&self) -> Option<String> {
        self.token.read().await.get().to_owned()
    }

    /// Returns the user agent
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Returns the API version
    pub const fn api_version(&self) -> u8 {
        self.api_version
    }

    /// Returns the API URL
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Returns the `reqwest::Client`
    pub const fn client(&self) -> &reqwest::Client {
        &self.client
    }

    // ____________________________________________________________________________________________________
    // Crate-public functions

    /// Creates a new `RestApi` instance.
    /// Only available internally, use `RestApi::builder()` instead.
    pub(crate) const fn new(
        client: reqwest::Client,
        user_agent: String,
        api_url: String,
        api_version: u8,
        token: Arc<RwLock<BearerToken>>,
    ) -> Self {
        Self {
            client,
            user_agent,
            api_url,
            api_version,
            token,
        }
    }

    /// Returns a `RequestBuilder` for a Wikibase REST API request
    /// # Errors
    /// Returns an error if the headers cannot be created
    pub(crate) async fn build_request<S: Into<String>>(
        &self,
        path: S,
        params: HashMap<String, String>,
        method: reqwest::Method,
    ) -> Result<reqwest::RequestBuilder, RestApiError> {
        let mut headers = self.headers().await?;
        headers.insert(reqwest::header::ACCEPT, "application/json".parse()?);
        match method {
            reqwest::Method::GET => {}
            reqwest::Method::PATCH => {
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("json-patch+json"),
                );
            }
            _ => {
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
            }
        }
        let path: String = path.into();
        let wikibase_path = if path.contains("/v0/") {
            // Use verbatim path for odd, old, non-standard paths
            path
        } else {
            // Use auto-prefixed path
            format!("{}{}", self.mediawiki_root(), path)
        };
        self.request_builder(&wikibase_path, headers, params, method)
    }

    /// Executes a `reqwest::Request`, and returns a `reqwest::Response`.
    /// # Errors
    /// Returns an error if the request cannot be executed
    pub(crate) async fn execute(
        &self,
        request: reqwest::Request,
    ) -> Result<reqwest::Response, RestApiError> {
        self.token.write().await.check(self, &request).await?;
        let response = self.client.execute(request).await?;
        Ok(response)
    }

    /// Returns a `HeaderMap` with the user agent and `OAuth2` bearer token (if present).
    /// Only available internally.
    pub(crate) async fn headers_from_token(
        &self,
        token: &BearerToken,
    ) -> Result<HeaderMap, RestApiError> {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, self.user_agent.parse()?);
        if let Some(access_token) = &token.get() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {access_token}").parse()?,
            );
        }
        Ok(headers)
    }

    // ____________________________________________________________________________________________________
    // Private functions

    /// Returns a `HeaderMap` with the user agent and `OAuth2` bearer token (if present)
    async fn headers(&self) -> Result<HeaderMap, RestApiError> {
        let token = self.token.read().await;
        self.headers_from_token(&token).await
    }

    /// Returns the root path for the `MediaWiki` REST API, based on the version number
    fn mediawiki_root(&self) -> String {
        format!("/v{}", self.api_version)
    }

    /// Builds a `reqwest::RequestBuilder` from the method, client, path, and parameters
    fn request_builder<S: Into<String>>(
        &self,
        path: S,
        headers: HeaderMap,
        params: HashMap<String, String>,
        method: reqwest::Method,
    ) -> Result<reqwest::RequestBuilder, RestApiError> {
        let url = format!("{}{}", self.api_url, path.into());
        Ok(match method {
            reqwest::Method::GET => self.client.get(url).headers(headers).query(&params),
            reqwest::Method::POST => self.client.post(url).headers(headers).form(&params),
            reqwest::Method::PATCH => self.client.patch(url).headers(headers).form(&params),
            reqwest::Method::PUT => self.client.put(url).headers(headers).form(&params),
            reqwest::Method::DELETE => self.client.delete(url).headers(headers).form(&params),
            _ => return Err(RestApiError::UnsupportedMethod(method)),
        })
    }
}
