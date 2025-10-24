use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{bearer_token::BearerToken, error::RestApiError, prelude::RestApi};

/// The default user agent
const DEFAULT_USER_AGENT: &str = "Rust MediaWiki REST API client";

/// The latest supported version of the Wikibase REST API
const WIKIBASE_REST_API_VERSION: u8 = 1;

#[derive(Debug)]
pub struct RestApiBuilder {
    client: Option<reqwest::Client>,
    token: BearerToken,
    user_agent: Option<String>,
    api_url: String,
    api_version: Option<u8>,
    renewal_interval: Option<std::time::Duration>,
}

// Public functions
impl RestApiBuilder {
    /// Sets the REST API URL, specifically the URL ending in "rest.php". This in mandatory.
    /// # Errors
    /// Returns an error if REST API URL is invalid.
    pub fn new<S: Into<String>>(api_url: S) -> Result<Self, RestApiError> {
        let api_url = Self::validate_api_url(&api_url.into())?;
        Ok(Self::new_from_validated(api_url))
    }

    /// Convenience function for a specific Wikimedia wiki group
    pub fn wikipedia(language: &str) -> Self {
        Self::new_from_validated(format!("https://{language}.wikipedia.org/w/rest.php"))
    }

    /// Convenience function for a specific Wikimedia wiki group
    pub fn wikitionary(language: &str) -> Self {
        Self::new_from_validated(format!("https://{language}.wikitionary.org/w/rest.php"))
    }

    /// Convenience function for a specific Wikimedia wiki group
    pub fn wikivoyage(language: &str) -> Self {
        Self::new_from_validated(format!("https://{language}.wikivoyage.org/w/rest.php"))
    }

    /// Convenience function for a specific Wikimedia wiki group
    pub fn wikibooks(language: &str) -> Self {
        Self::new_from_validated(format!("https://{language}.wikibooks.org/w/rest.php"))
    }

    /// Convenience function for a specific Wikimedia wiki group
    pub fn wikinews(language: &str) -> Self {
        Self::new_from_validated(format!("https://{language}.wikinews.org/w/rest.php"))
    }

    /// Convenience function for a specific Wikimedia wiki group
    pub fn wikisource(language: &str) -> Self {
        Self::new_from_validated(format!("https://{language}.wikisource.org/w/rest.php"))
    }

    /// Convenience function for a specific Wikimedia wiki group
    pub fn wikiversity(language: &str) -> Self {
        Self::new_from_validated(format!("https://{language}.wikiversity.org/w/rest.php"))
    }

    /// Convenience function for a specific Wikimedia wiki group
    pub fn wikiquote(language: &str) -> Self {
        Self::new_from_validated(format!("https://{language}.wikiquote.org/w/rest.php"))
    }

    /// Convenience function for a specific Wikimedia wiki
    pub fn commons() -> Self {
        Self::new_from_validated("https://commons.wikimedia.org/w/rest.php")
    }

    /// Convenience function for a specific Wikimedia wiki
    pub fn wikidata() -> Self {
        Self::new_from_validated("https://www.wikidata.org/w/rest.php")
    }

    /// Convenience function for a specific Wikimedia wiki
    pub fn wikispecies() -> Self {
        Self::new_from_validated("https://species.wikimedia.org/w/rest.php")
    }

    /// Convenience function for a specific Wikimedia wiki
    pub fn meta() -> Self {
        Self::new_from_validated("https://meta.wikimedia.org/w/rest.php")
    }

    /// Builds the `RestApi`. Returns an error if no REST API URL is set.
    /// The builder gets consumed by this operation.
    /// # Returns
    /// Returns a `RestApi` instance.
    pub fn build(self) -> RestApi {
        let api_url = self.api_url;
        let mut token = self.token;
        if let Some(interval) = self.renewal_interval {
            token.set_renewal_interval(interval.as_secs());
        }
        let token = Arc::new(RwLock::new(token));
        let user_agent = self.user_agent.unwrap_or(Self::default_user_agent());
        let api_version = self.api_version.unwrap_or(WIKIBASE_REST_API_VERSION);
        let client = self.client.unwrap_or_default();
        RestApi::new(client, user_agent, api_url, api_version, token)
    }

    /// Sets the `OAuth2` bearer token.
    pub fn with_access_token<S: Into<String>>(mut self, access_token: S) -> Self {
        self.token.set_access_token(access_token);
        self
    }

    /// Sets the `OAuth2` client ID and client secret
    #[cfg(not(tarpaulin_include))]
    pub fn with_oauth2_info<S1: Into<String>, S2: Into<String>>(
        mut self,
        client_id: S1,
        client_secret: S2,
    ) -> Self {
        self.token.set_oauth2_info(client_id, client_secret);
        self
    }

    /// Sets the interval for bearer token renewal. By default, the interval is `DEFAULT_RENEWAL_INTERVAL_SEC`.
    #[cfg(not(tarpaulin_include))]
    pub const fn with_access_token_renewal(
        mut self,
        renewal_interval: std::time::Duration,
    ) -> Self {
        self.renewal_interval = Some(renewal_interval);
        self
    }

    /// Sets the API version (u8). Default is `WIKIBASE_REST_API_VERSION`.
    pub const fn with_api_version(mut self, api_version: u8) -> Self {
        self.api_version = Some(api_version);
        self
    }

    /// Sets the user agent. By default, the user agent is "Rust Wikibase REST API; {`package_name`}/{`package_version`}"
    pub fn with_user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sets the `reqwest::Client`. By default, a new `reqwest::Client` is created.
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    // ____________________________________________________________________________________________________
    // Private functions

    /// Checks if the REST API URL is valid. The URL must end in "rest.php".
    /// Removes anything beyone that.
    fn validate_api_url(api_url: &str) -> Result<String, RestApiError> {
        let (base, _rest) = api_url
            .split_once("/rest.php")
            .ok_or_else(|| RestApiError::RestApiUrlInvalid(api_url.to_owned()))?;
        let ret = format!("{base}/rest.php");
        Ok(ret)
    }

    /// Returns the default user agent, a versioned string based on `DEFAULT_USER_AGENT`.
    fn default_user_agent() -> String {
        format!(
            "{DEFAULT_USER_AGENT}; {}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    fn new_from_validated<S: Into<String>>(api_url: S) -> Self {
        Self {
            client: None,
            token: BearerToken::default(),
            user_agent: None,
            api_url: api_url.into(),
            api_version: None,
            renewal_interval: None,
        }
    }
}
