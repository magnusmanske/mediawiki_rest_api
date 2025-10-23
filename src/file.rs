use std::collections::HashMap;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct File {
    title: String,
}

impl File {
    /// Creates a new file object with the given title.
    /// Note: the title is expected to be without the "File:" prefix.
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Retrieves file information.
    pub async fn get(&self, api: &RestApi) -> Result<FileInfo, RestApiError> {
        let path = format!("/file/{}", self.title);
        let params = HashMap::new();
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: FileInfo = response.json().await?;
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
        let (api, _mock_server) = get_mock_api("file_get.json", "/file/Commons-logo.svg").await;
        let page = File::new("Commons-logo.svg");
        let file_info = page.get(&api).await.expect("Failed to get page content");
        assert_eq!(
            file_info.file_description_url,
            "//en.wikipedia.org/wiki/File:Commons-logo.svg"
        );
    }
}
// Commons-logo.svg
