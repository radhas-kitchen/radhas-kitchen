extern crate tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/auth.proto")?;
    tonic_build::compile_protos("proto/jobs.proto")?;

    tonic_build::configure()
        .include_file("proto.rs")
        .compile_well_known_types(true)
        .compile(&["proto/auth.proto", "proto/jobs.proto"], &["proto"])?;

    Ok(())
}
