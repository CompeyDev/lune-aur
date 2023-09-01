#!/bin/bash

set -e

cd lune-src
current_ver=$(git describe --tags --abbrev=0)

[[ $(cat ../build.VERSION) == $current_ver ]] && echo "[!] Already at latest build, skipping" && exit 0

wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip && unzip android-ndk-*.zip
mv android-ndk-r25c $HOME/android-ndk

export PATH="$PATH:$HOME/android-ndk"
export ANDROID_NDK=$HOME/android-ndk
export ANDROID_NDK_HOME=$HOME/android-ndk
export PATH="$PATH:$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/bin/"

cp ../package/android/.cargo/config $HOME/.cargo/config

rustup toolchain install stable-x86_64-unknown-linux-gnu
rustup target add aarch64-linux-android

mv $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-clang 
mv $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar

cargo build --release --target aarch64-linux-android

# Create outputs
cp $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so build/libc++_shared.so
cp target/aarch64-linux-android/release/lune build/lune

zip "lune-$current_ver-aarch64-linux-android.zip" build/* 
