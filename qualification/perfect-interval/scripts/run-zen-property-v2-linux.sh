#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH" CARGO_NET_OFFLINE=true
work="$(mktemp -d /home/ubuntu/pf-verify/interval-property-v2.XXXXXXXX)"
echo "WORKSPACE=$work"
clone_pinned(){
 name="$1";src="$2";sha="$3"
 git clone --quiet --no-hardlinks "$src" "$work/$name"
 git -C "$work/$name" -c advice.detachedHead=false checkout --quiet --detach "$sha"
 test "$(git -C "$work/$name" rev-parse HEAD)" = "$sha"
 echo "VERIFIED_SOURCE=$name:$sha"
}
clone_pinned qualification /mnt/c/Dev/PerfectFoundations/_qualification_interval_m1_20261008 9fe77817542fdc73fa9f968e0ce06a6e80efa18f
base=/home/ubuntu/pf-verify/interval-m1-linux-exact-20261008
clone_pinned interval "$base/interval" 30aca00817bb79f0be3bf43e96c767db9fe45bc5
clone_pinned float "$base/float" bbd8b9e4aac0015facf0bb75819049cee68dc0e7
clone_pinned arithmetic "$base/arithmetic" 1a54d3c7cbbae4e73325cc70fd2777a2427b1504
clone_pinned rational "$base/rational" 35a8e9cc629ee578fe7b624e2134929ce7eeff8a
clone_pinned numeric "$base/numeric" 19b6747cd852a47a694a020b97ba70b6b3ef259b
export PFQ_EXACT_SOURCE_ROOT="$work" CARGO_TARGET_DIR="$work/target"
suite="$work/qualification/qualification/perfect-interval"
manifest="$suite/Cargo.toml"
python3 "$suite/scripts/materialize_exact.py" >"$work/source-graph.log"
want=286537c0f309c87a9377207f6a0e615b96cb3e674c91adcdb3611ffec3e65de7
test "$(sha256sum "$suite/vectors/property_v2.tsv" | cut -c1-64)" = "$want"
python3 "$suite/scripts/generate_properties_v2.py" >"$work/generation.log"
test "$(sha256sum "$suite/vectors/property_v2.tsv" | cut -c1-64)" = "$want"
echo "INDEPENDENT_MPFR_DUAL_PRECISION_CORPUS_REGENERATED_SHA_MATCH"
check(){
 title="$1";shift
 if timeout -k 10s 180s "$@" >"$work/$title.stdout" 2>"$work/$title.stderr";then
    echo "PASS=$title"
    grep -E 'PROPERTY_V2|BALL_12|PF-SEM-V1-FNV64|test result:' "$work/$title.stdout" | head -8 || true
 else
    code=$?
    echo "FAIL=$title code=$code"
    tail -40 "$work/$title.stderr"
    exit "$code"
 fi
}
check fmt cargo +1.99.0 fmt --manifest-path "$manifest" --all -- --check
for mode in all-features no-default-features; do
 check "properties-$mode" cargo +1.99.0 test --offline --manifest-path "$manifest" --"$mode" --test property_v2 -- --nocapture
 check "fingerprint-$mode" cargo +1.99.0 test --offline --manifest-path "$manifest" --"$mode" --test semantic_fingerprint_v1 -- --nocapture
 python3 "$suite/scripts/verify_semantic_fingerprint.py" "$work/fingerprint-$mode.stdout" | tee "$work/fingerprint-$mode.verified"
done
check clippy-std cargo +1.99.0 clippy --offline --manifest-path "$manifest" --all-targets --all-features -- -D warnings
check clippy-nostd cargo +1.99.0 clippy --offline --manifest-path "$manifest" --all-targets --no-default-features -- -D warnings
echo "FINAL_NATIVE_LINUX_PROPERTY_AND_FINGERPRINT_PASS"
