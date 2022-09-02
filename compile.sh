#!/usr/bin/sh

mkdir -p bin
cd src/client

echo Compiling nvda2speechd64.dll
cargo build --release --target x86_64-pc-windows-gnu -q

echo Compiling nvda2speechd32.dll
cargo build --release --target i686-pc-windows-gnu -q

cp target/x86_64-pc-windows-gnu/release/nvda2speechd.dll ../../bin/nvda2speechd64.dll
cp target/i686-pc-windows-gnu/release/nvda2speechd.dll ../../bin/nvda2speechd32.dll

cd ../server

echo Compiling nvda2speechd
cargo build --release -q
cp target/release/nvda2speechd ../../bin

