# Building and testing locally

The unit tests live in `replay-engine/src/tests/` and run entirely offline — no
MySQL, no archive download, no RPC. What they do need is one native dependency
set and one fixture program that is intentionally not committed.

## Native dependencies

The workspace pins `solana-* =1.17.22`, `rocksdb 0.21`, and `reqwest 0.11` with
`native-tls`. Two build scripts fail when the headers are missing:

```
openssl-sys       → "Could not find directory of OpenSSL installation"
librocksdb-sys    → "'stdarg.h' file not found"   (bindgen needs clang headers)
```

On Debian/Ubuntu:

```bash
sudo apt-get install -y libssl-dev libclang-dev clang libz-dev cmake pkg-config libudev-dev
```

Without root, `apt-get download` plus `dpkg -x` is enough — `openssl-sys` honors
`OPENSSL_INCLUDE_DIR` / `OPENSSL_LIB_DIR`, `bindgen` honors
`BINDGEN_EXTRA_CLANG_ARGS`. On Ubuntu the arch-specific `opensslconf.h` sits in
`usr/include/<triple>/openssl/`, so merge that directory into the include path
you hand to the compiler, otherwise the header expansion still fails.

## Test fixture program

`replay-engine/src/tests/mod.rs` does

```rust
pub const WHIRLPOOL_PROGRAM_FOR_TESTING: &[u8] = include_bytes!("whirlpool-for-testing.so");
```

and that `.so` is listed in `.gitignore` (last line), so it is absent from a
fresh clone. Build it from the pinned Whirlpool program before running the suite:

```bash
git clone -b yugure/for-replayer-20250824 https://github.com/orca-so/whirlpools
cd whirlpools/programs/whirlpool && cargo build-sbf
cp target/deploy/whirlpool.so \
   <this-repo>/replay-engine/src/tests/whirlpool-for-testing.so
```

The program version matters — an older binary silently fails the tests that
cover newer instructions. Measured against the binaries committed under
`replay-engine/src/programs/whirlpool/`:

| fixture program       | `cargo test -p replay-engine --lib` |
| --------------------- | ----------------------------------- |
| `whirlpool-20261015…` | 33 passed, 0 failed                 |
| `whirlpool-20260204…` | 33 passed, 0 failed                 |
| `whirlpool-20250505…` | 31 passed, 2 failed                 |
| `whirlpool-20220407…` | 26 passed, 7 failed                 |

`.github/workflows/ci.yml` uses the newest committed binary, which keeps the
same 33/33 result without needing `cargo-build-sbf` on the runner.

## Run the suite

```bash
cargo test --workspace
```