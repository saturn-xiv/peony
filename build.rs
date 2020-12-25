extern crate conan;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use conan::{BuildPolicy, InstallCommandBuilder};

fn shell(cmd: &mut Command) -> String {
    String::from_utf8(cmd.output().unwrap().stdout)
        .unwrap()
        .trim()
        .to_string()
}

fn main() {
    {
        let out_dir = env::var("OUT_DIR").unwrap();
        let git_version = shell(
            &mut Command::new("git")
                .arg("describe")
                .arg("--tags")
                .arg("--always")
                .arg("--first-parent")
                .arg("--dirty"),
        );
        let build_time = shell(&mut Command::new("date").arg("-u").arg("-R"));

        let dest_path = Path::new(&out_dir).join("env.rs");
        let mut fd = File::create(&dest_path).unwrap();

        writeln!(fd, r#"pub const VERSION: &str = "{}";"#, git_version).unwrap();
        writeln!(fd, r#"pub const BUILD_TIME: &str = "{}";"#, build_time).unwrap();
    }

    {
        println!("cargo:rustc-env=gRPC_PROTOBUF_PROVIDER=package");
        println!("cargo:rustc-env=gRPC_SSL_PROVIDER=package");
        println!("cargo:rustc-env=gRPC_ZLIB_PROVIDER=package");

        let root = Path::new("conan");
        let profile = root
            .join(format!(
                "{}-{}-{}",
                shell(&mut Command::new("lsb_release").arg("-cs")),
                env::var("CARGO_CFG_TARGET_OS").unwrap(),
                env::var("CARGO_CFG_TARGET_ARCH").unwrap()
            ))
            .display()
            .to_string();
        println!("use conan profile: {}", profile);
        let command = InstallCommandBuilder::new()
            .with_profile(&profile)
            .build_policy(BuildPolicy::Missing)
            .recipe_path(&root.join("conanfile.txt"))
            .build();
        println!("using conan build info");
        let build_info = command.generate().unwrap();
        build_info.cargo_emit();
    }
}
