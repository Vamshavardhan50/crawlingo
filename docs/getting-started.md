# Getting Started

Install Crawlingo and run your first web page extraction in under 5 minutes.

## Quick Install

```bash
# Python
pip install crawlingo

# Node.js
npm install crawlingo

# Rust (add to Cargo.toml)
cargo add crawlingo
```

## Your First Extraction

### Python

```python
from crawlingo import Page

# Fetch a page
page = Page("https://example.com")

# Basic info
print(f"Status: {page.status}")
print(f"Title: {page.title()}")

# Extract with CSS selectors
h1 = page.css("h1")
print(f"H1 text: {h1.text()}")

# Extract with XPath
paragraphs = page.xpath("//p")
for p in paragraphs:
    print(f"Paragraph: {p.text()}")

# Get clean markdown
print(page.markdown()[:200])
```

### Node.js

```javascript
const { Page } = require('crawlingo');

async function main() {
  const page = await Page.create('https://example.com');

  console.log(`Status: ${page.status}`);
  console.log(`Title: ${page.title()}`);

  const h1 = page.css('h1');
  console.log(`H1 text: ${h1.text.join(', ')}`);

  const paragraphs = page.xpath('//p');
  paragraphs.forEach((p, i) => {
    console.log(`Paragraph ${i + 1}: ${p.text}`);
  });
}

main();
```

### Rust

```rust
use crawlingo::Page;

let page = Page::new("https://example.com").unwrap();
println!("Status: {}", page.status());
println!("Title: {}", page.title()?);

let h1 = page.css("h1")?;
println!("H1 text: {}", h1.text());

for p in page.xpath("//p")? {
    println!("Paragraph: {}", p.text());
}
```

## Next Steps

- [Page API](page.md) — Detailed Page methods and selectors
- [Session API](session.md) — Headers, proxies, rate limits
- [Dataset API](dataset.md) — Structured data extraction
- [Selectors Guide](selectors.md) — All selector types in depth
