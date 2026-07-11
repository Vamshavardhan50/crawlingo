//! Offline tests for the fetch → parse → extract pipeline.
//!
//! These exercise the real `Dataset`/`Session` code paths end-to-end without any network access,
//! by injecting a `MockTransport` into the session. Before the `Transport` abstraction existed the
//! only way to test extraction was to load static HTML fixtures and bypass the fetch layer
//! entirely — the network, retry, and rate-limit code was untested.

use crawlingo::dataset::builder::{Dataset, DatasetField};
use crawlingo::dataset::schema::{DatasetSchema, FieldConstraint, FieldType};
use crawlingo::engine::fetcher::{MockTransport, NormalizedResponse};
use crawlingo::engine::session::Session;
use crawlingo::extraction::ExtractionType;
use std::sync::Arc;

fn css_field(name: &str, selector: &str, default: Option<&str>) -> DatasetField {
    DatasetField {
        name: name.to_string(),
        selector: selector.to_string(),
        selector_type: "css".to_string(),
        #[cfg(feature = "python")]
        transform: None,
        default: default.map(|s| s.to_string()),
        extract_type: Default::default(),
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

    assert_eq!(
        result.fields.get("title").map(String::as_str),
        Some("Premium Widget")
    );
    assert_eq!(
        result.fields.get("price").map(String::as_str),
        Some("$42.00")
    );
    // Missing selector falls back to the declared default.
    assert_eq!(
        result.fields.get("stock").map(String::as_str),
        Some("unknown")
    );

    // The transport was actually driven, exactly once, with the requested URL.
    assert_eq!(
        mock.calls(),
        vec!["https://shop.example.com/item/1".to_string()]
    );
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

    let result = dataset
        .build()
        .expect("build should use the injected transport");
    assert_eq!(result.fields.get("t").map(String::as_str), Some("Injected"));
    assert_eq!(mock.call_count(), 1);
}

#[test]
fn typed_extraction_normalizes_price() {
    let session = Arc::new(Session::new());
    let mock = Arc::new(MockTransport::new().with_html(
        "https://shop.example.com/item/4",
        "<span class='price'>$1,234.56</span>",
    ));
    session.set_transport(mock);

    let mut dataset = Dataset::new("https://shop.example.com/item/4", session);
    dataset.add_field(DatasetField {
        name: "price".to_string(),
        selector: "span.price".to_string(),
        selector_type: "css".to_string(),
        #[cfg(feature = "python")]
        transform: None,
        default: None,
        extract_type: ExtractionType::Price,
    });

    let result = dataset.build().expect("build should succeed");
    assert_eq!(
        result.fields.get("price").map(String::as_str),
        Some("1234.56")
    );
}

#[test]
fn schema_validation_coerces_typed_fields() {
    let session = Arc::new(Session::new());
    let mock = Arc::new(MockTransport::new().with_html(
        "https://shop.example.com/item/5",
        "<h1 class='title'>Widget</h1><span class='price'>$19.99</span>",
    ));
    session.set_transport(mock);

    let schema = DatasetSchema::new(vec![
        FieldConstraint::new("title", FieldType::String, true),
        FieldConstraint::new("price", FieldType::Float, true),
    ]);

    let mut dataset = Dataset::new("https://shop.example.com/item/5", session).with_schema(schema);
    dataset.add_field(css_field("title", "h1.title", None));
    dataset.add_field(css_field("price", "span.price", None));

    let result = dataset.build().expect("schema-valid build should succeed");
    assert_eq!(
        result.fields.get("price").map(String::as_str),
        Some("19.99")
    );
}

#[test]
fn schema_validation_rejects_missing_required_field() {
    let session = Arc::new(Session::new());
    let mock = Arc::new(MockTransport::new().with_html(
        "https://shop.example.com/item/6",
        "<h1 class='title'>Widget</h1>",
    ));
    session.set_transport(mock);

    let schema = DatasetSchema::new(vec![FieldConstraint::new("price", FieldType::Float, true)]);

    let mut dataset = Dataset::new("https://shop.example.com/item/6", session).with_schema(schema);
    dataset.add_field(css_field("price", "span.price", None));

    let result = dataset.build();
    assert!(
        result.is_err(),
        "missing required schema field should fail the build"
    );
}

#[tokio::test]
async fn build_many_streamed_extracts_all_urls_concurrently() {
    let session = Arc::new(Session::new());
    let mock = Arc::new(
        MockTransport::new()
            .with_html("https://shop.example.com/a", "<h1 class='t'>Alpha</h1>")
            .with_html("https://shop.example.com/b", "<h1 class='t'>Bravo</h1>")
            .with_html("https://shop.example.com/c", "<h1 class='t'>Charlie</h1>"),
    );
    session.set_transport(mock.clone());

    let mut dataset = Dataset::new("unused", session);
    dataset.add_field(css_field("t", "h1.t", None));

    let urls = vec![
        "https://shop.example.com/a".to_string(),
        "https://shop.example.com/b".to_string(),
        "https://shop.example.com/c".to_string(),
    ];
    let mut stream = dataset.build_many_streamed(urls, 2);

    let mut titles = Vec::new();
    while let Some(record) = stream.recv().await {
        let fields = record.expect("each url should extract successfully");
        assert!(
            fields.contains_key("url"),
            "url should be injected into the record"
        );
        titles.push(fields.get("t").cloned().unwrap_or_default());
    }
    titles.sort();
    assert_eq!(
        titles,
        vec![
            "Alpha".to_string(),
            "Bravo".to_string(),
            "Charlie".to_string()
        ]
    );
    assert_eq!(mock.call_count(), 3);
}

fn fake_response(url: &str, html: &str) -> NormalizedResponse {
    NormalizedResponse {
        url: url.to_string(),
        status: 200,
        headers: Default::default(),
        cookies: Default::default(),
        body: html.as_bytes().to_vec().into(),
        content_type: "text/html; charset=utf-8".to_string(),
        encoding: "utf-8".to_string(),
        timings: Default::default(),
    }
}

#[tokio::test]
async fn compile_stream_extracts_from_a_page_channel() {
    let session = Arc::new(Session::new());
    let dataset = {
        let mut d = Dataset::new("unused", session);
        d.add_field(css_field("t", "h1.t", None));
        d
    };

    let (tx, rx) = tokio::sync::mpsc::channel(4);
    let mut result_rx = dataset.compile_stream(rx);

    let page_a = crawlingo::parser::streaming::HtmlParser::parse(fake_response(
        "https://a.example.com",
        "<h1 class='t'>PageA</h1>",
    ))
    .unwrap();
    let page_b = crawlingo::parser::streaming::HtmlParser::parse(fake_response(
        "https://b.example.com",
        "<h1 class='t'>PageB</h1>",
    ))
    .unwrap();

    tx.send(page_a).await.unwrap();
    tx.send(page_b).await.unwrap();
    drop(tx);

    let mut titles = Vec::new();
    while let Some(res) = result_rx.recv().await {
        titles.push(res.unwrap().fields.get("t").cloned().unwrap_or_default());
    }
    titles.sort();
    assert_eq!(titles, vec!["PageA".to_string(), "PageB".to_string()]);
}
