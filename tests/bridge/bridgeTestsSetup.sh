#!/bin/bash
pushd lib || {
	echo "Failed to enter lib directory"
	exit 1
}
cargo build --bin lib || {
	echo "Cargo build failed"
	popd
	exit 1
}
popd

mkdir -p tests/bridge/.temp || {
	echo "Failed to create .temp directory"
	exit 1
}

cp lib/target/debug/lib tests/bridge/.temp || {
	echo "Failed to copy binary"
	exit 1
}
