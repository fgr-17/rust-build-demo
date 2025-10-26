#!/bin/sh
cd ${PROJDIR}
cargo add --dev mockall
cargo install cargo-llvm-cov
exec "$@"
