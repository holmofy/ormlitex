set dotenv-load := true

test:
    just attr/test
    just core/test
    just macro/test
    just ormlitex/test
    just cli/build

# Bump version. level=major,minor,patch
version level:
   #!/bin/bash -euxo pipefail
   git diff-index --exit-code HEAD > /dev/null || ! echo You have untracked changes. Commit your changes before bumping the version. || exit 1

   echo $(dye -c INFO) Make sure that it builds first.
   (cd ormlitex && cargo build --features runtime-tokio-rustls,sqlite)

   cargo set-version --bump {{ level }} --workspace
   VERSION=$(toml get ormlitex/Cargo.toml package.version)

   toml set macro/Cargo.toml dependencies.ormlitex-core.version $VERSION
   (cd macro && cargo update)
   toml set ormlitex/Cargo.toml dependencies.ormlitex-core.version $VERSION
   toml set ormlitex/Cargo.toml dependencies.ormlitex-macro.version $VERSION
   (cd ormlitex && cargo update)

   git commit -am "Bump version {{level}} to $VERSION"
   git tag v$VERSION
   git push

patch:
    just version patch
    just publish

publish:
   cd attr && cargo publish
   cd core && cargo publish --features sqlite,postgres,mysql,runtime-tokio-rustls
   cd macro && cargo publish --features sqlite,postgres,mysql,runtime-tokio-rustls
   cd ormlitex && cargo publish --features sqlite,postgres,mysql
   cd cli && cargo publish

doc:
   cd ormlitex && RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open -p ormlitex --no-deps

install:
    @just cli/install
