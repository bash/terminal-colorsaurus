# Notes

## Updating the lockfile

Update the lockfile with the MSRV-aware resolver:
```shell
CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback cargo +nightly -Zmsrv-policy generate-lockfile
```
