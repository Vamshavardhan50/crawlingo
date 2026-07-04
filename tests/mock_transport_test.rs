//! Offline tests for the fetch → parse → extract pipeline.
//!
//! These exercise the real `Dataset`/`Session` code paths end-to-end without any network access,
//! by injecting a `MockTransport` into the session. Before the `Transport` abstraction existed the
//! only way to test extraction was to load static HTML fixtures and bypass the fetch layer
//! entirely — the network, retry, and rate-limit code was untested.

use crawlingo::dataset::builder::{Dataset, DatasetField};
use crawlingo::engine::fetcher::MockTransport;
use crawlingo::engine::session::Session;
use std::sync::Arc;

fn css_field(name: &str, selector: &str, default: Option<&str>) -> DatasetField {
    DatasetField {
        name: name.to_string(),
        selector: selector.to_string(),
        selector_type: "css".to_string(),
        #[cfg(feature = "python")]
        transform: None,
        default: default.map(|s| s.to_string()),
    }
}

#[test]
fn dataset_build_extracts_fields_offline() {
    let session = Arc::new(Session::new());
    let mock = Arc::new(MockTransport::new().with_html(
        "https://shop.example.com/item/1",
        "<html><body>\
            <h1 class='title'>Premium Widget</h1>\
            <span class='price'>$42.00</span>\
         </body></html>",
    ));
    session.set_transport(mock.clone());

    let mut dataset = Dataset::new("https://shop.example.com/item/1", session);
    dataset.add_field(css_field("title", "h1.title", None));
    dataset.add_field(css_field("price", "span.price", None));
    dataset.add_field(css_field("stock", "span.stock", Some("unknown")));

    let result = dataset.build().expect("offline build should succeed");

    assert_eq!(result.fields.get("title").map(String::as_str), Some("Premium Widget"));
    assert_eq!(result.fields.get("price").map(String::as_str), Some("$42.00"));
    // Missing selector falls back to the declared default.
    assert_eq!(result.fields.get("stock").map(String::as_str), Some("unknown"));

    // The transport was actually driven, exactly once, with the requested URL.
    assert_eq!(mock.calls(), vec!["https://shop.example.com/item/1".to_string()]);
}

#[test]
fn session_shares_a_single_fetch_manager() {
    let session = Session::new();
    let first = session.fetch_manager();
    let second = session.fetch_manager();
    assert!(
        Arc::ptr_eq(&first, &second),
        "fetch_manager() must return the same shared instance, not a fresh one per call"
    );
}

#[test]
fn set_transport_overrides_the_session_manager() {
    let session = Arc::new(Session::new());
    // Prime the lazily-built real manager first...
    let _ = session.fetch_manager();
    // ...then inject a mock; subsequent fetches must use the mock.
    let mock = Arc::new(MockTransport::new().with_default_html("<h1 class='t'>Injected</h1>"));
    session.set_transport(mock.clone());

    let mut dataset = Dataset::new("https://any.example.com", session);
    dataset.add_field(css_field("t", "h1.t", None));

    let result = dataset.build().expect("build should use the injected transport");
    assert_eq!(result.fields.get("t").map(String::as_str), Some("Injected"));
    assert_eq!(mock.call_count(), 1);
}
