#!/usr/bin/env bash
# Offline, exact-source qualification on an authorized local Linux machine.
# No GitHub-hosted runners, registry substitutes, or production checkout writes.
set -euo pipefail

qualification_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd -P)"
suite="$qualification_root/qualification/perfect-decimal"
work_root="${PF_EVIDENCE_ROOT:-$HOME/pf-verify}"
mkdir -p "$work_root"
workspace="$(mktemp -d "$work_root/decimal-qualification.XXXXXXXX")"
summary="$workspace/summary.txt"
printf 'WORKSPACE=%s\n' "$workspace" | tee "$summary"

declare -A pins=(
  [decimal]=85f4408838131007146fff14b1c9c1d760c585a8
  [numeric]=19b6747cd852a47a694a020b97ba70b6b3ef259b
  [arithmetic]=1a54d3c7cbbae4e73325cc70fd2777a2427b1504
  [rational]=97cce0ae8ff7ef206929d273e1d4a5774f6a2a34
)
for tool in git python3 cargo rustup sha256sum; do
  command -v "$tool" >/dev/null || { echo "MISSING_TOOL=$tool" | tee -a "$summary"; exit 20; }
done
rustc +1.99.0 --version | tee -a "$summary"
echo "QUALIFICATION_REV=$(git -C "$qualification_root" rev-parse HEAD)" | tee -a "$summary"
for name in decimal numeric arithmetic rational; do
  var="PF_${name^^}_REPO"
  source_path="${!var:-}"
  if [[ -z "$source_path" || ! -e "$source_path/.git" ]]; then
    echo "MISSING_LOCAL_REPOSITORY=$var" | tee -a "$summary"
    exit 21
  fi
  git -C "$source_path" cat-file -e "${pins[$name]}^{commit}"
  git clone --quiet --no-hardlinks "$source_path" "$workspace/$name"
  git -C "$workspace/$name" -c advice.detachedHead=false checkout --quiet --detach "${pins[$name]}"
  actual="$(git -C "$workspace/$name" rev-parse HEAD)"
  [[ "$actual" == "${pins[$name]}" ]] || { echo "WRONG_SHA=$name" | tee -a "$summary"; exit 22; }
  [[ -z "$(git -C "$workspace/$name" status --porcelain)" ]] || { echo "DIRTY_CLONE=$name"; exit 23; }
  printf 'PIN_%s=%s\n' "$name" "$actual" | tee -a "$summary"
done

cp -a "$suite" "$workspace/consumer"
export QUAL_MANIFEST="$workspace/consumer/Cargo.toml"
export DECIMAL_CARGO_PATH="$workspace/decimal" DECIMAL_SHELL_PATH="$workspace/decimal"
export NUMERIC_CARGO_PATH="$workspace/numeric"
export ARITHMETIC_CARGO_PATH="$workspace/arithmetic"
export RATIONAL_CARGO_PATH="$workspace/rational"
export CARGO_NET_OFFLINE=true CARGO_TERM_COLOR=never
echo "PYTHON=$(python3 --version)" | tee -a "$summary"
echo "TOOLCHAIN=$(cargo +1.99.0 --version)" | tee -a "$summary"
python3 "$workspace/consumer/scripts/verify_oracle.py" | tee "$workspace/python-quantization.log"
python3 "$workspace/consumer/scripts/verify_fingerprint.py" | tee "$workspace/python-representation.log"
python3 "$workspace/consumer/scripts/materialize_private.py" > "$workspace/materialization.log"
printf '%s\n' MATERIALIZATION_PASS | tee -a "$summary"

run_check() {
  local name="$1"
  shift
  if "$@" >"$workspace/$name.stdout" 2>"$workspace/$name.stderr"; then
    printf 'PASS %s\n' "$name" | tee -a "$summary"
    grep -E '^test result:' "$workspace/$name.stdout" | tee -a "$summary" || true
  else
    local rc=$?
    printf 'FAIL %s EXIT=%s\n' "$name" "$rc" | tee -a "$summary"
    tail -n 35 "$workspace/$name.stderr"
    tail -n 35 "$workspace/$name.stdout"
    exit "$rc"
  fi
}
run_check locked_graph cargo +1.99.0 generate-lockfile --manifest-path "$QUAL_MANIFEST" --offline
lock_before="$(sha256sum "$workspace/consumer/Cargo.lock" | cut -d ' ' -f 1)"
run_check metadata cargo +1.99.0 metadata --manifest-path "$QUAL_MANIFEST" --locked --offline --format-version 1
run_check fmt cargo +1.99.0 fmt --manifest-path "$QUAL_MANIFEST" -- --check
run_check tests_all cargo +1.99.0 test --manifest-path "$QUAL_MANIFEST" --locked --offline --all-features
run_check tests_no_default cargo +1.99.0 test --manifest-path "$QUAL_MANIFEST" --locked --offline --no-default-features
run_check clippy_all cargo +1.99.0 clippy --manifest-path "$QUAL_MANIFEST" --locked --offline --all-targets --all-features -- -D warnings
run_check clippy_no_default cargo +1.99.0 clippy --manifest-path "$QUAL_MANIFEST" --locked --offline --all-targets --no-default-features -- -D warnings
run_check rustdoc env RUSTDOCFLAGS=-Dwarnings cargo +1.99.0 doc --manifest-path "$QUAL_MANIFEST" --locked --offline --no-deps --all-features

for target in wasm32v1-none thumbv7em-none-eabihf riscv64imac-unknown-none-elf; do
  if ! rustup +1.99.0 target list --installed | grep -Fxq "$target"; then
    printf 'BLOCKED target_not_installed=%s\n' "$target" | tee -a "$summary"
    exit 24
  fi
  run_check "no_std_$target" cargo +1.99.0 check --manifest-path "$QUAL_MANIFEST" --locked --offline --no-default-features --target "$target"
done

lock_after="$(sha256sum "$workspace/consumer/Cargo.lock" | cut -d ' ' -f 1)"
[[ "$lock_before" == "$lock_after" ]] || { echo "LOCK_DRIFT" | tee -a "$summary"; exit 25; }
printf 'LOCK_SHA256=%s\n' "$lock_after" | tee -a "$summary"
printf 'QUAL_MANIFEST_SHA256=%s\n' "$(sha256sum "$suite/Cargo.toml" | cut -d ' ' -f 1)" | tee -a "$summary"
printf 'HOST_ARCH=%s\n' "$(uname -m)" | tee -a "$summary"
printf 'FINAL=PASS_LOCAL_ONLY\n' | tee -a "$summary"
echo "EVIDENCE=$workspace"
