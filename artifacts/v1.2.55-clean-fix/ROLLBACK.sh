#!/bin/sh
set -eu
# Roll back a copy, never the working tree.
cp MODIFIED_FILE rollback-copy-Cargo.toml
sed -i 's/version = "1.2.55"/version = "1.2.54"/' rollback-copy-Cargo.toml
