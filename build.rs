
use std::env;
use std::process::Command;

fn main() {
    let work_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(not(feature="device-selected"))]
        let platform_subdir = "cmsis_freertos_stm32f4";

    #[cfg(feature = "stm32f3x")]
        let platform_subdir = "cmsis_freertos_stm32f3";
    #[cfg(feature = "stm32f4x")]
        let platform_subdir = "cmsis_freertos_stm32f4";
    #[cfg(feature = "stm32f7x")]
        let platform_subdir = "cmsis_freertos_stm32f7";
    #[cfg(feature = "stm32h7x")]
        let platform_subdir = "cmsis_freertos_stm32h7";

    let platform_dir = format!("{}/platforms/{}", work_dir, platform_subdir);

    // clean library each time?
    Command::new("make")
        .current_dir(platform_dir.clone())
        .arg("clean")
        .output()
        .expect("make clean failed ");

    let debug_flag = "DEBUG=0"; // DEBUG = 1
    let opt_flag = "OPT=-O1"; // OPT = -Og
    Command::new("make")
        .current_dir(platform_dir.clone())
        .arg(debug_flag)
        .arg(opt_flag)
        .output()
         .expect("make failed ");

    // static library should now be in the build directory of the specific platform
    println!("cargo:rustc-link-search={}/build/", platform_dir.clone());
    // link the generated library
    println!("cargo:rustc-link-lib=cmsis_rtos2");

}