# Page

Fetch and inspect an individual page to develop selectors before creating a bulk dataset job.

<a id="create"></a>

## Create a page

Python Page(url) loads lazily when content is accessed. Page.create(url) fetches immediately. Node.js uses await Page.create(url, options); its constructor accepts a native page object, not a URL.

### Python

```python
from crawlingo import Page

page = Page.create("https://example.com/", timeout=30)
print(page.status, page.title())
print(page.css("h1").text())
```

### Node.js

```javascript
import { Page } from 'crawlingo';

const page = await Page.create('https://example.com/', { timeout: 30 });
console.log(page.status, page.title());
console.log(page.css('h1').text);
```

<a id="options"></a>

## Fetch options

| Python argument | Node.js options key | Type / use |
| --- | --- | --- |
| url | url (first argument) | string: HTTP(S) page address. |
| timeout | timeout | Seconds: int / number. |
| headers | headers | Dictionary / record of request headers. |
| cookies | cookies | Dictionary / record of cookies. |
| proxy | proxy | Proxy URL string. |
| session | session | Existing Session instance. |
| auto_match | autoMatch | boolean: enable supported selector recovery. |
| retries | Not exposed | Python constructor argument; do not assume a Node option exists. |
| Not exposed | browserProfile | Node option string for a transport profile. |

<a id="members"></a>

## Properties and selector methods

| Python | Node.js | Returns |
| --- | --- | --- |
| page.url | page.url | string: page URL. |
| page.status | page.status | int / number: HTTP status. |
| html() | page.html | string: retained HTML. |
| title() | title() | string: document title. |
| css(selector) | css(selector) | ElementCollection for a CSS selector. |
| xpath(query) | xpath(query) | ElementCollection for a supported XPath query. |
| find_text(text) | findText(text) | ElementCollection for matching text. |
| after_text(text) | afterText(text) | ElementCollection after matching text. |
| before_text(text) | beforeText(text) | ElementCollection before matching text. |
| regex(pattern) | regex(pattern) | ElementCollection from a regular-expression query. |

- [Selector examples and limitations](selectors.md)

<a id="bulk"></a>

## From one page to a dataset

Use Page to inspect a representative page and select a record container. Check pages with missing fields too. Page retains HTML and parsed content; do not retain a million Page objects in an application list. Move the extraction rule into DatasetJob fields and feed it a URL file.

- [DatasetJob API](api-dataset-job.md)
- [Complete developer tutorial](developer-tutorial.md)
