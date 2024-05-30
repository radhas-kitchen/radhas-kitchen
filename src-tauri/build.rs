extern crate tauri_build;
extern crate tonic_build;

use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri_build::build();

    tonic_build::configure()
        .file_descriptor_set_path(PathBuf::from(env::var("OUT_DIR")?).join("proto_descriptor.bin"))
        .include_file("proto.rs")
        .compile_well_known_types(true)
        .type_attribute(
            "rkapi.auth.LoginRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.auth.LoginResponse",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.auth.CreateUserRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.auth.CreateUserRequest.kind",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.auth.DataUserProvider",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.auth.DataUserConsumer",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.auth.Authorization",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.jobs.Job",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.jobs.JobStatus",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.jobs.JobId",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "rkapi.jobs.JobUpdateRequest",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(
            &["../proto/auth.proto", "../proto/jobs.proto"],
            &["../proto"],
        )?;

    Ok(())
}
