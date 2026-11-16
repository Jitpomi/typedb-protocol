# Maintained compatibility branch

Base: upstream `3.13.4`. All upstream source, history and licenses are retained.

Tonic/Prost 0.14; regenerate from unchanged protocol definitions. TLS tests require trusted roots and a matching hostname. Vendored protoc comes from the standard registry and is selected explicitly, without changing process environment.

This is an explicit JITPOMI-maintained source branch, not a claim that upstream released this change. Applications must pin the reviewed commit. No advisory ignores, renamed copies of the obsolete dependencies, TLS bypasses or target filters are used.

Validation: `cargo test --manifest-path grpc/rust/Cargo.toml` plus DogRS/Business integration and audit checks. On upstream release, verify the source/API and stored-format compatibility, rerun the same checks, and replace the source pin with the maintained registry release.
