use std::collections::HashMap;

use crate::common::buffer::ByteBuf;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ExtendedHandshake {
    // #[serde(bound(deserialize = "ByteBuf: From<&'de [u8]>"))]
    // pub m: HashMap<Vec<u8>, u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub v: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub yourip: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub ipv6: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub ipv4: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reqq: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_ago: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_only: Option<u32>,
}