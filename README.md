# Android Rust JNI Demo

## Build
```shell
cd rust
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 -o ../android/app/src/main/jniLibs build --release

cd ../android
./gradlew clean installDebug
```

[cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
