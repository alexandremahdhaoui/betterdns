use std::fmt;
use std::fmt::{Display, Formatter};
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

// RecordType
pub(crate) const A: &str = "A";
pub(crate) const NS: &str = "NS";
pub(crate) const SOA: &str = "SOA";

// SOA default values
const DEFAULT_REFRESH: u32 = 7200;
const DEFAULT_RETRY: u32 = 3600;
const DEFAULT_EXPIRE: u32 = 604800;
const DEFAULT_MINIMUM: u32 = 300;

// ------------------------------------------ RecordData -------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) enum RecordData {
    A(Record),
    NS(Record),
    SOA(SOARecord),
}

impl RecordData {
    pub(crate) fn get_name(&self) -> String {
        match &self {
            Self::A(r) => r.name.to_string(),
            Self::NS(r) => r.name.to_string(),
            Self::SOA(r) => r.name.to_string(),
        }
    }

    pub(crate) fn get_type(&self) -> &str {
        match &self {
            Self::A(_) => A,
            Self::NS(_) => NS,
            Self::SOA(_) => SOA
        }
    }

    pub(crate) fn from_str(s: &str) -> Result<Self, String> {
        let v = split_str(s);
        let length = v.len();

        if length == 1 && v[0].is_empty() {
            return Err("skipping empty line".to_string())
        }

        if length != 4 && length != 10 {
            return Err(format!(
                "length of DNS record should be equal to 4 or 10, received: {}",
                length
            ));
        }
        let record_type = v[2].as_str();
        match record_type {
            A => Ok(RecordData::A(Record {
                name: v[0].to_string(),
                class: v[1].to_string(),
                record_type: record_type.to_string(),
                value: v[3].to_string(),
            })),
            NS => Ok(RecordData::NS(Record {
                name: v[0].to_string(),
                class: v[1].to_string(),
                record_type: record_type.to_string(),
                value: v[3].to_string(),
            })),
            SOA => {
                if length != 10 {
                    return Err(format!(
                        "length of SOA record should be equal to 10, received: {}",
                        length
                    ));
                }

                Ok(RecordData::SOA(SOARecord {
                    name: v[0].to_string(),
                    class: v[1].to_string(),
                    mname: v[3].to_string(),
                    rname: v[4].to_string(),
                    serial: v[5].parse().unwrap(), // serial is mandatory.
                    refresh: v[6].parse().unwrap_or(DEFAULT_REFRESH),
                    retry: v[7].parse().unwrap_or(DEFAULT_RETRY),
                    expire: v[8].parse().unwrap_or(DEFAULT_EXPIRE),
                    minimum: v[9].parse().unwrap_or(DEFAULT_MINIMUM),
                }))
            }
            _ => Err(format!(
                "record_type should be equal to `A`, `NS` or `SOA`, received: `{}`",
                record_type
            )),
        }
    }
}

impl Display for RecordData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(r) => write!(f, "{}", r),
            Self::NS(r) => write!(f, "{}", r),
            Self::SOA(r) => write!(f, "{}", r),
        }
    }
}

fn split_str(s: &str) -> Vec<String> {
    let s = clean_str(s);
    s.split(" ").map(|s| s.to_string()).collect()
}

fn clean_str(s: &str) -> String {
    let regex = Regex::new(r"(;.*)+").unwrap();
    let s = regex.replace_all(s, "").to_string();

    let regex = Regex::new(r" {2,}").unwrap();
    regex.replace_all(&s, " ").to_string()
}

// ------------------------------------------ SOARecord --------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct SOARecord {
    name: String,
    class: String, // "IN"
    mname: String, // Primary master name server for this zone
    rname: String, // Email address of the administrator responsible for this zone.
    // Serial number for this zone.
    // If a secondary name server slaved to this one observes an increase in this number,
    // the slave will assume that the zone has been updated and initiate a zone transfer.
    serial: u32,
    // Number of seconds after which secondary name servers should query the master for the SOA
    // record, to detect zone changes. Recommendation for small and stable zones:[4] 86400 seconds (24 hours).
    refresh: u32,
    // Number of seconds after which secondary name servers should retry to request the serial
    // number from the master if the master does not respond. It must be less than Refresh.
    // Recommendation for small and stable zones: 7200 seconds (2 hours).
    retry: u32,
    // Number of seconds after which secondary name servers should stop answering request for this
    // zone if the master does not respond.
    // This value must be bigger than the sum of Refresh and Retry.
    // Recommendation for small and stable zones: 3600000 seconds (1000 hours).
    expire: u32,
    // Used in calculating the time to live for purposes of negative caching.
    // Authoritative name servers take the smaller of the SOA TTL and the SOA MINIMUM to send as the
    // SOA TTL in negative responses. Resolvers use the resulting SOA TTL to understand for how long
    // they are allowed to cache a negative response.
    // Recommendation for small and stable zones: 172800 seconds (2 days).
    // Originally this field had the meaning of a minimum TTL value for resource records in this
    // zone; it was changed to its current meaning by RFC 2308.
    minimum: u32,
}

impl SOARecord {
    pub(crate) fn new() -> Self {
        Self{
            name: "".to_string(),
            class: "".to_string(),
            mname: "".to_string(),
            rname: "".to_string(),
            serial: 0,
            refresh: 0,
            retry: 0,
            expire: 0,
            minimum: 0,
        }
    }

    pub(crate) fn increment(&mut self) {
        self.serial += 1
    }
}

impl Display for SOARecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {} {} {} {} {} {} {}",
            self.name,
            self.class,
            SOA,
            self.mname,
            self.rname,
            self.serial,
            self.refresh,
            self.retry,
            self.expire,
            self.minimum,
        )
    }
}

// -------------------------------------------- Record ---------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Record {
    name: String,
    class: String,
    record_type: String,
    value: String,
}

impl Record {
    pub(crate) fn from_json(j: Json<Self>) -> Self {
        Self{
            name: j.name.to_string(),
            class: j.class.to_string(),
            record_type: j.record_type.to_string(),
            value: j.value.to_string(),
        }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}",
            self.name,
            self.class,
            self.record_type,
            self.value
        )
    }
}

// ----------------------------------------- OriginRecord ------------------------------------------

const ORIGIN: &str = "$ORIGIN ";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct OriginRecord {
    origin: String
}

impl OriginRecord {
    pub(crate) fn from_str(s: &str) -> Self { Self{ origin: s.to_string() } }
    pub(crate) fn new() -> Self {Self{ origin: "example.com".to_string() }}
}

impl Display for OriginRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", ORIGIN, self.origin)
    }
}

// ------------------------------------------- TTLRecord -------------------------------------------

const TTL: &str = "$TTL ";
const DEFAULT_TTL: u32 = 3600;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct TTLRecord {
    ttl: u32
}

impl TTLRecord {
    pub(crate) fn from_str(s: &str) -> Self {
        Self{
            ttl: parse_ttl(s),
        }
    }
    pub(crate) fn new() -> Self {
        Self{ ttl: 0 }
    }
}

impl Display for TTLRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", TTL, self.ttl)
    }
}

fn parse_ttl(s: &str) -> u32 {
    if s.to_string().contains(TTL) {
        let s = s.replace(TTL, "");
        let s = s.replace(" ", "");
        return match s.parse::<u32>() {
            Ok(u) => u,
            Err(_) => {
                println!("ttl should be a parseable u32 integer, received: {}", s);
                DEFAULT_TTL
            }
        }
    }
    println!("zonefile should contain TTL");
    DEFAULT_TTL
}