use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let hpx_path = PathBuf::from("/opt/hpx");
    let hpx_include = hpx_path.join("include");

    println!("cargo:rustc-env=AUTOCXX_INC={}", hpx_include.display());
    println!("cargo:rustc-link-search={}", hpx_path.join("lib").display());

    // tcmalloc search paths
    println!("cargo:rustc-link-search=/home/pingu/opt/gperftools/lib");

    // Link against HPX libraries
    println!("cargo:rustc-link-lib=hpx");
    println!("cargo:rustc-link-lib=hpx_core");
    println!("cargo:rustc-link-lib=hpx_init");
    println!("cargo:rustc-link-lib=hpx_wrap");

    // Link against other libraries required by hpx
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=tcmalloc_minimal");
    println!("cargo:rustc-link-lib=hwloc");
    println!("cargo:rustc-link-lib=pthread");

    let src_path = PathBuf::from("src");
    let root_path = PathBuf::from(".");

    let include_arg = format!("-I{}", hpx_include.display());

    let mut b = autocxx_build::Builder::new("src/bindings.rs", &[&src_path, &root_path])
        .extra_clang_args(&[
            &include_arg,
            "-std=c++17",
            "-fPIC",
            "-DHPX_APPLICATION_EXPORTS",
            "-pthread",
        ])
        .build()?;

    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-fPIC")
        .flag_if_supported("-DHPX_APPLICATION_EXPORTS")
        .flag_if_supported("-pthread")
        .flag_if_supported("-Wl,-wrap=main")
        .include(hpx_include)
        .compile("cpp-autocxx");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/bindings.rs");

    Ok(())
}
