use std::{env, fs};
use std::path::PathBuf;

fn main() {
    nix_sys_common_build::configure_library(
        "nix-expr-c",
        "NIXEXPR_STATIC",
        cfg!(feature = "static"),
        cfg!(feature = "shared"),
        |lib, builder| {
            let out_dir = env::var("OUT_DIR").unwrap();

            for include_path in &lib.include_paths {
                let nix_api_value = include_path.join("nix_api_value.h");
                if nix_api_value.exists() {
                    let content = fs::read_to_string(nix_api_value).unwrap();
                    let nix_api_value = PathBuf::from(&out_dir).join("nix_api_value.h");
                    let content = content.replace(
                        r#"[[deprecated("use nix_value instead")]] typedef nix_value Value;"#,
                        "",
                    );
                    fs::write(nix_api_value, content).expect("Could not write to nix_api_value.h");
                    break;
                }
            }

            builder
                .clang_arg(format!("-I{out_dir}"))
                .header_contents("lib.h", r#"
                #include <nix_api_expr.h>
                #include <nix_api_value.h>
                "#)
        }
    );
}
