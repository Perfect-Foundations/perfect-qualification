#!/usr/bin/env bash
# Exact-source offline qualification, independent from repository worktrees.
set -euo pipefail
source_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd -P)"
evidence_root="${PF_EVIDENCE_ROOT:-$HOME/pf-verify}"
mkdir -p "$evidence_root"
workspace="$(mktemp -d "$evidence_root/number-theory-qualification.XXXXXXXX")"
summary="$workspace/summary.txt"
echo "WORKSPACE=$workspace" | tee "$summary"

declare -A pin=(
[number-theory]=41aaa4763da2d11aea9e65666f9ed7f84d43ed61
[arithmetic]=9ec660f9abfd7fe93e441e77dbb1158c371603f5
[numeric]=19b6747cd852a47a694a020b97ba70b6b3ef259b
)
for dep in number-theory arithmetic numeric; do
  var="PF_$(echo "$dep" | tr '[:lower:]-' '[:upper:]_')_REPO"
  src="${!var:-}"
  if [[ -z "$src" || ! -e "$src/.git" ]]; then echo "MISSING_SOURCE=$var" | tee -a "$summary";exit 21;fi
  git -C "$src" cat-file -e "${pin[$dep]}^{commit}"
  git clone --quiet --no-hardlinks "$src" "$workspace/$dep"
  git -C "$workspace/$dep" -c advice.detachedHead=false checkout --quiet --detach "${pin[$dep]}"
  actual="$(git -C "$workspace/$dep" rev-parse HEAD)"
  [[ "$actual" == "${pin[$dep]}" ]] || exit 22
  echo "PIN_$dep=$actual" | tee -a "$summary"
done
qualification_head="$(git -C "$source_root" rev-parse HEAD)"
git clone --quiet --no-hardlinks "$source_root" "$workspace/qualification"
git -C "$workspace/qualification" -c advice.detachedHead=false checkout --quiet --detach "$qualification_head"
echo "QUALIFICATION_HEAD=$qualification_head" | tee -a "$summary"
export NUMBER_THEORY_CARGO_PATH="$workspace/number-theory"
export ARITHMETIC_CARGO_PATH="$workspace/arithmetic"
export NUMERIC_CARGO_PATH="$workspace/numeric"
export QUAL_MANIFEST="$workspace/qualification/qualification/perfect-number-theory/Cargo.toml"
qual="$(dirname "$QUAL_MANIFEST")"
export CARGO_NET_OFFLINE=true CARGO_TERM_COLOR=never
rustc +1.99.0 --version | tee -a "$summary"
python3 "$qual/scripts/verify_oracle.py" | tee "$workspace/oracle.log"
python3 "$qual/scripts/verify_fingerprint.py" | tee "$workspace/fingerprint.log"
python3 "$qual/scripts/materialize_private.py" | tee "$workspace/materializer.log"

check() {
 local name="$1";shift
 echo "BEGIN=$name" | tee -a "$summary"
 if timeout -k 10s 180s "$@" > "$workspace/$name.stdout" 2>"$workspace/$name.stderr";then
  echo "PASS=$name EXIT=0" | tee -a "$summary"
  grep '^test result:' "$workspace/$name.stdout" | tee -a "$summary" || :
 else
  rc=$?
  echo "FAIL=$name EXIT=$rc" | tee -a "$summary"
  tail -n 25 "$workspace/$name.stderr"
  exit "$rc"
 fi
}
check lock cargo +1.99.0 generate-lockfile --offline --manifest-path "$QUAL_MANIFEST"
lock_before="$(sha256sum "$qual/Cargo.lock"|cut -d' ' -f1)"
check metadata cargo +1.99.0 metadata --offline --locked --manifest-path "$QUAL_MANIFEST" --format-version 1
check fmt cargo +1.99.0 fmt --manifest-path "$QUAL_MANIFEST" -- --check
check tests_all cargo +1.99.0 test --offline --locked --manifest-path "$QUAL_MANIFEST" --all-features
check tests_no_default cargo +1.99.0 test --offline --locked --manifest-path "$QUAL_MANIFEST" --no-default-features
check clippy_all cargo +1.99.0 clippy --offline --locked --manifest-path "$QUAL_MANIFEST" --all-targets --all-features -- -D warnings
check clippy_no_default cargo +1.99.0 clippy --offline --locked --manifest-path "$QUAL_MANIFEST" --all-targets --no-default-features -- -D warnings
check rustdoc env RUSTDOCFLAGS="-D warnings" cargo +1.99.0 doc --offline --locked --manifest-path "$QUAL_MANIFEST" --all-features --no-deps
for target in wasm32v1-none thumbv7em-none-eabihf riscv64imac-unknown-none-elf; do
 if ! rustup +1.99.0 target list --installed | grep -Fxq "$target";then
   echo "BLOCKED_TARGET=$target" | tee -a "$summary"; exit 24
 fi
 check "nostd_$target" cargo +1.99.0 check --offline --locked --manifest-path "$QUAL_MANIFEST" --no-default-features --target "$target"
done
lock_after="$(sha256sum "$qual/Cargo.lock"|cut -d' ' -f1)"
[[ "$lock_before" == "$lock_after" ]] || { echo "LOCK_DRIFT" | tee -a "$summary";exit 25; }
echo "LOCK_SHA256=$lock_after" | tee -a "$summary"
echo "HOST=$(uname -sm)" | tee -a "$summary"
echo "FINAL=PASS_LOCAL_ONLY" | tee -a "$summary"
