"""Run from any directory after building/installing the Python SDK."""
import json
import os
from pathlib import Path
from crawlingo import DatasetJob

os.chdir(Path(__file__).resolve().parent)
with open("job.json", encoding="utf-8") as source:
    job = DatasetJob(json.load(source))
report = job.run()
print(json.dumps(report, indent=2))
raise SystemExit(2 if report["pages_failed"] or report["records_rejected"] or report["cancelled"] else 0)
