use std::collections::HashMap;

use crate::{error::RestApiError, prelude::RestApi};

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

    pub async fn get(&self, api: &RestApi, follow_redirect: bool) -> Result<String, RestApiError> {
        let path = format!("/page/{}", self.title);
        let mut params = HashMap::new();
        params.insert("redirect".to_string(), follow_redirect.to_string());
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret = response.text().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::rest_api_builder::RestApiBuilder;

    use super::*;

    #[tokio::test]
    async fn test_get_page() {
        let api = RestApiBuilder::wikipedia("en")
            .expect("Failed to build API")
            .build();
        let page = Page::new("Rust (programming language)");
        let content = page
            .get(&api, false)
            .await
            .expect("Failed to get page content");
        assert!(content.contains("Mozilla sponsorship"));
    }
}
