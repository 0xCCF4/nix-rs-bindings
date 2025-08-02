use std::env;
use std::path::PathBuf;
use pkg_config::Library;

pub fn configure_library<F: FnOnce(&Library, bindgen::Builder) -> bindgen::Builder>(
    pkg_name: &str,
    static_config_environment_overwrite: &str,
    feature_static: bool,
    feature_shared: bool,
    configure_bindings: F,
) -> PathBuf {
    let env_override = env::var(static_config_environment_overwrite)
        .map(|value| value == "1" || value == "static" || value == "true")
        .ok();

    #[allow(unreachable_code)]
    let static_link = if let Some(static_overwrite) = env_override {
        static_overwrite
    } else {
        if feature_shared && feature_static {
            panic!(
                "Both features \"shared\" and \"static\" are set at the same time. This is forbidden. You may override the settings via specifying the NIX_API_STATIC environment variable, which is prioritized over build features."
            );
        } else if !feature_shared && !feature_static {
            println!(
                "cargo:warning=Neither feature 'shared' nor feature 'static' specified -> defaulting to 'shared'."
            );
        }
        feature_static
    };

    let library = pkg_config::Config::new()
        .statik(static_link)
        .probe(pkg_name)
        .unwrap();

    for link_path in &library.link_paths {
        println!(
            "cargo:rustc-link-search=native={}",
            link_path.to_str().expect("non-UTF-8 path")
        );
    }

    for name in &library.libs {
        println!(
            "cargo:rustc-link-lib={}{}",
            if static_link { "static=" } else { "" },
            name
        );
    }

    let bindings = bindgen::Builder::default()
        // defines
        .clang_args(library.defines.iter().map(|(name, value)| {
            if let Some(value) = value {
                format!("-D{name}={value}")
            } else {
                format!("-D{name}")
            }
        })).header_contents("stub.h", "");

    let bindings = configure_bindings(&library, bindings);

    let bindings = bindings
        // Include paths
        .clang_arg(format!("-I{}", env::var("OUT_DIR").unwrap()))
        .clang_args(
            library
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_str().expect("path is valid UTF-8"))),
        )
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

    out_path
}
