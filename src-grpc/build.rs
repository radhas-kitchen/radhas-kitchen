use std::{env, path::PathBuf};

extern crate tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .file_descriptor_set_path(PathBuf::from(env::var("OUT_DIR")?).join("proto_descriptor.bin"))
        .include_file("proto.rs")
        .compile_well_known_types(true)
        .compile(
            &["../proto/auth.proto", "../proto/jobs.proto"],
            &["../proto"],
        )?;

    Ok(())
}
