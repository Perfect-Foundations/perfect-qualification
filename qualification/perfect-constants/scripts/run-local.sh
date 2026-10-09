#!/usr/bin/env bash
# Offline, exact-SHA local qualification. Source and other worktrees untouched.
set -euo pipefail
QUAL_SRC="${PF_QUALIFICATION_REPO:?set PF_QUALIFICATION_REPO to a clean qualification git checkout}"
CONSTANTS_SRC="${PF_CONSTANTS_REPO:?set PF_CONSTANTS_REPO to a Constants git checkout}"
PIN="d88d8055f45fb075c905f1fe9e82a623029faf41"
test -d "$QUAL_SRC/.git" && test -d "$CONSTANTS_SRC/.git"
test -z "$(git -C "$QUAL_SRC" status --porcelain)" || { echo "QUALIFICATION_CHECKOUT_DIRTY"; exit 21; }
qual_head="$(git -C "$QUAL_SRC" rev-parse HEAD)"
git -C "$CONSTANTS_SRC" cat-file -e "$PIN^{commit}"
root="${PF_EVIDENCE_ROOT:-$HOME/pf-verify}"
mkdir -p "$root"
workspace="$(mktemp -d "$root/constants-qualification.XXXXXXXX")"
summary="$workspace/summary.txt"
echo "WORKSPACE=$workspace" | tee "$summary"
echo "QUALIFICATION_SHA=$qual_head" | tee -a "$summary"
for pair in "constants:$CONSTANTS_SRC" "qualification:$QUAL_SRC"; do
  name="${pair%%:*}"
  src="${pair#*:}"
  git clone --quiet --no-hardlinks "$src" "$workspace/$name"
done
git -C "$workspace/constants" -c advice.detachedHead=false checkout --quiet --detach "$PIN"
git -C "$workspace/qualification" -c advice.detachedHead=false checkout --quiet --detach "$qual_head"
test "$(git -C "$workspace/constants" rev-parse HEAD)" = "$PIN"
test "$(git -C "$workspace/qualification" rev-parse HEAD)" = "$qual_head"
echo "CONSTANTS_SHA=$PIN" | tee -a "$summary"
qual="$workspace/qualification/qualification/perfect-constants"
manifest="$qual/Cargo.toml"
export CONSTANTS_CARGO_PATH="$workspace/constants" CARGO_NET_OFFLINE=true CARGO_TERM_COLOR=never
export CARGO_TARGET_DIR="$workspace/target"
echo "HOST=$(uname -sm)" | tee -a "$summary"
rustc +1.99.0 --version | tee -a "$summary"
python3 "$qual/scripts/verify_oracle.py" | tee "$workspace/oracle.log"
python3 "$qual/scripts/materialize_private.py" | tee "$workspace/materializer.log"
grep '^PINNED_ORIGINAL_GIT_DEP' "$workspace/materializer.log" | tee -a "$summary"
sha256sum "$qual/Cargo.toml.original-git-dependency" | tee -a "$summary"
check() {
  local label="$1"; shift
  echo "BEGIN=$label" | tee -a "$summary"
  if timeout -k 10s 180s "$@" > "$workspace/$label.stdout" 2> "$workspace/$label.stderr"; then
    echo "PASS=$label EXIT=0" | tee -a "$summary"
    grep 'test result:\|CONSTANTS_INDEPENDENT_LOOKUP_ALLOCATIONS' "$workspace/$label.stdout" | tee -a "$summary" || true
  else
    local code=$?
    echo "FAIL=$label EXIT=$code" | tee -a "$summary"
    tail -n 40 "$workspace/$label.stderr"
    exit "$code"
  fi
}
check lock cargo +1.99.0 generate-lockfile --offline --manifest-path "$manifest"
lock_before="$(sha256sum "$qual/Cargo.lock" | cut -d' ' -f1)"
check metadata cargo +1.99.0 metadata --offline --locked --format-version 1 --manifest-path "$manifest"
check fmt cargo +1.99.0 fmt --manifest-path "$manifest" --all -- --check
check tests_all cargo +1.99.0 test --offline --locked --manifest-path "$manifest" --all-features
check tests_no_default cargo +1.99.0 test --offline --locked --manifest-path "$manifest" --no-default-features
check clippy_all cargo +1.99.0 clippy --offline --locked --manifest-path "$manifest" --all-targets --all-features -- -D warnings
check clippy_no_default cargo +1.99.0 clippy --offline --locked --manifest-path "$manifest" --all-targets --no-default-features -- -D warnings
check rustdoc env RUSTDOCFLAGS="-D warnings" cargo +1.99.0 doc --offline --locked --manifest-path "$manifest" --no-deps --all-features
check alloc_all cargo +1.99.0 run --offline --locked --manifest-path "$manifest" --all-features --example alloc_probe
check alloc_no_default cargo +1.99.0 run --offline --locked --manifest-path "$manifest" --no-default-features --example alloc_probe
for target in wasm32v1-none thumbv7em-none-eabihf riscv64imac-unknown-none-elf; do
  if ! rustup +1.99.0 target list --installed | grep -Fxq "$target"; then
    echo "BLOCKED_TARGET=$target" | tee -a "$summary"
    exit 24
  fi
  check "classb_$target" cargo +1.99.0 check --offline --locked --manifest-path "$manifest" --lib --no-default-features --target "$target"
done
lock_after="$(sha256sum "$qual/Cargo.lock" | cut -d' ' -f1)"
test "$lock_before" = "$lock_after" || { echo "LOCK_DRIFT" | tee -a "$summary"; exit 25; }
echo "DEV_LOCK_SHA256=$lock_after" | tee -a "$summary"
echo "FINAL=PASS_LOCAL_ONLY" | tee -a "$summary"
