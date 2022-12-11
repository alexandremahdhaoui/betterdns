use std::fmt::{Display, Formatter};
use std::fs;
use crate::dns_manifest_parser::record::{OriginRecord, RecordData, SOARecord, TTLRecord};
use rocket::serde::{Deserialize, Serialize};

// --------------------------------------- ManifestBuilder -----------------------------------------

pub(crate) struct ManifestBuilder {
    manifest: Manifest
}

impl ManifestBuilder {
    pub(crate) fn build(&mut self) -> Manifest{
        std::mem::replace(&mut self.manifest, Manifest::new())
    }

    pub(crate) fn _from(manifest: Manifest) -> Self{
        Self{ manifest }
    }

    pub(crate) fn from_path(filepath: &str) -> Result<Self, String>{
        match Manifest::from_path(filepath) {
            Ok(manifest) => Ok(Self{ manifest}),
            Err(e) => Err(e)
        }

    }

    pub(crate) fn add_record(&mut self, record: RecordData) -> &mut Self {
        self.manifest.records.push(record);
        self
    }

    pub(crate) fn update_record(&mut self, name: &str, rec: RecordData) -> &mut Self {
        if let Some(i) = self.manifest.records
            .iter()
            .position(|r| {
                name == r.get_name() && r.get_type() == rec.get_type()
            }) {
            let _ = std::mem::replace(&mut self.manifest.records[i], rec);
        }
        self
    }

    pub(crate) fn delete_record(&mut self, name: &str, record_type: &str) -> &mut Self {
        if let Some(i) = self.manifest.records
            .iter()
            .position(
                |r| r.get_name() == name && r.get_type() == record_type
            ) {
            self.manifest.records.remove(i);
        }
        self
    }

    /// increments the version of the SOA record.
    pub(crate) fn increment(&mut self) -> &mut Self {
        self.manifest.soa.increment();
        self
    }
}

// ------------------------------------------- Manifest --------------------------------------------


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Manifest {
    origin: OriginRecord,
    ttl: TTLRecord,
    soa: SOARecord,
    // list of all records
    records: Vec<RecordData>
}

impl Manifest {
    fn from_str(s: &str) -> Result<Self, String> {
        let lines: Vec<&str> = s.split("\n").collect();

        // Parses Origin
        let origin = OriginRecord::from_str(lines[0]);
        // Parses ttl
        let ttl = TTLRecord::from_str(lines[1]);

        // Parses soa
        let soa: SOARecord;
        match RecordData::from_str(lines[2]) {
            Ok(rec) => {
                match rec {
                    RecordData::SOA(r) => soa = r,
                    _ => return Err("expected SOA".to_string())
                }
            }
            Err(_) => return Err("expected SOA".to_string())
        }

        // Parses records from other lines
        let mut records: Vec<RecordData> = Vec::new();
        for line in lines[3..].iter() {
            match RecordData::from_str(line) {
                Ok(rec) => records.push(rec),
                Err(e) => println!("{}", e)
            }
        }

        Ok(Self{
            origin,
            ttl,
            soa,
            records,
        })
    }

    pub(crate) fn from_path(filepath: &str) -> Result<Self, String> {
        let s = fs::read_to_string(filepath).unwrap();
        Self::from_str(&s)
    }

    fn _from_bytes(b: Vec<u8>) -> Result<Manifest, String> {
        let result = String::from_utf8(b);
        match result {
            Ok(s) => Self::from_str(&s),
            Err(e) => Err(e.to_string())
        }
    }

    fn _to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    fn new() -> Self {
        Self{
            origin: OriginRecord::new(),
            ttl: TTLRecord::new(),
            soa: SOARecord::new(),
            records: vec![],
        }
    }
}

impl Display for Manifest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = vec!(
            self.ttl.to_string(),
            self.origin.to_string(),
            self.soa.to_string(),
        );

        let records: Vec<String> = self.records
            .iter()
            .map(|r| r.to_string())
            .collect();
        v.extend(records);

        let s = v.join("\n");

        write!(f, "{}", s)
    }
}