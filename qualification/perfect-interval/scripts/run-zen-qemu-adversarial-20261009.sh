#!/usr/bin/env bash
set -euo pipefail
test "$(uname -m)" = "x86_64"
test "$(id -u)" = "10001"
test ! -S /var/run/docker.sock
test "$(awk '/^CapEff:/ {print $2}' /proc/self/status)" = "0000000000000000"
for spec in "qualification:215ec5f3f5f72ad0e200a0dee3034f97fa5b68ea" "interval:30aca00817bb79f0be3bf43e96c767db9fe45bc5" "float:bbd8b9e4aac0015facf0bb75819049cee68dc0e7" "arithmetic:1a54d3c7cbbae4e73325cc70fd2777a2427b1504" "rational:35a8e9cc629ee578fe7b624e2134929ce7eeff8a" "numeric:19b6747cd852a47a694a020b97ba70b6b3ef259b"; do
 name="$(printf '%s' "$spec" | cut -d: -f1)"
 sha="$(printf '%s' "$spec" | cut -d: -f2)"
 test "$(git -c safe.directory="/workspace/$name" -C "/workspace/$name" rev-parse HEAD)" = "$sha"
 echo "QEMU_EXACT_SOURCE_$name=$sha"
done
echo "QEMU_HOST=$(uname -sm) USER_ID=$(id -u)"
qemu-aarch64 --version | head -1
rustc +1.99.0 --version
src=/workspace/qualification/qualification/perfect-interval
test "$(sha256sum "$src/vectors/arb_adversarial_v1.tsv" | cut -c1-64)" = "8bb8011b5bf4e3292ea631df2d8e5f13bbdbb6e08f1df2eea8402b39fd9f9e57"
mkdir -p /tmp/interval-qual/.cargo
cp -a "$src/." /tmp/interval-qual/
printf '[source.crates-io]\nreplace-with = "vendored-sources"\n[source.vendored-sources]\ndirectory = "/workspace/vendor"\n' > /tmp/interval-qual/.cargo/config.toml
export CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=/tmp/interval-qual-target
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu"
manifest=/tmp/interval-qual/Cargo.toml
cd /tmp/interval-qual
cargo +1.99.0 generate-lockfile --offline --manifest-path "$manifest"
for mode in all-features no-default-features;do
 for testmod in arb_adversarial domain_semantics;do
  timeout -k 10s 120s cargo +1.99.0 test --offline --locked --manifest-path "$manifest" --target aarch64-unknown-linux-gnu --"$mode" --test "$testmod" -- --nocapture
  echo "QEMU_PASS=$mode $testmod EXIT=0"
 done
done
echo "FINAL_QEMU_AARCH64_NEW_ARB_ADVERSARIAL_PASS_EMULATED_NOT_NATIVE"
