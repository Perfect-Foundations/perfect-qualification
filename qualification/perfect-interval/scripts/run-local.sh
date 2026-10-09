#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"
root="${PFQ_EVIDENCE_ROOT:-$HOME/pf-verify}"
mkdir -p "$root"
work="$(mktemp -d "$root/interval-mpfr-qualification.XXXXXXXX")"
summary="$work/summary.txt"
echo "WORKSPACE=$work" | tee "$summary"
for line in \
 "qualification:PFQ_QUAL_REPO:${PFQ_QUAL_REPO:?}" \
 "interval:PFQ_INTERVAL_REPO:${PFQ_INTERVAL_REPO:?}" \
 "float:PFQ_FLOAT_REPO:${PFQ_FLOAT_REPO:?}" \
 "arithmetic:PFQ_ARITHMETIC_REPO:${PFQ_ARITHMETIC_REPO:?}" \
 "rational:PFQ_RATIONAL_REPO:${PFQ_RATIONAL_REPO:?}" \
 "numeric:PFQ_NUMERIC_REPO:${PFQ_NUMERIC_REPO:?}";do
  name="${line%%:*}"
  source="${line#*:*:}"
  git clone --quiet --no-hardlinks "$source" "$work/$name"
done
for spec in \
 "interval:30aca00817bb79f0be3bf43e96c767db9fe45bc5" \
 "float:bbd8b9e4aac0015facf0bb75819049cee68dc0e7" \
 "arithmetic:1a54d3c7cbbae4e73325cc70fd2777a2427b1504" \
 "rational:35a8e9cc629ee578fe7b624e2134929ce7eeff8a" \
 "numeric:19b6747cd852a47a694a020b97ba70b6b3ef259b";do
   n="${spec%%:*}"
   sha="${spec#*:}"
   git -C "$work/$n" -c advice.detachedHead=false checkout --quiet --detach "$sha"
   test "$(git -C "$work/$n" rev-parse HEAD)" = "$sha"
   echo "SOURCE_$n=$sha" | tee -a "$summary"
done
qual_sha="$(git -C "$work/qualification" rev-parse HEAD)"
test "$qual_sha" = "${PFQ_QUAL_SHA:?set exact checked-out qualification SHA}"
echo "QUALIFICATION_SHA=$qual_sha" | tee -a "$summary"
export PFQ_EXACT_SOURCE_ROOT="$work" CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$work/target"
source_dir="$work/qualification/qualification/perfect-interval"
manifest="$source_dir/Cargo.toml"
python3 "$source_dir/scripts/materialize_exact.py" | tee "$work/materialize.log"
original_digest="$(sha256sum "$source_dir/vectors/real_m1_mpfr_20261008.tsv" | cut -d' ' -f1)"
test "$original_digest" = "604bf0ae6b6d506c4aaa93ebe0e374a23fb7b1327222b9a53aa43293d163c744"
if python3 -c 'import ctypes; ctypes.CDLL("libmpfr.so.6")' 2>/dev/null;then
   python3 "$source_dir/scripts/generate_vectors.py" | tee "$work/mpfr-generation.log"
   test "$original_digest" = "$(sha256sum "$source_dir/vectors/real_m1_mpfr_20261008.tsv" | cut -d' ' -f1)"
   echo "MPFR_INDEPENDENT_REGEN_PASS" | tee -a "$summary"
else
   echo "MPFR_REGEN_NOT_RUN_RUNTIME_LIBRARY_ABSENT" | tee -a "$summary"
fi
if [ -n "${PFQ_MPFI_LIB:-}" ] && [ -f "$PFQ_MPFI_LIB" ];then
   python3 "$source_dir/scripts/verify_mpfi_reference.py" | tee "$work/mpfi-reference.log"
   echo "MPFI_1_5_3_INDEPENDENT_REFERENCE_PASS" | tee -a "$summary"
else
   echo "MPFI_REFERENCE_NOT_RUN_LIBRARY_NOT_PROVIDED" | tee -a "$summary"
fi
new_digest="$(sha256sum "$source_dir/vectors/arb_adversarial_v1.tsv" | cut -d' ' -f1)"
test "$new_digest" = "8bb8011b5bf4e3292ea631df2d8e5f13bbdbb6e08f1df2eea8402b39fd9f9e57"
python3 "$source_dir/scripts/verify_arb_output.py" | tee "$work/arb-committed-reference.log"
python3 "$source_dir/scripts/verify_arb_corners.py" | tee "$work/arb-committed-corner-reference.log"
echo "ARB_POINT_CORNER_CACHED_EXACT_FRACTION_330_CERTIFIED_NOT_LIVE_REGENERATED" | tee -a "$summary"
echo "ARB_CACHED_CERTIFICATE_MATHEMATICALLY_VALIDATED_NOT_LIVE_REGENERATED" | tee -a "$summary"
if python3 -c 'import ctypes; ctypes.CDLL("libmpfr.so.6")' 2>/dev/null;then
   python3 "$source_dir/scripts/generate_arb_adversarial.py" | tee "$work/arb-adversarial-generation.log"
   test "$new_digest" = "$(sha256sum "$source_dir/vectors/arb_adversarial_v1.tsv" | cut -d' ' -f1)"
   echo "ARB_ADVERSARIAL_MPFR_REGEN_PASS_330" | tee -a "$summary"
else
   echo "ARB_ADVERSARIAL_MPFR_REGEN_NOT_RUN_MISSING_LIBMPFR" | tee -a "$summary"
fi
if [ -n "${PFQ_FLINT_ROOT:-}" ] && [ -f "$PFQ_FLINT_ROOT/usr/include/flint/arb.h" ];then
  inc="$PFQ_FLINT_ROOT/usr/include"
  lib="$PFQ_FLINT_ROOT/usr/lib/x86_64-linux-gnu"
  cc -std=c11 -O2 -Wall -Wextra -Werror -I"$inc" -I"$inc/x86_64-linux-gnu" -L"$lib" -Wl,-rpath,"$lib" "$source_dir/scripts/flint_arb_oracle.c" -lflint -o "$work/flint-arb-reference"
  "$work/flint-arb-reference" "$source_dir/vectors/arb_adversarial_v1.tsv" >"$work/arb-raw.tsv" 2>"$work/arb-runtime.log"
  python3 "$source_dir/scripts/verify_arb_output.py" "$work/arb-raw.tsv" | tee "$work/arb-exact-rational.log"
  cc -std=c11 -O2 -Wall -Wextra -Werror -I"$inc" -I"$inc/x86_64-linux-gnu" -L"$lib" -Wl,-rpath,"$lib" "$source_dir/scripts/flint_arb_corners.c" -lflint -o "$work/flint-arb-corners"
  "$work/flint-arb-corners" "$source_dir/vectors/arb_adversarial_v1.tsv" >"$work/arb-corners-live.tsv" 2>"$work/arb-corners-runtime.log"
  python3 "$source_dir/scripts/verify_arb_corners.py" "$work/arb-corners-live.tsv" | tee "$work/arb-corners-exact-rational.log"
  echo "FLINT_ARB_3_0_1_292_FINITE_NATIVE_INTERVAL_AND_38_NONFINITE_UNSUPPORTED; 330_POINT_CORNER_CERTIFIED" | tee -a "$summary"
else
  echo "FLINT_ARB_REFERENCE_NOT_RUN_ISOLATED_SYSROOT_NOT_PROVIDED" | tee -a "$summary"
fi
echo "HOST=$(uname -sm)" | tee -a "$summary"
rustc +1.99.0 --version | tee -a "$summary"
check() {
  label="$1";shift
  if timeout -k 10s 180s "$@" >"$work/$label.stdout" 2>"$work/$label.stderr";then
    echo "PASS=$label EXIT=0" | tee -a "$summary"
    grep -E 'test result:|MPFR_BALL_CASES=' "$work/$label.stdout" | tee -a "$summary" || true
  else
    code=$?
    echo "FAIL=$label EXIT=$code" | tee -a "$summary"
    tail -40 "$work/$label.stderr"
    exit "$code"
  fi
}
check lock cargo +1.99.0 generate-lockfile --offline --manifest-path "$manifest"
check fmt cargo +1.99.0 fmt --manifest-path "$manifest" --all -- --check
check tests_std cargo +1.99.0 test --offline --locked --manifest-path "$manifest" --all-features
check tests_no_default cargo +1.99.0 test --offline --locked --manifest-path "$manifest" --no-default-features
check clippy_std cargo +1.99.0 clippy --offline --locked --manifest-path "$manifest" --all-targets --all-features -- -D warnings
check clippy_no_default cargo +1.99.0 clippy --offline --locked --manifest-path "$manifest" --all-targets --no-default-features -- -D warnings
check doc env RUSTDOCFLAGS="-D warnings" cargo +1.99.0 doc --offline --locked --manifest-path "$manifest" --no-deps --all-features
for t in wasm32v1-none thumbv7em-none-eabihf riscv64imac-unknown-none-elf;do
 if rustup +1.99.0 target list --installed | grep -Fxq "$t";then
  check "classb_$t" cargo +1.99.0 check --offline --locked --manifest-path "$manifest" --lib --no-default-features --target "$t"
 else
  echo "BLOCKED_CLASS_B_TARGET=$t" | tee -a "$summary"
 fi
done
sha256sum "$source_dir/Cargo.lock" | tee -a "$summary"
echo "FINAL_INTERVAL_QUALIFICATION_LOCAL_PASS_NATIVE_$(uname -m)" | tee -a "$summary"
