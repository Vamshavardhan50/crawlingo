"""Crawlingo: Dataset with schema validation and export to multiple formats.

Usage:
    pip install crawlingo
    python 09_dataset_schema.py
"""

from crawlingo import Dataset
from crawlingo.dataset import DatasetSchema, FieldConstraint, FieldType


def main():
    print("=== Crawlingo Schema Validation Example ===\n")

    # Define a schema with required typed fields
    schema = DatasetSchema([
        FieldConstraint("title", FieldType.String, required=True),
        FieldConstraint("price", FieldType.Float, required=True),
        FieldConstraint("description", FieldType.String, required=False),
    ])

    # Build dataset with schema validation
    dataset = (
        Dataset("https://httpbin.org/html")
        .with_schema(schema)
        .field("title", "h1")
        .field("price", "p", extraction_type="price")
        .field("description", "div", default="No description")
        .build()
    )

    print("Extracted results:")
    print(f"  {dataset.to_dict()}\n")

    # Export
    dataset.to_json("schema_output.json")
    dataset.to_csv("schema_output.csv")
    print("Exported to schema_output.json and schema_output.csv")


if __name__ == "__main__":
    main()
