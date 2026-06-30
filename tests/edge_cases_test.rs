use crawlingo::parser::streaming::parse_html;
use crawlingo::selector::css;

#[test]
fn test_html_structure_edge_cases() {
    // 1. Empty HTML
    let empty_html = b"";
    let tree = parse_html(empty_html).unwrap();
    assert!(tree.nodes.is_empty());

    // 2. Missing html, head, body tags
    let minimal_html = b"<div><p>Hello World</p></div>";
    let tree = parse_html(minimal_html).unwrap();
    assert!(!tree.nodes.is_empty());
    let p_idx = css::query(&tree, "p");
    assert!(!p_idx.is_empty());
    assert_eq!(tree.get_text(p_idx[0]), "Hello World");

    // 3. Unclosed tags & Incorrect nesting
    let malformed_html = b"<div><p>Paragraph <span>Nested</div>";
    let tree = parse_html(malformed_html).unwrap();
    assert!(!tree.nodes.is_empty());

    // 4. Duplicate IDs and Invalid attributes
    let dup_attrs = b"<div id='test' class='a' id='test2' invalid-attr='val'>Content</div>";
    let tree = parse_html(dup_attrs).unwrap();
    assert!(!tree.nodes.is_empty());

    // 5. Self-closing SVG and MathML tags
    let svg_html = b"<div><svg><path d='M10 10 H 90 V 90 H 10 Z' /><circle cx='50' cy='50' r='40' /></svg></div>";
    let tree = parse_html(svg_html).expect("Should handle self-closing SVG tags without crashing");
    let path_idx = css::query(&tree, "path");
    assert!(!path_idx.is_empty(), "Should parse SVG path successfully");

    let mathml_html = b"<math><mi>x</mi><mo>+</mo><mi>y</mi></math>";
    let tree = parse_html(mathml_html).unwrap();
    assert!(!tree.nodes.is_empty());

    // 6. Broken comments & DOCTYPE
    let comment_html = b"<!DOCTYPE html SYSTEM 'broken'><!-- unclosed comment <div>hello</div>";
    let _tree = parse_html(comment_html).expect("Should parse unclosed comments without panicking");

    // 7. Extremely deep DOM trees (e.g. 500 nested divs)
    let mut deep_html = Vec::new();
    for _ in 0..500 {
        deep_html.extend_from_slice(b"<div>");
    }
    deep_html.extend_from_slice(b"Leaf");
    for _ in 0..500 {
        deep_html.extend_from_slice(b"</div>");
    }
    let tree = parse_html(&deep_html).unwrap();
    assert!(!tree.nodes.is_empty());
    let leaf_idx = css::query(&tree, "div");
    assert!(!leaf_idx.is_empty());
}

#[test]
fn test_encoding_edge_cases() {
    // 1. UTF-8 with Emojis
    let utf8_html = "<div>Hello 🚀 World!</div>".as_bytes();
    let tree = parse_html(utf8_html).unwrap();
    let div_idx = css::query(&tree, "div");
    assert_eq!(tree.get_text(div_idx[0]), "Hello 🚀 World!");

    // 2. RTL scripts (Arabic/Hebrew)
    let rtl_html = "<div>שלום עולם</div>".as_bytes();
    let tree = parse_html(rtl_html).unwrap();
    let div_idx = css::query(&tree, "div");
    assert_eq!(tree.get_text(div_idx[0]), "שלום עולם");

    // 3. Combining characters
    let combining_html = "<div>ñ</div>".as_bytes();
    let tree = parse_html(combining_html).unwrap();
    let div_idx = css::query(&tree, "div");
    assert_eq!(tree.get_text(div_idx[0]), "ñ");

    // 4. Invalid UTF-8 bytes (raw ISO-8859-1 or Windows-1252)
    // 0xE9 is 'é' in Latin-1 / Windows-1252
    let latin1_html = b"<div>Caf\xE9</div>";
    let tree = parse_html(latin1_html).unwrap();
    let div_idx = css::query(&tree, "div");
    // Since we decode lossily, it should recover and not panic
    let txt = tree.get_text(div_idx[0]);
    assert!(txt.contains("Caf"));
}

#[test]
fn test_additional_elements_parsing() {
    // Links, forms, images, tables, media, metadata, performance, structured data
    let complex_html = b"
        <html>
        <head>
            <title>Test Page</title>
            <meta name='viewport' content='width=device-width, initial-scale=1.0' />
            <link rel='canonical' href='https://example.com/canonical' />
            <script type='application/ld+json'>{\"@context\": \"https://schema.org\"}</script>
        </head>
        <body>
            <a href='/relative' data-custom='test'>Link</a>
            <form action='/submit' method='POST' enctype='multipart/form-data'>
                <input type='hidden' name='csrf' value='12345' />
                <input type='file' name='upload' />
            </form>
            <img src='img.jpg' srcset='img-2x.jpg 2x' alt='Image' loading='lazy' />
            <table>
                <tr>
                    <td rowspan='2' colspan='2'>Table Header</td>
                </tr>
            </table>
            <video src='video.mp4'><track kind='subtitles' /></video>
        </body>
        </html>
    ";

    let tree = parse_html(complex_html).unwrap();
    assert!(!tree.nodes.is_empty());

    let canonical_idx = css::query(&tree, "link");
    assert!(!canonical_idx.is_empty());
    assert_eq!(
        tree.nodes[canonical_idx[0]].attrs.get("href").unwrap(),
        "https://example.com/canonical"
    );

    let form_idx = css::query(&tree, "form");
    assert!(!form_idx.is_empty());

    let img_idx = css::query(&tree, "img");
    assert!(!img_idx.is_empty());

    let td_idx = css::query(&tree, "td");
    assert!(!td_idx.is_empty());
}

#[test]
fn test_local_edge_cases_file() {
    let html = include_bytes!("fixtures/edge_cases.html");
    let tree = parse_html(html).expect("Failed to parse edge_cases.html");
    assert!(!tree.nodes.is_empty());

    // Verify SVG path and rect are parsed
    let rect_idx = css::query(&tree, "rect");
    assert!(!rect_idx.is_empty(), "Should parse SVG rect");

    let path_idx = css::query(&tree, "path");
    assert!(!path_idx.is_empty(), "Should parse SVG path");

    // Verify encodings RTL and UTF-8
    let utf8_idx = css::query(&tree, ".utf8");
    assert!(!utf8_idx.is_empty());
    assert!(tree.get_text(utf8_idx[0]).contains("🚀"));

    // Verify links
    let mailto_idx = css::query(&tree, "#mail-link");
    assert!(!mailto_idx.is_empty());
    assert_eq!(
        tree.nodes[mailto_idx[0]].attrs.get("href").unwrap(),
        "mailto:test@example.com"
    );

    // Verify table rowspan/colspan
    let th_idx = css::query(&tree, "th");
    assert!(!th_idx.is_empty());
    assert_eq!(tree.nodes[th_idx[0]].attrs.get("colspan").unwrap(), "2");

    // Verify metadata
    let link_indices = css::query(&tree, "link");
    assert!(link_indices.len() >= 2);
    let alternate_node = link_indices
        .iter()
        .map(|&idx| &tree.nodes[idx])
        .find(|node| node.attrs.contains_key("hreflang"))
        .expect("Should find alternate link with hreflang");
    assert_eq!(alternate_node.attrs.get("hreflang").unwrap(), "es");
}
