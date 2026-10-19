"""Run an explicitly configured, rate-limited public-site sample and verify restart.

Uses the built Rust CLI for every dataset request. Python only monitors the child,
reads local shards, and computes integrity hashes. Run from the repository root.
"""
import argparse
import ctypes
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import time
from datetime import datetime, timezone


def windows_peak(process):
    if os.name != "nt":
        return None
    from ctypes import wintypes

    class Counters(ctypes.Structure):
        _fields_ = [("cb", wintypes.DWORD), ("faults", wintypes.DWORD)] + [
            (name, ctypes.c_size_t) for name in
            ("peak", "working", "paged_peak", "paged", "nonpaged_peak", "nonpaged", "pagefile", "pagefile_peak")
        ]
    counters = Counters()
    counters.cb = ctypes.sizeof(counters)
    function = ctypes.windll.psapi.GetProcessMemoryInfo
    function.argtypes = [wintypes.HANDLE, ctypes.c_void_p, wintypes.DWORD]
    function.restype = wintypes.BOOL
    return counters.peak if function(int(process._handle), ctypes.byref(counters), counters.cb) else None


def digest(path):
    hasher = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(65536), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def snapshot(directory):
    return {p.name: {"bytes": p.stat().st_size, "sha256": digest(p)}
            for p in sorted(directory.glob("part-*")) if p.is_file()}


def run(command, log):
    started = time.perf_counter()
    peak = None
    with log.open("w", encoding="utf-8") as stream:
        process = subprocess.Popen(command, stdout=stream, stderr=subprocess.STDOUT)
        while process.poll() is None:
            sample = windows_peak(process)
            if sample is not None:
                peak = max(peak or 0, sample)
            time.sleep(0.02)
        sample = windows_peak(process)
        if sample is not None:
            peak = max(peak or 0, sample)
    return {"exit_code": process.returncode, "process_wall_seconds": round(time.perf_counter() - started, 3),
            "peak_cli_working_set_bytes": peak}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("job", type=Path)
    parser.add_argument("evidence", type=Path)
    parser.add_argument("--binary", type=Path, default=Path("target/release/crawlingo.exe" if os.name == "nt" else "target/release/crawlingo"))
    parser.add_argument("--session", type=Path, default=Path("examples/research_dataset/session.json"))
    args = parser.parse_args()
    config = json.loads(args.job.read_text(encoding="utf-8"))
    output = Path(config["output_dir"])
    if output.exists() or args.evidence.exists():
        raise SystemExit("Use a fresh output directory and evidence filename for an initial measurement.")
    if not config.get("respect_robots", True):
        raise SystemExit("This public-site measurement requires respect_robots=true.")
    args.evidence.parent.mkdir(parents=True, exist_ok=True)
    command = [str(args.binary.resolve()), "--config", str(args.session), "--rate-limit-rps", "1", "--timeout", "30", "dataset", str(args.job)]
    evidence = {"captured_at_utc": datetime.now(timezone.utc).isoformat(), "platform": platform.platform(),
                "logical_cpus": os.cpu_count(), "command": command, "rate_limit_rps_per_host": 1,
                "job_config": config, "binary_sha256": digest(args.binary),
                "input_sha256": digest(Path(config["urls_file"])), "session_sha256": digest(args.session),
                "memory_scope": "Rust CLI process only; Windows peak working set, sampled every 20ms; null on other systems."}
    evidence["initial"] = run(command, args.evidence.with_suffix(".initial.log"))
    if (output / "report.json").exists():
        evidence["initial"]["report"] = json.loads((output / "report.json").read_text(encoding="utf-8"))
    before = snapshot(output)
    evidence["initial"]["artifacts"] = before
    if evidence["initial"]["exit_code"] == 0:
        report = evidence["initial"]["report"]
        count = 0
        keys = set()
        for shard in sorted(output.glob("part-" + "[0-9]" * 8 + ".jsonl")):
            with shard.open(encoding="utf-8") as stream:
                for line in stream:
                    row = json.loads(line)
                    assert all(isinstance(value, str) for value in row.values()), "Unexpected field type"
                    assert row["_source_url"].startswith("https://peps.python.org/"), "Unexpected source"
                    assert row["_row"].isdigit(), "Missing row provenance"
                    for field in config["fields"]:
                        if field.get("required"):
                            assert row[field["name"]].strip(), "Missing required value"
                    key = tuple(row[field] for field in config["dedup_fields"])
                    assert key not in keys, "Duplicate record key"
                    keys.add(key)
                    count += 1
        assert count == report["records_written"] and count > 0, "Report/export count mismatch"
        evidence["verified_records"] = count
        evidence["resume"] = run(command, args.evidence.with_suffix(".resume.log"))
        evidence["resume"]["report"] = json.loads((output / "report.json").read_text(encoding="utf-8"))
        evidence["resume"]["shards_unchanged"] = before == snapshot(output)
        assert evidence["resume"]["exit_code"] == 0 and evidence["resume"]["shards_unchanged"], "Restart changed artifacts"
        for name, value in report.items():
            if name != "elapsed_ms":
                assert evidence["resume"]["report"][name] == value, "Restart changed cumulative counters"
    args.evidence.write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"evidence": str(args.evidence), "initial": evidence["initial"]["exit_code"],
                      "verified_records": evidence.get("verified_records"), "report": evidence["initial"].get("report")}))
    raise SystemExit(evidence["initial"]["exit_code"])


if __name__ == "__main__":
    main()
