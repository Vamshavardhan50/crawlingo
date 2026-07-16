import crawlingo

def main():
    print("=== Operation Cozy Sweater: Basic Web Fetch ===")
    
    # We will initialize a default Session
    session = crawlingo.Session()
    
    # Let's fetch the page
    url = "https://httpbin.org/html"
    print(f"Fetching darlin' page: {url}")
    
    try:
        page = crawlingo.Page(url, session=session)
        print(f"Status: {page.status}")
        print(f"Title: {page.title()}")
        
        # Extract outbound links
        links = page.css("a")
        print(f"Discovered {len(links)} links:")
        for idx in range(len(links)):
            el = links.at(idx)
            if el:
                print(f"  - Link text: '{el.text()}' | href: '{el.attr('href')}'")
    except Exception as e:
        print(f"Oh honey, something went wrong: {e}")

if __name__ == "__main__":
    main()
