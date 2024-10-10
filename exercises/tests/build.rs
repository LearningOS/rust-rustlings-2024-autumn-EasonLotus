//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("无法获取系统时间")
        .as_secs();

    // 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    if cfg!(test) {
        println!("cargo:rustc-cfg=feature=\"pass\"");

}
}

