extern crate bindgen;

use bindgen::Builder;
use std::path::PathBuf;

fn main() {
    // 指定C头文件路径
    let header_path = PathBuf::from("./include/sharedspice.h");

    // 创建bindgen Builder对象，并配置选项
    let bindings = Builder::default()
        .header(header_path.to_string_lossy())
        .clang_args(&["--includestdbool.h"])
        // .("-include stdbool.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // 生成Rust FFI代码，并输出到指定文件
    bindings
        .write_to_file("./src/sharedspice.rs")
        .expect("Couldn't write bindings!");
}
