import crawlingo
import os

def main():
    print("=== Operation Taboo Manuscript: Schema & Parquet Export ===")
    
    session = crawlingo.Session()
    
    # Target listing page
    url = "https://quotes.toscrape.com/"
    
    # 1. Define dataset schema constraints
    schema = crawlingo.DatasetSchema()
    schema.add_field("quote", crawlingo.FieldType.String, True)
    schema.add_field("author", crawlingo.FieldType.String, True)
    
    # 2. Setup Dataset Query
    dataset = crawlingo.Dataset(url, session)
    dataset.field("quote", ".quote .text")
    dataset.field("author", ".quote .author")
    
    # Attach our schema constraints darlin'
    dataset.with_schema(schema)
    
    try:
        print("Extracting dataset...")
        result = dataset.build()
        
        # 3. Export to Parquet
        parquet_path = "manuscripts.parquet"
        result.to_parquet(parquet_path)
        print(f"Successfully exported schema-validated records to: {os.path.abspath(parquet_path)}")
        
    except Exception as e:
        print(f"Oops, validation or build failed: {e}")

if __name__ == "__main__":
    main()
