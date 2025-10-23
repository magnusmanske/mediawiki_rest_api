use crate::{
    error::RestApiError,
    prelude::{RestApi, SearchResults},
};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Search;

impl Search {
    pub async fn search<S: Into<String>>(_ctype: S, _api: &RestApi) -> Result<(), RestApiError> {
        unimplemented!()
    }

    pub async fn page<S: Into<String>>(
        query: S,
        limit: Option<usize>,
        api: &RestApi,
    ) -> Result<SearchResults, RestApiError> {
        let path = "/search/page";
        let mut params = HashMap::new();
        params.insert("q".to_string(), query.into());
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        let request = api
            .mediawiki_request_builder(path, params, reqwest::Method::GET)
            .await?
            .build()?;
        let response = api.execute(request).await?;
        let ret: SearchResults = response.json().await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // async fn get_mock_api(test_file: &str, test_path: &str) -> (RestApi, MockServer) {
    //     let mock_path = format!("w/rest.php/v1{}", test_path.replace(' ', "%20"));
    //     let mock_server = MockServer::start().await;

    //     let test_text: String =
    //         std::fs::read_to_string(format!("test_data/{test_file}")).expect("Test file missing");
    //     if test_file.ends_with(".json") {
    //         let json: Value = serde_json::from_str(&test_text).expect("Failed to parse JSON");
    //         Mock::given(method("GET"))
    //             .and(path(&mock_path))
    //             .respond_with(ResponseTemplate::new(200).set_body_json(&json))
    //             .mount(&mock_server)
    //             .await;
    //     } else {
    //         Mock::given(method("GET"))
    //             .and(path(&mock_path))
    //             .respond_with(ResponseTemplate::new(200).set_body_string(&test_text))
    //             .mount(&mock_server)
    //             .await;
    //     }

    //     let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
    //         .expect("Failed to create RestApi")
    //         .build();
    //     (api, mock_server)
    // }

    #[tokio::test]
    async fn test_search_page() {
        let query = "Rust programming language";
        let test_text: String =
            std::fs::read_to_string("test_data/search_page.json").expect("Test file missing");
        let json: Value = serde_json::from_str(&test_text).expect("Failed to parse JSON");

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("w/rest.php/v1/search/page"))
            .and(query_param("q", query))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json))
            .mount(&mock_server)
            .await;
        let api = RestApi::builder(&(mock_server.uri() + "/w/rest.php"))
            .expect("Failed to create RestApi")
            .build();

        // let api = crate::rest_api_builder::RestApiBuilder::wikipedia("en").build();
        let results = Search::page(query, None, &api)
            .await
            .expect("Failed to get page content");
        assert_eq!(results.pages.len(), 50);
    }
}
