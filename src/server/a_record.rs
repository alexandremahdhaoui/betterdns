use std::fs;
use crate::dns_manifest_parser::record::{Record, RecordData};
use rocket::serde::{json::Json};
use crate::dns_manifest_parser::record;
use crate::dns_manifest_parser::manifest::ManifestBuilder;

const MANIFEST_PATH: &str = "./dns_manifest";

#[get("/")]
pub(crate) fn get_a() -> &'static str {
    "/a"
}

#[get("/<name>")]
pub(crate) fn get_a_by_name(name: &str) -> String {
    format!("/a/{}", name)
}

// curl -XPOST 127.0.0.1:8000/a --data '{"name": "yolo.com.", "class": "IN", "record_type": "A", "value": "127.0.0.1"}'
#[post("/", data = "<record>")]
pub(crate) fn create_a(record: Json<Record>) -> String {
    let builder= &mut ManifestBuilder::from_path(MANIFEST_PATH).unwrap();
    let record_data = RecordData::A(Record::from_json(record));
    let manifest = builder
        .add_record(record_data)
        .increment()
        .build()
        .to_string();
    write_manifest(&manifest).expect("an error occurred while writing manifest");
    manifest
}

// curl -XPUT 127.0.0.1:8000/a/yolo.com. --data '{"name": "yolo.com.", "class": "IN", "record_type": "A", "value": "10.0.0.1"}'
#[put("/<name>", data = "<record>")]
pub(crate) fn update_a(name: &str, record: Json<Record>) -> String {
    let builder= &mut ManifestBuilder::from_path(MANIFEST_PATH).unwrap();
    let record_data = RecordData::A(Record::from_json(record));
    let manifest = builder
        .update_record(name, record_data)
        .increment()
        .build()
        .to_string();
    write_manifest(&manifest).expect("an error occurred while writing manifest");
    manifest
}

// curl -XDELETE 127.0.0.1:8000/a/yolo.com.
#[delete("/<name>")]
pub(crate) fn delete_a(name: &str) -> Result<String, String> {
    let builder= &mut ManifestBuilder::from_path(MANIFEST_PATH).unwrap();
    let manifest = builder
        .delete_record(name, record::A)
        .increment()
        .build()
        .to_string();
    write_manifest(&manifest).expect("an error occurred while writing manifest");
    Ok(manifest)
}

// --------------------------------------------- utils ---------------------------------------------

pub(crate) fn write_manifest(content: &str) -> std::io::Result<()> {
    fs::write(MANIFEST_PATH, content)
}