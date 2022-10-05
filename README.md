# p2panda node in Android app

> :warn: This is an experimental setup to see if we can bring the p2panda `aquadoggo` node into an native Android app! And yes, it works! Give me some time to clean this up and write more documentation.

*Please note: This was tested with a smartphone with ARM64 architecture.*

## Requirements

* Rust
* Android NDK
* Android SDK

## Usage

```bash
# Setup
cargo install cargo-apk

# Build native `.so` library
cargo apk build --release

# Copy library to Android project
cp ./target/release/apk/lib/arm64-v8a/libaquadoggo_android.so ../android/Doggodroid/app/src/main/jniLibs/arm64-v8a

# Open the Android project `Doggodroid` in Android Studio and run it from there ..!
```

## Notes

* This uses `ndk-glue` to activate the Android Logcat logger, through this we get nice logging, though it could also be realized without `ndk-glue` and just done directly with `android_logger` crate
* `android:usesCleartextTraffic="true"` in `AndroidManifest.xml` is important to make requests using `http://` work

## Links

* https://github.com/rust-windowing/android-ndk-rs
* https://github.com/tauri-apps/cargo-mobile
* https://github.com/p2panda/aquadoggo
