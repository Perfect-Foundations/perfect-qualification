#!/usr/bin/env python3
"""Fail closed on Wave24 canonical exact numerical output across three local targets.
Only test stdout records beginning W24| are hashed. No Rust Debug, timing,
build paths or test harness output enters the canonical representation.
"""
import argparse
import hashlib
import json
from pathlib import Path

GROUPS = {"POLY":"POLY", "FEC":"RS", "FIN":"FIN"}
PLATFORMS = ("win","wsl","qemu")
SOURCE = {
    "POLY":"7774e361c8a86c1ad3f5358d8df7a353371713d9",
    "FEC":"ca6fd8437a3bb024ebe199cf6a6f6c3d0427bde5",
    "FIN":"7d0e44ad33d33313c671348605d5562e7bbed478",
}
def canonical(log:Path,component:str)->list[str]:
    if not log.is_file():
        raise ValueError(f"missing completed log {log}")
    lines=log.read_text(encoding="utf-8").splitlines()
    rows=[line for line in lines if line.startswith("W24|")]
    if len(rows)!=128:
        raise ValueError(f"{log}: expected 128 records, got {len(rows)}")
    for i,line in enumerate(rows):
        fields=line.split("|")
        if len(fields)<5 or fields[1]!=component or fields[2]!=f"{i:03}":
            raise ValueError(f"{log}: case {i} invalid order/domain: {line[:120]}")
        line.encode("ascii")
    return rows
def digest(lines:list[str])->str:
    return hashlib.sha256(("\n".join(lines)+"\n").encode("ascii")).hexdigest()
def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--log-dir",required=True,type=Path)
    parser.add_argument("--output",type=Path)
    parser.add_argument("--poly-source",choices=("7774e361c8a86c1ad3f5358d8df7a353371713d9","ae45f4d386dff6d525424b70e3115074dc7f8f51"),
                        default="ae45f4d386dff6d525424b70e3115074dc7f8f51")
    parser.add_argument("--poly-log-tag",default="",
                        help="Optional immutable Polynomial log tag, e.g. 7774")
    args=parser.parse_args()
    results={}
    transcripts={}
    current_sources={**SOURCE,"POLY":args.poly_source}
    for name,domain in GROUPS.items():
        runs={}
        for host in PLATFORMS:
            tag=f"-{args.poly_log_tag}" if name=="POLY" and args.poly_log_tag else ""
            path=args.log_dir/f"wave24-canonical-{name}-{host}{tag}-20261013.txt"
            runs[host]=canonical(path,domain)
        for host in PLATFORMS[1:]:
            if runs["win"]!=runs[host]:
                index=next((i for i,(a,b) in enumerate(zip(runs["win"],runs[host])) if a!=b),None)
                raise ValueError(f"{name}: FIRST DIVERGENCE at case {index} win={runs['win'][index]} {host}={runs[host][index]}")
        transcripts[name]=runs
        results[name]={"source_sha":current_sources[name],"records":128,
            "sha256":{host:digest(rows) for host,rows in runs.items()}}
    combined={}
    for host in PLATFORMS:
        lines=[f"DOMAIN|{name}|{current_sources[name]}" for name in GROUPS]
        lines += [line for name in GROUPS for line in transcripts[name][host]]
        combined[host]=digest(lines)
    if len(set(combined.values()))!=1:
        raise ValueError("combined canonical digests mismatch")
    result={"schema":"perfect-foundations.wave24.canonical.v1",
        "rust":"1.99.0","features":"all-features","platforms":list(PLATFORMS),
        "canonical_rule":"ASCII W24 records, ordinal ascending, LF line separator, terminal LF",
        "groups":results,"combined_sha256":combined}
    output=json.dumps(result,indent=2,sort_keys=True)+"\n"
    if args.output:
        args.output.parent.mkdir(parents=True,exist_ok=True)
        args.output.write_text(output,encoding="utf-8",newline="\n")
    print(output)
if __name__=="__main__":
    main()
