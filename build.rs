fn main() {
    #[cfg(windows)]
    {
        let sdk_paths = [
            ("INTELOCLSDKROOT", "x64", "x86"),
            ("AMDAPPSDKROOT", "x86_64", "x86"),
            ("CUDA_PATH", "x64", "Win32"),
            ("OPENCL_ROOT", "x64", "x86"),
        ];

        sdk_paths.iter().find_map(|&(env_var, x64_path, x86_path)| {
            std::env::var(env_var).ok().map(|sdk_base_path| {
                let lib_dir = "lib";
                let arch_path = if cfg!(target_arch = "x86_64") {
                    x64_path
                } else {
                    x86_path
                };

                let path = std::path::PathBuf::from(sdk_base_path)
                    .join(lib_dir)
                    .join(arch_path);

                println!("cargo:rustc-link-search=native={}", path.display());
            })
        });
    }
}
