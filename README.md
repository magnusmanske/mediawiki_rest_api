[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![License](https://img.shields.io/badge/license-APACHE2-blue?style=flat-square)](LICENSE-APACHE2)

This Rust crate provides a client for the MediaWiki REST API.

# Usage
```rust
use mediawiki_rest_api::prelude::*;

// Create a client for the English Wikipedia REST API
let api = RestApiBuilder::wikipedia("en").build();

// From the page "Rust (programming language)"...
let page = Page::new("Rust (programming language)");

// ...retrieve basic information and wikitext
let (page_info,wikitext) = page.get(&api, false).await.unwrap();
```
