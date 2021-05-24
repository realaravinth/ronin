#!/bin/bash

for arch in x86_64-unknown-linux-musl x86_64-pc-windows-gnu aarch64-linux-android arm-linux-androideabi
do 
	cross build  --package ronin-cli  --release --target $arch
done
