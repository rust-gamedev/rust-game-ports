use std::{env, path::PathBuf};

const RESOURCES_RELATIVE_PATH: &str = "resources";

pub fn resource_path(resource_path: &str) -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    PathBuf::from(manifest_dir)
        .join(RESOURCES_RELATIVE_PATH)
        .join(resource_path)
}
