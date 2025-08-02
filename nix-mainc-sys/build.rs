fn main() {
    nix_sys_common_build::configure_library(
        "nix-main-c",
        "NIXMAINC_STATIC",
        cfg!(feature = "static"),
        cfg!(feature = "shared"),
        |_, a| {
            a.header_contents("lib.h", "#include <nix_api_main.h>")
        }
    );
}
