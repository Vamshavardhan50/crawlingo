import sys
import os

# Allow importing crawlingo from the local python source tree directly
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "../sdk/python")))

from crawlingo import Dataset

def main():
    print("=== Crawlingo Structured Dataset Example ===")
    
    url = "https://httpbin.org/html"
    print(f"Building dataset query for {url}...")
    
    # Fluent API definition
    dataset = (
        Dataset(url)
        .auto_match(True)  # Learns element DOM fingerprint for self-healing
        .field("title", "h1")
        .field("author", "p", selector_type="xpath")
        .field("content", "div")
        .build()
    )
    
    # Print results
    print("\nExtracted Fields:")
    result_dict = dataset.to_dict()
    for field, value in result_dict.items():
        print(f"  {field}: {value[:60]}...")
        
    # Export results
    print("\nExporting results...")
    dataset.to_json("dataset_result.json")
    dataset.to_csv("dataset_result.csv")
    print("Created 'dataset_result.json' and 'dataset_result.csv'.")

if __name__ == "__main__":
    main()
