use crawlingo::parser::streaming::parse_html;
use crawlingo::selector::css;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_css_selector(c: &mut Criterion) {
    // Large document simulation
    let mut html = String::new();
    html.push_str("<html><body>");
    for i in 0..100 {
        html.push_str(&format!(
            "<div class='product' id='p-{}'><h1 class='title'>Product {}</h1><span class='price'>${}</span></div>",
            i, i, i * 10
        ));
    }
    html.push_str("</body></html>");

    let tree = parse_html(html.as_bytes()).unwrap();
    let selector = "div.product span.price";

    c.bench_function("css_query_large_doc", |b| {
        b.iter(|| css::query(black_box(&tree), black_box(selector)))
    });
}

criterion_group!(benches, bench_css_selector);
criterion_main!(benches);
