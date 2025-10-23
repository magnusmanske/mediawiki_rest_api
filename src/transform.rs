use serde_json::json;

use crate::{error::RestApiError, prelude::RestApi};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Transform;

impl Transform {
    pub async fn wikitext2html<S: Into<String>>(
        wikitext: S,
        api: &RestApi,
    ) -> Result<String, RestApiError> {
        let path = "/transform/wikitext/to/html";
        let params = HashMap::new();
        let body = json!({
            "wikitext": wikitext.into()
        })
        .to_string();
        let mut request = api
            .mediawiki_request_builder(path, params, reqwest::Method::POST)
            .await?
            .body(body)
            .build()?;
        request
            .headers_mut()
            .insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);
        request
            .headers_mut()
            .insert(reqwest::header::ACCEPT, "text/html".parse()?);

        let response = api.execute(request).await?;
        let ret: String = response.text().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest_api_builder::RestApiBuilder;
    // use wiremock::matchers::{method, path};
    // use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_wikitext2html() {
        let api = RestApiBuilder::wikipedia("en").build();
        let wikitext = "[[Rust (programming language)|]]";
        let html = Transform::wikitext2html(wikitext, &api)
            .await
            .expect("Failed to transform wikitext to HTML");
        assert!(html.contains(">[[Rust (programming language)|]]</p></section></body></html>"));
    }
}
