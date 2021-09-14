fn main() {
    #[cfg(target_os = "windows")]
    windows::link_vlc();
}

#[cfg(target_os = "windows")]
mod windows {
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    compile_error!("Only x86 and x86_64 are supported at the moment. Adding support for other architectures should be trivial.");

    use std::env;
    use std::ffi::OsString;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use vswhom::VsFindResult;

    pub fn link_vlc() {
        let vlc_path = vlc_path();

        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

        let vs = VsFindResult::search().expect("Could not locate Visual Studio");
        let vs_exe_path = PathBuf::from(
            vs.vs_exe_path
                .expect("Could not retrieve executable path for Visual Studio"),
        );

        generate_lib_from_dll(&out_dir, &vs_exe_path, &vlc_path);
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        // NOTE: Without this directive, linking fails with:
        //       ```
        //       error LNK2019: unresolved external symbol vsnprintf referenced in function _{MangledSymbolName}
        //          msvcrt.lib(vsnprintf.obj) : error LNK2001: unresolved external symbol vsnprintf
        //          msvcrt.lib(vsnprintf.obj) : error LNK2001: unresolved external symbol _vsnprintf
        //       ```
        //       https://stackoverflow.com/a/34230122
        println!("cargo:rustc-link-lib=dylib=legacy_stdio_definitions");
    }

    fn generate_lib_from_dll(out_dir: &Path, vs_exe_path: &Path, vlc_path: &Path) {
        // https://wiki.videolan.org/GenerateLibFromDll/

        let vs_dumpbin = vs_exe_path.join("dumpbin.exe");
        let vs_lib = vs_exe_path.join("lib.exe");
        let vlc_def_path = out_dir.join("libvlc.def");
        let vlc_import_lib = out_dir.join("vlc.lib");

        let libvlc = vlc_path.join("libvlc.dll");
        let exports = Command::new(vs_dumpbin)
            .current_dir(out_dir)
            .arg("/EXPORTS")
            .arg(libvlc.display().to_string().trim_end_matches(r"\"))
            .output()
            .unwrap();
        let exports = String::from_utf8(exports.stdout).unwrap();
        println!("{}", exports);

        let mut vlc_def = String::from("EXPORTS\n");
        for line in exports.lines() {
            if let Some(line) = line.get(26..) {
                if line.starts_with("libvlc_") {
                    vlc_def.push_str(line);
                    vlc_def.push_str("\r\n");
                }
            }
        }
        fs::write(&vlc_def_path, vlc_def.into_bytes()).unwrap();

        // FIXME: Handle paths with spaces in them.
        Command::new(vs_lib)
            .current_dir(out_dir)
            .arg("/NOLOGO")
            .args(&[
                format!(
                    r#"/DEF:{}"#,
                    vlc_def_path.display().to_string().trim_end_matches(r"\")
                ),
                format!(
                    r#"/OUT:{}"#,
                    vlc_import_lib.display().to_string().trim_end_matches(r"\")
                ),
                format!(
                    "/MACHINE:{}",
                    match target_arch().as_str() {
                        "x86" => "x86",
                        "x86_64" => "x64",
                        _ => unreachable!(),
                    }
                ),
            ])
            .spawn()
            .unwrap();
    }

    fn vlc_path() -> PathBuf {
        #[allow(unused_assignments)]
        let arch_path: Option<OsString> = match target_arch().as_str() {
            "x86" => env::var_os("VLC_LIB_DIR_X86"),
            "x86_64" => env::var_os("VLC_LIB_DIR_X86_64"),
            _ => unreachable!(),
        };

        arch_path
            .or_else(|| env::var_os("VLC_LIB_DIR"))
            .map(PathBuf::from)
            .expect("VLC_LIB_DIR not set")
    }

    fn target_arch() -> String {
        env::var("CARGO_CFG_TARGET_ARCH").unwrap()
    }
}
