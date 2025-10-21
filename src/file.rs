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
    use crate::rest_api_builder::RestApiBuilder;

    #[tokio::test]
    async fn test_get() {
        let api = RestApiBuilder::wikipedia("en").build();
        let page = File::new("Commons-logo.svg");
        let file_info = page.get(&api).await.expect("Failed to get page content");
        assert_eq!(
            file_info.file_description_url,
            "//en.wikipedia.org/wiki/File:Commons-logo.svg"
        );
    }
}
// Commons-logo.svg
