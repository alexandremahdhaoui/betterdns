use crate::dns_manifest_parser::manifest::ManifestBuilder;

const MANIFEST_PATH: &str = "./dns_manifest";

#[get("/")]
pub(crate) fn get() -> String {
    ManifestBuilder::from_path(MANIFEST_PATH)
        .unwrap()
        .build()
        .to_string()
}