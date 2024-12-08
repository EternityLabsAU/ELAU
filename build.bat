echo General Cargo build
cargo build
pause
echo Android Cargo build
cargo ndk --target aarch64-linux-android build
pause