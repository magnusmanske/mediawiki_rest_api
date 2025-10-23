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
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_wikitext2html() {
        let wikitext = "[[Rust (programming language)|]]";

        // Set up mock server
        let v: String =
            std::fs::read_to_string("test_data/wikitext2html.html").expect("Test file missing");
        let mock_path = "w/rest.php/v1/transform/wikitext/to/html";
        let mock_server = MockServer::start().await;
        let body = json!({
            "wikitext": wikitext
        });
        Mock::given(method("POST"))
            .and(path(mock_path))
            .and(body_json(body))
            .and(header(reqwest::header::CONTENT_TYPE, "application/json"))
            .and(header(reqwest::header::ACCEPT, "text/html"))
            .respond_with(ResponseTemplate::new(200).set_body_string(v))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();

        // let api = RestApiBuilder::wikipedia("en").build();
        let html = Transform::wikitext2html(wikitext, &api)
            .await
            .expect("Failed to transform wikitext to HTML");
        assert!(html.contains(">[[Rust (programming language)|]]</p></section></body></html>"));
    }
}
