"""Compare local PEP output with independently saved source HTML; no network calls.

Usage: python scripts/verify-pep-sample.py INDEX_HTML PEP_1_HTML RESULT_JSON
The index and PEP 1 snapshots must be saved at approximately the collection time.
"""
from html.parser import HTMLParser
from pathlib import Path
import hashlib
import html
import json
import re
import sys


class Directory(HTMLParser):
    def __init__(self):
        super().__init__()
        self.active = False
        self.in_row = False
        self.cell = None
        self.cells = []
        self.rows = []
        self.classification = ""

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "table" and "pep-zero-table" in attrs.get("class", "").split():
            self.active = True
        if self.active and tag == "tr":
            self.in_row, self.cells, self.classification = True, [], ""
        if self.in_row and tag == "td":
            self.cell = ""
        if self.in_row and tag == "abbr":
            self.classification = attrs.get("title", "")

    def handle_data(self, data):
        if self.cell is not None:
            self.cell += data

    def handle_endtag(self, tag):
        if tag == "td" and self.cell is not None:
            self.cells.append(" ".join(self.cell.split()))
            self.cell = None
        if tag == "tr" and self.in_row:
            if len(self.cells) >= 4:
                self.rows.append((int(self.cells[1]), self.cells[2], self.classification))
            self.in_row = False
        if tag == "table":
            self.active = False


class Document(HTMLParser):
    def __init__(self):
        super().__init__()
        self.sections = 0
        self.parts = []

    def handle_starttag(self, tag, attrs):
        if tag == "section" and (self.sections or dict(attrs).get("id") == "pep-content"):
            self.sections += 1

    def handle_endtag(self, tag):
        if tag == "section" and self.sections:
            self.sections -= 1

    def handle_data(self, data):
        if self.sections:
            self.parts.append(data)


def compact(text):
    return re.sub(r"\s+", "", text)


def main():
    index_file, detail_file, result_file = map(Path, sys.argv[1:])
    index = Directory()
    index.feed(index_file.read_text(encoding="utf-8"))
    expected = {number: (title, classification) for number, title, classification in index.rows}
    actual = [json.loads(line) for line in Path("work/research/directory.jsonl").read_text(encoding="utf-8").splitlines()]
    mismatches = [row["pep_number"] for row in actual
                  if expected.get(row["pep_number"]) != (row["title"], html.unescape(row["classification"]))]
    detail = Document()
    detail.feed(detail_file.read_text(encoding="utf-8"))
    records = []
    for shard in sorted(Path("work/research/details").glob("part-" + "[0-9]" * 8 + ".jsonl")):
        records.extend(json.loads(line) for line in shard.read_text(encoding="utf-8").splitlines())
    sample = next(row for row in records if row["canonical_url"] == "https://peps.python.org/pep-0001/")
    ordered_text_equal = compact("".join(detail.parts)) == compact(html.unescape(sample["document_text"]))
    result = {
        "independent_index_rows": len(expected), "exported_directory_rows": len(actual),
        "title_and_classification_mismatches": len(mismatches),
        "pep_1_text_matches_source_character_order_ignoring_whitespace": ordered_text_equal,
        "scope": "All directory IDs/titles/classifications; PEP 1 text only. HTMLParser baseline, entity decoding, whitespace ignored. Other document bodies not semantically certified.",
        "source_sha256": {file.name: hashlib.sha256(file.read_bytes()).hexdigest() for file in [index_file, detail_file]},
    }
    result_file.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result))
    if len(expected) != len(actual) or mismatches or not ordered_text_equal:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
