"""Normalize directory rows and freeze a bounded URL sample; no network calls."""
import argparse
import html
import json
from pathlib import Path
from urllib.parse import urljoin, urlsplit


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--limit", type=int, default=20)
    args = parser.parse_args()
    if not 1 <= args.limit <= 50:
        parser.error("Use a sample of 1 to 50 documents; larger runs need a separate collection plan.")
    root = Path("work/research")
    report = json.loads((root / "index/report.json").read_text(encoding="utf-8"))
    if not report["complete"] or report["cancelled"] or report["pages_failed"] or report["records_rejected"]:
        raise SystemExit("Inspect the directory job report and error shards before continuing.")
    seen = set()
    selected = []
    output = root / "directory.jsonl"
    # Exclusive creation prevents accidentally replacing a frozen dataset or URL list.
    if output.exists() or (root / "detail-urls.txt").exists():
        raise SystemExit("Prepared inputs already exist. Reuse them, or choose a fresh run directory/config.")
    with output.open("x", encoding="utf-8") as destination:
        for shard in sorted((root / "index").glob("part-" + "[0-9]" * 8 + ".jsonl")):
            for line in shard.read_text(encoding="utf-8").splitlines():
                row = json.loads(line)
                number, title = row["pep_number"], html.unescape(row["title"])
                url = urljoin(row["_source_url"], row["relative_url"])
                parsed = urlsplit(url)
                if parsed.scheme != "https" or parsed.netloc != "peps.python.org" or not parsed.path.startswith("/pep-"):
                    raise ValueError(f"Unexpected directory destination: {url}")
                key = int(number)
                if key in seen:
                    raise ValueError(f"Duplicate PEP number: {key}")
                seen.add(key)
                destination.write(json.dumps({
                    "pep_number": key, "title": title, "url": url,
                    "classification": row["classification"], "_source_url": row["_source_url"],
                }, ensure_ascii=False) + "\n")
                if len(selected) < args.limit:
                    selected.append(url)
    if not selected:
        raise SystemExit("The directory produced no records.")
    (root / "detail-urls.txt").write_text("\n".join(selected) + "\n", encoding="utf-8")
    print(f"Normalized {len(seen)} directory records; froze {len(selected)} detail URLs.")


if __name__ == "__main__":
    main()
