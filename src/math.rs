use crate::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Math;

impl Math {
    /// Retrieves the HTML popup information from Wikidata for a given QID.
    pub async fn popup_html(qid: usize, api: &RestApi) -> Result<PopupInfo, RestApiError> {
        let path = format!("/math/v0/popup/html/{qid}");
        let params = HashMap::new();
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: PopupInfo = response.json().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn get_mock_api(test_file: &str, test_path: &str) -> (RestApi, MockServer) {
        let mock_path = format!("w/rest.php/{}", test_path.replace(' ', "%20"));
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
    async fn test_popup_html() {
        let (api, _mock_server) =
            get_mock_api("math_popup_html.json", "math/v0/popup/html/12345").await;
        // let api = crate::rest_api_builder::RestApiBuilder::wikipedia("en").build();
        let popup = Math::popup_html(12345, &api)
            .await
            .expect("Failed to get page content");
        assert_eq!(popup.title, "Count von Count");
    }
}
