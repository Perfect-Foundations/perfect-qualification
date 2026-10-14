"""Compare byte-exact Polynomial results across two hosts and two feature modes."""
from pathlib import Path
from hashlib import sha256
import sys
if len(sys.argv) != 5:
    raise SystemExit("pass Windows all/no-default, Linux all/no-default files")
paths = [Path(p) for p in sys.argv[1:]]
data = [p.read_bytes() for p in paths]
assert data[0].startswith(b"PF-CANONICAL-POLYNOMIAL-V1\n")
assert data[0].endswith(b"\n") and b"\r" not in data[0]
labels = [x.split(b"|", 1)[0] for x in data[0].splitlines()[1:]]
assert len(labels) >= 30 and len(set(labels)) == len(labels)
for path, value in zip(paths, data):
    print(path.name, len(value), sha256(value).hexdigest())
    assert value == data[0], f"cross-host byte mismatch: {path}"
print("CROSS_HOST_FEATURE_DETERMINISM_PASS", len(labels))
