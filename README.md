# Vehicles

Provides vehicle manufacturers and their makes using NHTSA's [Vehicle API](https://vpic.nhtsa.dot.gov/api/)

## Platform Integration - Android

Integration with Android requires generating libraries for each supported architecture plus generating a foreign-function interface (FFI) to be used for calling code.
You'll clone this repo, generate both, then add the generated files into an Android project.

### Rust (this repo)

1. Clone this repo and ensure you've set up a Rust environment (ie, `cargo`). Then, add the 4 Android architectures with `cargo add --target`:

- aarch64-linux-android
- armv7-linux-androideabi
- i686-linux-android
- x86_64-linux-android

2. Run `cargo build --release` to generate a library for each architecture
3. Run `cargo run --features=uniffi/cli --bin uniffi-bindgen generate --library target/release/libvehicles.so --language kotlin --out-dir out` to generate a Kotlin file in the `/out` directory.

### Android (your project)

1. Create a `/jniLibs` directory in your desired module (ie, `/src/main/jniLibs`)
2. Create child directories for each architecture:

- arm64-v8a
- armeabi-v7a
- x86
- x86_64

3. Add each generated library from the Rust project into its respective architecture directory in the Android project
4. Create a new directory structure for the Kotlin FFI in your `java` or `kotlin` directory according to the package declaration in the generated Kotlin file (ie, `/src/main/java/uniffi/vehicles`)
5. Add the Kotlin file to the new directory.
6. To call code from the Rust library, import and use declarations from the Kotlin file.
