This Rust crate provides a client for the MediaWiki REST API.

# Usage
```rust
use mediawiki_rest_api::prelude::*;

// Create a client for the English Wikipedia REST API
let api = RestApiBuilder::wikipedia("en").build();

// From the page "Rust (programming language)"...
let page = Page::new("Rust (programming language)");

// ...retrieve basic information...
let result = page.get(&api, false).await.unwrap();

// ...and get the wikitext
let wikitext = result.source;
```
