use serde_json::json;

use crate::{error::RestApiError, prelude::RestApi};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Transform;

impl Transform {
    /// Transforms wikitext to HTML.
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

    /// Transforms HTML to wikitext.
    pub async fn html2wikitext<S: Into<String>>(
        html: S,
        api: &RestApi,
    ) -> Result<String, RestApiError> {
        let path = "/transform/html/to/wikitext";
        let params = HashMap::new();
        let body = json!({
            "html": html.into()
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
            .insert(reqwest::header::ACCEPT, "text/plain".parse()?);

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
        let body = json!({
            "wikitext": wikitext
        });
        let expected_html: String =
            std::fs::read_to_string("test_data/wikitext2html.html").expect("Test file missing");

        // Set up mock server
        let mock_path = "w/rest.php/v1/transform/wikitext/to/html";
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(mock_path))
            .and(body_json(body))
            .and(header(reqwest::header::CONTENT_TYPE, "application/json"))
            .and(header(reqwest::header::ACCEPT, "text/html"))
            .respond_with(ResponseTemplate::new(200).set_body_string(&expected_html))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();

        // let api = RestApiBuilder::wikipedia("en").build();
        let html = Transform::wikitext2html(wikitext, &api)
            .await
            .expect("Failed to transform wikitext to HTML");
        assert_eq!(html, expected_html);
    }

    #[tokio::test]
    async fn test_html2wikitext() {
        let expected_wikitext = "[[Rust (programming language)|]]\n";

        // Set up mock server
        let html: String =
            std::fs::read_to_string("test_data/wikitext2html.html").expect("Test file missing");
        let mock_path = "w/rest.php/v1/transform/html/to/wikitext";
        let mock_server = MockServer::start().await;
        let body = json!({
            "html": html
        });
        Mock::given(method("POST"))
            .and(path(mock_path))
            .and(body_json(body))
            .and(header(reqwest::header::CONTENT_TYPE, "application/json"))
            .and(header(reqwest::header::ACCEPT, "text/plain"))
            .respond_with(ResponseTemplate::new(200).set_body_string(expected_wikitext))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();

        // let api = RestApiBuilder::wikipedia("en").build();
        let wikitext = Transform::html2wikitext(html, &api)
            .await
            .expect("Failed to transform wikitext to HTML");
        assert_eq!(wikitext, expected_wikitext);
    }
}
