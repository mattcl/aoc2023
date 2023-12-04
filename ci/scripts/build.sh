#!/bin/sh
set -e

# unpack and install the tools
cd aoc-tools
tar -xvf aoc-tools-*-x86_64-unknown-linux-musl.tar.gz
mv aoc-tools /usr/local/bin/
cd ../
aoc-tools --version

# our task has called our input repo
cd repo

# fail fast if we won't pass a simple cargo check
cargo check --all-targets

# run the unit tests
cargo test

# run integration tests
just test

# run benchmarks
just bench-all

# summary
set -ex
aoc-tools criterion-summary target/criterion

# build the cli
just build-cli

# package the release into a tarball for export

# walk up a dir to be at the same level with the export release dir
cd ../

# this is fragile: we're going to assume we always have a --version flag
VERSION=$("repo/target/release/aoc" --version | cut -d " " -f 2)

# we need a way to reference the version
echo "$VERSION" > release/VERSION

echo "Packaging $VERSION for $TARGET"

mkdir dist
cp "repo/target/release/aoc" dist/

cd dist
ARCHIVE="aoc-${VERSION}-${TARGET}.tar.gz"
tar czf "$ARCHIVE" "aoc"
cd ../

mv "dist/$ARCHIVE" "release/$ARCHIVE"

# we need a way to reference the file name
echo "$ARCHIVE" > release/ARCHIVE_NAME
