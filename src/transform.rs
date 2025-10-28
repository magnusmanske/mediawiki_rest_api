use crate::{
    error::RestApiError,
    prelude::{Lint, RestApi},
};
use serde_json::json;
use std::collections::HashMap;
use urlencoding::encode;

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
            .build_request(path, params, reqwest::Method::POST)
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

    /// Transforms wikitext to HTML, using a title for context.
    pub async fn wikitext2html_title<S1: Into<String>, S2: Into<String>>(
        wikitext: S1,
        title: S2,
        api: &RestApi,
    ) -> Result<String, RestApiError> {
        let params = HashMap::new();
        let wikitext: String = wikitext.into();
        let body = json!({
            "wikitext": wikitext
        })
        .to_string();
        let path = format!("/transform/wikitext/to/html/{}", encode(&title.into()));
        let mut request = api
            .build_request(path, params, reqwest::Method::POST)
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
            .build_request(path, params, reqwest::Method::POST)
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

    /// Transforms HTML to wikitext, using a title for context.
    pub async fn html2wikitext_title<S1: Into<String>, S2: Into<String>>(
        html: S1,
        title: S2,
        api: &RestApi,
    ) -> Result<String, RestApiError> {
        let path = format!("/transform/html/to/wikitext/{}", encode(&title.into()));
        let params = HashMap::new();
        let body = json!({
            "html": html.into()
        })
        .to_string();
        let mut request = api
            .build_request(path, params, reqwest::Method::POST)
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

    /// Returns lint errors for wikitext.
    pub async fn wikitext2lint<S: Into<String>>(
        wikitext: S,
        api: &RestApi,
    ) -> Result<Vec<Lint>, RestApiError> {
        let path = "/transform/wikitext/to/lint";
        let params = HashMap::new();
        let body = json!({
            "wikitext": wikitext.into()
        })
        .to_string();
        let mut request = api
            .build_request(path, params, reqwest::Method::POST)
            .await?
            .body(body)
            .build()?;
        request.headers_mut().remove(reqwest::header::ACCEPT);
        request
            .headers_mut()
            .insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);
        let response = api.execute(request).await?;
        let ret: Vec<Lint> = response.json().await?;
        Ok(ret)
    }

    /// Returns lint errors for wikitext.
    pub async fn wikitext2lint_title<S1: Into<String>, S2: Into<String>>(
        title: S1,
        wikitext: S2,
        api: &RestApi,
    ) -> Result<Vec<Lint>, RestApiError> {
        let path = format!("/transform/wikitext/to/lint/{}", encode(&title.into()));
        let params = HashMap::new();
        let body = json!({
            "wikitext": wikitext.into()
        })
        .to_string();
        let mut request = api
            .build_request(path, params, reqwest::Method::POST)
            .await?
            .body(body)
            .build()?;
        request.headers_mut().remove(reqwest::header::ACCEPT);
        request
            .headers_mut()
            .insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);
        let response = api.execute(request).await?;
        let ret: Vec<Lint> = response.json().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
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

        let html = Transform::wikitext2html(wikitext, &api)
            .await
            .expect("Failed to transform wikitext to HTML");
        assert_eq!(html, expected_html);
    }

    #[tokio::test]
    async fn test_wikitext2html_title() {
        let wikitext = "!{{FULLPAGENAME}}?";
        let body = json!({
            "wikitext": wikitext
        });
        let expected_html: String = std::fs::read_to_string("test_data/wikitext2html_title.html")
            .expect("Test file missing");

        // Set up mock server
        let mock_path = format!(
            "w/rest.php/v1/transform/wikitext/to/html/{}",
            encode("Talk:Foo/Bar")
        );
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

        // let api = crate::rest_api_builder::RestApiBuilder::wikipedia("en").build();
        let html = Transform::wikitext2html_title(wikitext, "Talk:Foo/Bar", &api)
            .await
            .expect("Failed to transform wikitext to HTML");
        assert_eq!(html.trim(), expected_html.trim());
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

        let wikitext = Transform::html2wikitext(html, &api)
            .await
            .expect("Failed to transform wikitext to HTML");
        assert_eq!(wikitext, expected_wikitext);
    }

    #[tokio::test]
    async fn test_html2wikitext_title() {
        let expected_wikitext = "!{{FULLPAGENAME}}?";
        let title = "Talk:Foo/Bar";

        // Set up mock server
        let html: String = std::fs::read_to_string("test_data/wikitext2html_title.html")
            .expect("Test file missing");
        let mock_path = format!("w/rest.php/v1/transform/html/to/wikitext/{}", encode(title));

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

        let wikitext = Transform::html2wikitext_title(html, title, &api)
            .await
            .expect("Failed to transform wikitext to HTML");
        assert_eq!(wikitext.trim(), expected_wikitext.trim());
    }

    #[tokio::test]
    async fn test_html2lint() {
        let wikitext = "== Hello Jupiter ==[[Link|text]]<table><table>";

        // Set up mock server
        let json_text: String =
            std::fs::read_to_string("test_data/wikitext2lint.json").expect("Test file missing");
        let json_result: Value = serde_json::from_str(&json_text).expect("Failed to parse JSON");
        let mock_path = "w/rest.php/v1/transform/wikitext/to/lint";

        let mock_server = MockServer::start().await;
        let body = json!({
            "wikitext": wikitext
        });
        Mock::given(method("POST"))
            .and(path(mock_path))
            .and(body_json(body))
            .and(header(reqwest::header::CONTENT_TYPE, "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json_result))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();

        let result = Transform::wikitext2lint(wikitext, &api)
            .await
            .expect("transform failed");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].type_name, "missing-end-tag");
    }

    #[tokio::test]
    async fn test_html2lint_title() {
        let title = "Cambridge";
        let wikitext = "== Hello Jupiter ==[[Link|text]]<table><table>";

        // Set up mock server
        let json_text: String =
            std::fs::read_to_string("test_data/wikitext2lint.json").expect("Test file missing");
        let json_result: Value = serde_json::from_str(&json_text).expect("Failed to parse JSON");
        let mock_path = format!("w/rest.php/v1/transform/wikitext/to/lint/{}", encode(title));

        let mock_server = MockServer::start().await;
        let body = json!({
            "wikitext": wikitext
        });
        Mock::given(method("POST"))
            .and(path(mock_path))
            .and(body_json(body))
            .and(header(reqwest::header::CONTENT_TYPE, "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json_result))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();

        let result = Transform::wikitext2lint_title(title, wikitext, &api)
            .await
            .expect("transform failed");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].type_name, "missing-end-tag");
    }
}
