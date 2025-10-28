[![Crates.io](https://img.shields.io/crates/v/mediawiki_rest_api?style=flat-square)](https://crates.io/crates/mediawiki_rest_api)
[![Crates.io](https://img.shields.io/crates/d/mediawiki_rest_api?style=flat-square)](https://crates.io/crates/mediawiki_rest_api)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![License](https://img.shields.io/badge/license-APACHE2-blue?style=flat-square)](LICENSE-APACHE2)

This Rust crate provides a client for the [MediaWiki REST API](https://www.mediawiki.org/wiki/API:REST_API).

# Usage
```rust
use mediawiki_rest_api::prelude::*;

// Create a client for the English Wikipedia REST API
let api = RestApiBuilder::wikipedia("en").build();

// From the page "Rust (programming language)"...
let page = Page::new("Rust (programming language)");

// ...retrieve basic information and wikitext
let (page_info,wikitext) = page.get(&api, false).await.unwrap();

// Edit an existing page
let token = "my_oauth_token";
let api = RestApiBuilder::wikipedia("en")
    .with_access_token(token)
    .build();
let (page_info,_) = page.get(&api, false).await.unwrap();
page.edit(&api, &page_info.latest, "new page wikitext", "a comment")
    .await
    .expect("Failed to edit page");

// Convert some wikitext to Parsoid HTML.
let html = Transform::wikitext2html("[[Foo|bar]]", &api).await.unwrap();
```

# Status
* `File`: complete
* `Page`: complete
* `Revision`: complete
* `Transform`: complete
* `Math`: complete
* `OAuth2`: implemented (as part of token management for POST etc)
* `CheckUser`: not implemented
* `EventBus`: not implemented
* `FlaggedRevs`: not implemented
* `GrowthExperiments`: not implemented
* `IPinfo`: not implemented
* `SecurePoll`: not implemented
* `SiteMap`: not implemented
* `CampaignEvents`: not implemented
* `WikimediaCampaignEvents`: not implemented
