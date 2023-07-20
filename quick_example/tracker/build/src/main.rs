use std::{
    fs,
    path::PathBuf,
    process::{
        Command,
        Output,
    },
};

use cbindgen::{
    Builder,
    Config,
    ParseConfig,
};

fn main() {
    let output_dir = PathBuf::from("output");
    fs::create_dir_all(&output_dir).expect("cannot create output directory");

    let tracker_dir = PathBuf::from("../");
    // let target_dir = tracker_dir.join("target/release");
    let target_dir = tracker_dir.join("target/debug");
    let bindings_dir = PathBuf::from("bindings");

    check(
        &Command::new("cargo")
        .current_dir(&tracker_dir)
        // .args(["build", "--release"])
        .arg("build")
        .output()
        .expect("cannot run cargo build"),
    );

    for lib in ["libtracker.a", "libtracker.so"] {
        fs::copy(target_dir.join(lib), output_dir.join(lib)).expect("cannot copy libs");
    }

    // should do that by ourselves, and not rely on a binary (doesn't look to difficult
    // in cargo-expand's source code)
    let output = Command::new("cargo")
        .current_dir(&tracker_dir)
        .arg("expand")
        .output()
        .expect("cannot run cargo expand");
    check(&output);
    fs::write(bindings_dir.join("src/lib.rs"), output.stdout)
        .expect("cannot generate expanded ffi");

    let config = Config {
        cpp_compat: true,
        parse: ParseConfig {
            parse_deps: true,
            include: Some(vec!["pauli_tracker".to_string()]),
            ..Default::default()
        },
        ..Default::default()
    };

    let builder = Builder::new()
        .with_crate(bindings_dir.as_path())
        .with_config(config);

    builder
        .clone()
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("unable to generate C bindings")
        .write_to_file(output_dir.join("tracker.h"));

    // yah, those bindings are not really perfect, i.e., broken; we probably want to use
    // the cxx crate anyway for c++ bindings
    builder
        .generate()
        .expect("unable to generate C++ bindings")
        .write_to_file(output_dir.join("tracker.hpp"));
}

fn check(output: &Output) {
    if !output.status.success() {
        panic!("{:?}", output);
    }
}
