# Simple Redis App on Fly.io

This is a spike on using
* test-driven development using test containers
* redis
* redis as supplied "for free" by fly.io

## Testing

Just needs `cargo test` but uses testcontainers, so needs 
a running docker setup locally

## Deploying to Fly.io

`flyctl deploy --remote-only`

Note: `--remote-only` is needed as for some reason when a local 
docker build is used, it fails with:

```text
#14 58.73    Compiling getrandom v0.1.16
#14 59.00 qemu: uncaught target signal 11 (Segmentation fault) - core dumped
#14 59.01 error: could not compile `typenum`
#14 59.01
#14 59.01 Caused by:
#14 59.01   process didn't exit successfully: `rustc --crate-name build_script_main --edition=2018 /root/.cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.13.0/build/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debug-assertions=off -C metadata=23fe1de9b8e7ca54 -C extra-filename=-23fe1de9b8e7ca54 --out-dir /usr/src/app/target/release/build/typenum-23fe1de9b8e7ca54 -L dependency=/usr/src/app/target/release/deps --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)
#14 59.01 warning: build failed, waiting for other jobs to finish...
#14 61.70 error: build failed
------
Error error building: executor failed running [/bin/sh -c cargo build --release]: exit code: 101
```

Note that this was with docker running on a Mac M1, so may be due to that. 

## Using

To list all states available in region-local Redis in fly.io: go to https://throbbing-sound-1820.fly.dev/

To cause new states to be created / updated:

    curl -X POST https://throbbing-sound-1820.fly.dev/update

